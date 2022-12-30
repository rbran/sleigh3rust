pub type AddrType = u16;
macro_rules! impl_read_to_type {
    ($ unsigned_type : ty , $ signed_type : ty , $ len : literal , $ read_unsigned : ident , $ read_signed : ident , $ write_unsigned : ident , $ write_signed : ident) => {
        const fn $read_unsigned<const BIG_ENDIAN: bool>(
            data: [u8; $len],
            start_bit: usize,
            len_bits: usize,
        ) -> $unsigned_type {
            const TYPE_BITS: usize = <$unsigned_type>::BITS as usize;
            assert!(TYPE_BITS / 8 == $len);
            assert!(len_bits > 0);
            assert!(len_bits + start_bit <= TYPE_BITS);
            let mut data = if BIG_ENDIAN {
                <$unsigned_type>::from_be_bytes(data)
            } else {
                <$unsigned_type>::from_le_bytes(data)
            };
            let value_mask = <$unsigned_type>::MAX >> (TYPE_BITS - len_bits);
            data = data >> start_bit;
            data = data & value_mask;
            data
        }
        const fn $read_signed<const BIG_ENDIAN: bool>(
            data: [u8; $len],
            start_bit: usize,
            len_bits: usize,
        ) -> $signed_type {
            const TYPE_BITS: usize = <$signed_type>::BITS as usize;
            assert!(len_bits > 1);
            assert!(TYPE_BITS / 8 == $len);
            let data = $read_unsigned::<BIG_ENDIAN>(data, start_bit, len_bits);
            let value_mask =
                <$signed_type>::MAX as $unsigned_type >> (TYPE_BITS - len_bits);
            let sign_mask = !value_mask;
            let value_part = data & value_mask;
            let sign_part = data & sign_mask;
            if sign_part != 0 {
                sign_mask as $signed_type | value_part as $signed_type
            } else {
                data as $signed_type
            }
        }
        const fn $write_unsigned<const BIG_ENDIAN: bool>(
            value: $unsigned_type,
            mem: $unsigned_type,
            start_bit: usize,
            len_bits: usize,
        ) -> [u8; $len] {
            const TYPE_BITS: usize = <$unsigned_type>::BITS as usize;
            assert!(len_bits > 0);
            assert!(len_bits + start_bit <= TYPE_BITS);
            let value_max = <$unsigned_type>::MAX >> (TYPE_BITS - len_bits);
            let mask = value_max << start_bit;
            let mut value = value;
            value <<= start_bit;
            value = (mem & !mask) | value;
            if BIG_ENDIAN {
                value.to_be_bytes()
            } else {
                value.to_le_bytes()
            }
        }
        const fn $write_signed<const BIG_ENDIAN: bool>(
            value: $signed_type,
            mem: $signed_type,
            start_bit: usize,
            len_bits: usize,
        ) -> [u8; $len] {
            const TYPE_BITS: usize = <$unsigned_type>::BITS as usize;
            assert!(len_bits > 0);
            assert!(len_bits + start_bit <= TYPE_BITS);
            let value_max = <$signed_type>::MAX >> (TYPE_BITS - len_bits);
            let value_min = <$signed_type>::MIN >> (TYPE_BITS - len_bits);
            let mask = <$unsigned_type>::MAX >> (TYPE_BITS - len_bits);
            let value = value as $unsigned_type & mask;
            let mem = mem as $unsigned_type;
            $write_unsigned::<BIG_ENDIAN>(value, mem, start_bit, len_bits)
        }
    };
}
impl_read_to_type!(u8, i8, 1, read_u8, read_i8, write_u8, write_i8);
impl_read_to_type!(u16, i16, 2, read_u16, read_i16, write_u16, write_i16);
impl_read_to_type!(u32, i32, 4, read_u32, read_i32, write_u32, write_i32);
impl_read_to_type!(u64, i64, 8, read_u64, read_i64, write_u64, write_i64);
impl_read_to_type!(
    u128, i128, 16, read_u128, read_i128, write_u128, write_i128
);
pub trait GlobalSetTrait {}
pub trait MemoryRead {
    type AddressType;
    fn read(&self, addr: Self::AddressType, buf: &mut [u8]);
}
pub trait MemoryWrite {
    type AddressType;
    fn write(&mut self, addr: Self::AddressType, buf: &[u8]);
}
pub trait ContextTrait {}
#[derive(Debug, Clone, Copy, Default)]
pub struct SpacesStruct {}
impl ContextTrait for SpacesStruct {}
fn meaning_number<T>(hex: bool, num: T) -> DisplayElement
where
    i64: TryFrom<T>,
    <i64 as TryFrom<T>>::Error: core::fmt::Debug,
{
    DisplayElement::Number(hex, i64::try_from(num).unwrap())
}
fn meaning_0_display<T>(num: T) -> DisplayElement
where
    u16: TryFrom<T>,
    <u16 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_0_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_0_value<T>(num: T) -> Register
where
    u16: TryFrom<T>,
    <u16 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u16::try_from(num).unwrap() {
        0 => Register::ZRlo,
        1 => Register::ZRhi,
        2 => Register::AD_resultlo,
        3 => Register::AD_resulthi,
        4 => Register::HSI_timelo,
        5 => Register::HSI_timehi,
        6 => Register::HSI_status,
        7 => Register::SBUF,
        8 => Register::INT_MASK,
        9 => Register::INT_PEND,
        10 => Register::TIMER1lo,
        11 => Register::TIMER1hi,
        12 => Register::TIMER2lo,
        13 => Register::TIMER2hi,
        14 => Register::PORT0,
        15 => Register::PORT1,
        16 => Register::PORT2,
        17 => Register::SP_STAT,
        18 => Register::INT_PEND1,
        19 => Register::INT_MASK1,
        20 => Register::WSR,
        21 => Register::IOS0,
        22 => Register::IOS1,
        23 => Register::IOS2,
        24 => Register::SPlo,
        25 => Register::SPhi,
        26 => Register::R1A,
        27 => Register::R1B,
        28 => Register::R1C,
        29 => Register::R1D,
        30 => Register::R1E,
        31 => Register::R1F,
        32 => Register::R20,
        33 => Register::R21,
        34 => Register::R22,
        35 => Register::R23,
        36 => Register::R24,
        37 => Register::R25,
        38 => Register::R26,
        39 => Register::R27,
        40 => Register::R28,
        41 => Register::R29,
        42 => Register::R2A,
        43 => Register::R2B,
        44 => Register::R2C,
        45 => Register::R2D,
        46 => Register::R2E,
        47 => Register::R2F,
        48 => Register::R30,
        49 => Register::R31,
        50 => Register::R32,
        51 => Register::R33,
        52 => Register::R34,
        53 => Register::R35,
        54 => Register::R36,
        55 => Register::R37,
        56 => Register::R38,
        57 => Register::R39,
        58 => Register::R3A,
        59 => Register::R3B,
        60 => Register::R3C,
        61 => Register::R3D,
        62 => Register::R3E,
        63 => Register::R3F,
        64 => Register::R40,
        65 => Register::R41,
        66 => Register::R42,
        67 => Register::R43,
        68 => Register::R44,
        69 => Register::R45,
        70 => Register::R46,
        71 => Register::R47,
        72 => Register::R48,
        73 => Register::R49,
        74 => Register::R4A,
        75 => Register::R4B,
        76 => Register::R4C,
        77 => Register::R4D,
        78 => Register::R4E,
        79 => Register::R4F,
        80 => Register::R50,
        81 => Register::R51,
        82 => Register::R52,
        83 => Register::R53,
        84 => Register::R54,
        85 => Register::R55,
        86 => Register::R56,
        87 => Register::R57,
        88 => Register::R58,
        89 => Register::R59,
        90 => Register::R5A,
        91 => Register::R5B,
        92 => Register::R5C,
        93 => Register::R5D,
        94 => Register::R5E,
        95 => Register::R5F,
        96 => Register::R60,
        97 => Register::R61,
        98 => Register::R62,
        99 => Register::R63,
        100 => Register::R64,
        101 => Register::R65,
        102 => Register::R66,
        103 => Register::R67,
        104 => Register::R68,
        105 => Register::R69,
        106 => Register::R6A,
        107 => Register::R6B,
        108 => Register::R6C,
        109 => Register::R6D,
        110 => Register::R6E,
        111 => Register::R6F,
        112 => Register::R70,
        113 => Register::R71,
        114 => Register::R72,
        115 => Register::R73,
        116 => Register::R74,
        117 => Register::R75,
        118 => Register::R76,
        119 => Register::R77,
        120 => Register::R78,
        121 => Register::R79,
        122 => Register::R7A,
        123 => Register::R7B,
        124 => Register::R7C,
        125 => Register::R7D,
        126 => Register::R7E,
        127 => Register::R7F,
        128 => Register::R80,
        129 => Register::R81,
        130 => Register::R82,
        131 => Register::R83,
        132 => Register::R84,
        133 => Register::R85,
        134 => Register::R86,
        135 => Register::R87,
        136 => Register::R88,
        137 => Register::R89,
        138 => Register::R8A,
        139 => Register::R8B,
        140 => Register::R8C,
        141 => Register::R8D,
        142 => Register::R8E,
        143 => Register::R8F,
        144 => Register::R90,
        145 => Register::R91,
        146 => Register::R92,
        147 => Register::R93,
        148 => Register::R94,
        149 => Register::R95,
        150 => Register::R96,
        151 => Register::R97,
        152 => Register::R98,
        153 => Register::R99,
        154 => Register::R9A,
        155 => Register::R9B,
        156 => Register::R9C,
        157 => Register::R9D,
        158 => Register::R9E,
        159 => Register::R9F,
        160 => Register::RA0,
        161 => Register::RA1,
        162 => Register::RA2,
        163 => Register::RA3,
        164 => Register::RA4,
        165 => Register::RA5,
        166 => Register::RA6,
        167 => Register::RA7,
        168 => Register::RA8,
        169 => Register::RA9,
        170 => Register::RAA,
        171 => Register::RAB,
        172 => Register::RAC,
        173 => Register::RAD,
        174 => Register::RAE,
        175 => Register::RAF,
        176 => Register::RB0,
        177 => Register::RB1,
        178 => Register::RB2,
        179 => Register::RB3,
        180 => Register::RB4,
        181 => Register::RB5,
        182 => Register::RB6,
        183 => Register::RB7,
        184 => Register::RB8,
        185 => Register::RB9,
        186 => Register::RBA,
        187 => Register::RBB,
        188 => Register::RBC,
        189 => Register::RBD,
        190 => Register::RBE,
        191 => Register::RBF,
        192 => Register::RC0,
        193 => Register::RC1,
        194 => Register::RC2,
        195 => Register::RC3,
        196 => Register::RC4,
        197 => Register::RC5,
        198 => Register::RC6,
        199 => Register::RC7,
        200 => Register::RC8,
        201 => Register::RC9,
        202 => Register::RCA,
        203 => Register::RCB,
        204 => Register::RCC,
        205 => Register::RCD,
        206 => Register::RCE,
        207 => Register::RCF,
        208 => Register::RD0,
        209 => Register::RD1,
        210 => Register::RD2,
        211 => Register::RD3,
        212 => Register::RD4,
        213 => Register::RD5,
        214 => Register::RD6,
        215 => Register::RD7,
        216 => Register::RD8,
        217 => Register::RD9,
        218 => Register::RDA,
        219 => Register::RDB,
        220 => Register::RDC,
        221 => Register::RDD,
        222 => Register::RDE,
        223 => Register::RDF,
        224 => Register::RE0,
        225 => Register::RE1,
        226 => Register::RE2,
        227 => Register::RE3,
        228 => Register::RE4,
        229 => Register::RE5,
        230 => Register::RE6,
        231 => Register::RE7,
        232 => Register::RE8,
        233 => Register::RE9,
        234 => Register::REA,
        235 => Register::REB,
        236 => Register::REC,
        237 => Register::RED,
        238 => Register::REE,
        239 => Register::REF,
        240 => Register::RF0,
        241 => Register::RF1,
        242 => Register::RF2,
        243 => Register::RF3,
        244 => Register::RF4,
        245 => Register::RF5,
        246 => Register::RF6,
        247 => Register::RF7,
        248 => Register::RF8,
        249 => Register::RF9,
        250 => Register::RFA,
        251 => Register::RFB,
        252 => Register::RFC,
        253 => Register::RFD,
        254 => Register::RFE,
        255 => Register::RFF,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_1_display<T>(num: T) -> DisplayElement
where
    u16: TryFrom<T>,
    <u16 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_1_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_1_value<T>(num: T) -> Register
where
    u16: TryFrom<T>,
    <u16 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u16::try_from(num).unwrap() {
        0 => Register::ZR,
        2 => Register::AD_result,
        4 => Register::HSI_time,
        6 => Register::HSI_SBUF,
        8 => Register::INTERRUPT,
        10 => Register::TIMER1,
        12 => Register::TIMER2,
        14 => Register::PORT01,
        16 => Register::PORT2_SPS,
        18 => Register::INT1,
        20 => Register::WSR_IOS0,
        22 => Register::IOS12,
        24 => Register::SP,
        26 => Register::RW1A,
        28 => Register::RW1C,
        30 => Register::RW1E,
        32 => Register::RW20,
        34 => Register::RW22,
        36 => Register::RW24,
        38 => Register::RW26,
        40 => Register::RW28,
        42 => Register::RW2A,
        44 => Register::RW2C,
        46 => Register::RW2E,
        48 => Register::RW30,
        50 => Register::RW32,
        52 => Register::RW34,
        54 => Register::RW36,
        56 => Register::RW38,
        58 => Register::RW3A,
        60 => Register::RW3C,
        62 => Register::RW3E,
        64 => Register::RW40,
        66 => Register::RW42,
        68 => Register::RW44,
        70 => Register::RW46,
        72 => Register::RW48,
        74 => Register::RW4A,
        76 => Register::RW4C,
        78 => Register::RW4E,
        80 => Register::RW50,
        82 => Register::RW52,
        84 => Register::RW54,
        86 => Register::RW56,
        88 => Register::RW58,
        90 => Register::RW5A,
        92 => Register::RW5C,
        94 => Register::RW5E,
        96 => Register::RW60,
        98 => Register::RW62,
        100 => Register::RW64,
        102 => Register::RW66,
        104 => Register::RW68,
        106 => Register::RW6A,
        108 => Register::RW6C,
        110 => Register::RW6E,
        112 => Register::RW70,
        114 => Register::RW72,
        116 => Register::RW74,
        118 => Register::RW76,
        120 => Register::RW78,
        122 => Register::RW7A,
        124 => Register::RW7C,
        126 => Register::RW7E,
        128 => Register::RW80,
        130 => Register::RW82,
        132 => Register::RW84,
        134 => Register::RW86,
        136 => Register::RW88,
        138 => Register::RW8A,
        140 => Register::RW8C,
        142 => Register::RW8E,
        144 => Register::RW90,
        146 => Register::RW92,
        148 => Register::RW94,
        150 => Register::RW96,
        152 => Register::RW98,
        154 => Register::RW9A,
        156 => Register::RW9C,
        158 => Register::RW9E,
        160 => Register::RWA0,
        162 => Register::RWA2,
        164 => Register::RWA4,
        166 => Register::RWA6,
        168 => Register::RWA8,
        170 => Register::RWAA,
        172 => Register::RWAC,
        174 => Register::RWAE,
        176 => Register::RWB0,
        178 => Register::RWB2,
        180 => Register::RWB4,
        182 => Register::RWB6,
        184 => Register::RWB8,
        186 => Register::RWBA,
        188 => Register::RWBC,
        190 => Register::RWBE,
        192 => Register::RWC0,
        194 => Register::RWC2,
        196 => Register::RWC4,
        198 => Register::RWC6,
        200 => Register::RWC8,
        202 => Register::RWCA,
        204 => Register::RWCC,
        206 => Register::RWCE,
        208 => Register::RWD0,
        210 => Register::RWD2,
        212 => Register::RWD4,
        214 => Register::RWD6,
        216 => Register::RWD8,
        218 => Register::RWDA,
        220 => Register::RWDC,
        222 => Register::RWDE,
        224 => Register::RWE0,
        226 => Register::RWE2,
        228 => Register::RWE4,
        230 => Register::RWE6,
        232 => Register::RWE8,
        234 => Register::RWEA,
        236 => Register::RWEC,
        238 => Register::RWEE,
        240 => Register::RWF0,
        242 => Register::RWF2,
        244 => Register::RWF4,
        246 => Register::RWF6,
        248 => Register::RWF8,
        250 => Register::RWFA,
        252 => Register::RWFC,
        254 => Register::RWFE,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_2_display<T>(num: T) -> DisplayElement
where
    u16: TryFrom<T>,
    <u16 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_2_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_2_value<T>(num: T) -> Register
where
    u16: TryFrom<T>,
    <u16 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u16::try_from(num).unwrap() {
        0 => Register::ZR_AD,
        4 => Register::HSI,
        8 => Register::INT_TIMER1,
        12 => Register::TIMER2_PORT01,
        16 => Register::PORT2_INT1,
        20 => Register::WSR_IOS012,
        24 => Register::SPR1A,
        28 => Register::RL1C,
        32 => Register::RL20,
        36 => Register::RL24,
        40 => Register::RL28,
        44 => Register::RL2C,
        48 => Register::RL30,
        52 => Register::RL34,
        56 => Register::RL38,
        60 => Register::RL3C,
        64 => Register::RL40,
        68 => Register::RL44,
        72 => Register::RL48,
        76 => Register::RL4C,
        80 => Register::RL50,
        84 => Register::RL54,
        88 => Register::RL58,
        92 => Register::RL5C,
        96 => Register::RL60,
        100 => Register::RL64,
        104 => Register::RL68,
        108 => Register::RL6C,
        112 => Register::RL70,
        116 => Register::RL74,
        120 => Register::RL78,
        124 => Register::RL7C,
        128 => Register::RL80,
        132 => Register::RL84,
        136 => Register::RL88,
        140 => Register::RL8C,
        144 => Register::RL90,
        148 => Register::RL94,
        152 => Register::RL98,
        156 => Register::RL9C,
        160 => Register::RLA0,
        164 => Register::RLA4,
        168 => Register::RLA8,
        172 => Register::RLAC,
        176 => Register::RLB0,
        180 => Register::RLB4,
        184 => Register::RLB8,
        188 => Register::RLBC,
        192 => Register::RLC0,
        196 => Register::RLC4,
        200 => Register::RLC8,
        204 => Register::RLCC,
        208 => Register::RLD0,
        212 => Register::RLD4,
        216 => Register::RLD8,
        220 => Register::RLDC,
        224 => Register::RLE0,
        228 => Register::RLE4,
        232 => Register::RLE8,
        236 => Register::RLEC,
        240 => Register::RLF0,
        244 => Register::RLF4,
        248 => Register::RLF8,
        252 => Register::RLFC,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_3_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_3_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_3_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::ZR,
        1 => Register::AD_result,
        2 => Register::HSI_time,
        3 => Register::HSI_SBUF,
        4 => Register::INTERRUPT,
        5 => Register::TIMER1,
        6 => Register::TIMER2,
        7 => Register::PORT01,
        8 => Register::PORT2_SPS,
        9 => Register::INT1,
        10 => Register::WSR_IOS0,
        11 => Register::IOS12,
        12 => Register::SP,
        13 => Register::RW1A,
        14 => Register::RW1C,
        15 => Register::RW1E,
        16 => Register::RW20,
        17 => Register::RW22,
        18 => Register::RW24,
        19 => Register::RW26,
        20 => Register::RW28,
        21 => Register::RW2A,
        22 => Register::RW2C,
        23 => Register::RW2E,
        24 => Register::RW30,
        25 => Register::RW32,
        26 => Register::RW34,
        27 => Register::RW36,
        28 => Register::RW38,
        29 => Register::RW3A,
        30 => Register::RW3C,
        31 => Register::RW3E,
        32 => Register::RW40,
        33 => Register::RW42,
        34 => Register::RW44,
        35 => Register::RW46,
        36 => Register::RW48,
        37 => Register::RW4A,
        38 => Register::RW4C,
        39 => Register::RW4E,
        40 => Register::RW50,
        41 => Register::RW52,
        42 => Register::RW54,
        43 => Register::RW56,
        44 => Register::RW58,
        45 => Register::RW5A,
        46 => Register::RW5C,
        47 => Register::RW5E,
        48 => Register::RW60,
        49 => Register::RW62,
        50 => Register::RW64,
        51 => Register::RW66,
        52 => Register::RW68,
        53 => Register::RW6A,
        54 => Register::RW6C,
        55 => Register::RW6E,
        56 => Register::RW70,
        57 => Register::RW72,
        58 => Register::RW74,
        59 => Register::RW76,
        60 => Register::RW78,
        61 => Register::RW7A,
        62 => Register::RW7C,
        63 => Register::RW7E,
        64 => Register::RW80,
        65 => Register::RW82,
        66 => Register::RW84,
        67 => Register::RW86,
        68 => Register::RW88,
        69 => Register::RW8A,
        70 => Register::RW8C,
        71 => Register::RW8E,
        72 => Register::RW90,
        73 => Register::RW92,
        74 => Register::RW94,
        75 => Register::RW96,
        76 => Register::RW98,
        77 => Register::RW9A,
        78 => Register::RW9C,
        79 => Register::RW9E,
        80 => Register::RWA0,
        81 => Register::RWA2,
        82 => Register::RWA4,
        83 => Register::RWA6,
        84 => Register::RWA8,
        85 => Register::RWAA,
        86 => Register::RWAC,
        87 => Register::RWAE,
        88 => Register::RWB0,
        89 => Register::RWB2,
        90 => Register::RWB4,
        91 => Register::RWB6,
        92 => Register::RWB8,
        93 => Register::RWBA,
        94 => Register::RWBC,
        95 => Register::RWBE,
        96 => Register::RWC0,
        97 => Register::RWC2,
        98 => Register::RWC4,
        99 => Register::RWC6,
        100 => Register::RWC8,
        101 => Register::RWCA,
        102 => Register::RWCC,
        103 => Register::RWCE,
        104 => Register::RWD0,
        105 => Register::RWD2,
        106 => Register::RWD4,
        107 => Register::RWD6,
        108 => Register::RWD8,
        109 => Register::RWDA,
        110 => Register::RWDC,
        111 => Register::RWDE,
        112 => Register::RWE0,
        113 => Register::RWE2,
        114 => Register::RWE4,
        115 => Register::RWE6,
        116 => Register::RWE8,
        117 => Register::RWEA,
        118 => Register::RWEC,
        119 => Register::RWEE,
        120 => Register::RWF0,
        121 => Register::RWF2,
        122 => Register::RWF4,
        123 => Register::RWF6,
        124 => Register::RWF8,
        125 => Register::RWFA,
        126 => Register::RWFC,
        127 => Register::RWFE,
        _ => unreachable!("Invalid Attach Value"),
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op8(u8);
impl TokenField_op8 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op6(u8);
impl TokenField_op6 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op5(u8);
impl TokenField_op5 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_cond(u8);
impl TokenField_cond {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op4(u8);
impl TokenField_op4 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_aa(u8);
impl TokenField_aa {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_bitno(u8);
impl TokenField_bitno {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_highb(u8);
impl TokenField_highb {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_imm8(i8);
impl TokenField_imm8 {
    fn execution(&self) -> i8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_simm8(i8);
impl TokenField_simm8 {
    fn execution(&self) -> i8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_baop(u8);
impl TokenField_baop {
    fn execution(&self) -> Register {
        meaning_0_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_0_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_breg8(u8);
impl TokenField_breg8 {
    fn execution(&self) -> Register {
        meaning_0_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_0_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_dbreg(u8);
impl TokenField_dbreg {
    fn execution(&self) -> Register {
        meaning_0_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_0_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_waop(u8);
impl TokenField_waop {
    fn execution(&self) -> Register {
        meaning_1_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_1_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_wreg8(u8);
impl TokenField_wreg8 {
    fn execution(&self) -> Register {
        meaning_1_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_1_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_dwreg(u8);
impl TokenField_dwreg {
    fn execution(&self) -> Register {
        meaning_1_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_1_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_lreg(u8);
impl TokenField_lreg {
    fn execution(&self) -> Register {
        meaning_2_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_2_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_dlreg(u8);
impl TokenField_dlreg {
    fn execution(&self) -> Register {
        meaning_2_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_2_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_imm7(u8);
impl TokenField_imm7 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_iwreg7(u8);
impl TokenField_iwreg7 {
    fn execution(&self) -> Register {
        meaning_3_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_3_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_addbit8(u8);
impl TokenField_addbit8 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_imm16(u16);
impl TokenField_imm16 {
    fn execution(&self) -> u16 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_disp16(i16);
impl TokenField_disp16 {
    fn execution(&self) -> i16 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op16(u8);
impl TokenField_op16 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_jmp11_hi(i8);
impl TokenField_jmp11_hi {
    fn execution(&self) -> i8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_jmp11_lo(u8);
impl TokenField_jmp11_lo {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
struct TokenParser<const LEN: usize>([u8; LEN]);
impl<const LEN: usize> TokenParser<LEN> {
    fn new(data: &[u8]) -> Option<Self> {
        let token_slice: &[u8] = data.get(..LEN)?;
        let token_data = <[u8; LEN]>::try_from(token_slice).unwrap();
        Some(Self(token_data))
    }
    fn TokenFieldop8(&self) -> TokenField_op8 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op8(inner_value)
    }
    fn TokenFieldop6(&self) -> TokenField_op6 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 2u64 as usize, 6u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op6(inner_value)
    }
    fn TokenFieldop5(&self) -> TokenField_op5 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op5(inner_value)
    }
    fn TokenFieldcond(&self) -> TokenField_cond {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_cond(inner_value)
    }
    fn TokenFieldop4(&self) -> TokenField_op4 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 4u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op4(inner_value)
    }
    fn TokenFieldaa(&self) -> TokenField_aa {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 2u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_aa(inner_value)
    }
    fn TokenFieldbitno(&self) -> TokenField_bitno {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_bitno(inner_value)
    }
    fn TokenFieldhighb(&self) -> TokenField_highb {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 4u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_highb(inner_value)
    }
    fn TokenFieldimm8(&self) -> TokenField_imm8 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<false>(work_value, 0u64 as usize, 8u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_imm8(inner_value)
    }
    fn TokenFieldsimm8(&self) -> TokenField_simm8 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<false>(work_value, 0u64 as usize, 8u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_simm8(inner_value)
    }
    fn TokenFieldbaop(&self) -> TokenField_baop {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_baop(inner_value)
    }
    fn TokenFieldbreg8(&self) -> TokenField_breg8 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_breg8(inner_value)
    }
    fn TokenFielddbreg(&self) -> TokenField_dbreg {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_dbreg(inner_value)
    }
    fn TokenFieldwaop(&self) -> TokenField_waop {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_waop(inner_value)
    }
    fn TokenFieldwreg8(&self) -> TokenField_wreg8 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_wreg8(inner_value)
    }
    fn TokenFielddwreg(&self) -> TokenField_dwreg {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_dwreg(inner_value)
    }
    fn TokenFieldlreg(&self) -> TokenField_lreg {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_lreg(inner_value)
    }
    fn TokenFielddlreg(&self) -> TokenField_dlreg {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_dlreg(inner_value)
    }
    fn TokenFieldimm7(&self) -> TokenField_imm7 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 1u64 as usize, 7u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_imm7(inner_value)
    }
    fn TokenFieldiwreg7(&self) -> TokenField_iwreg7 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 1u64 as usize, 7u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_iwreg7(inner_value)
    }
    fn TokenFieldaddbit8(&self) -> TokenField_addbit8 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_addbit8(inner_value)
    }
    fn TokenFieldimm16(&self) -> TokenField_imm16 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 0u64 as usize, 16u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_imm16(inner_value)
    }
    fn TokenFielddisp16(&self) -> TokenField_disp16 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i16::<false>(work_value, 0u64 as usize, 16u64 as usize);
            i16::try_from(value).unwrap()
        };
        TokenField_disp16(inner_value)
    }
    fn TokenFieldop16(&self) -> TokenField_op16 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op16(inner_value)
    }
    fn TokenFieldjmp11_hi(&self) -> TokenField_jmp11_hi {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<false>(work_value, 0u64 as usize, 3u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_jmp11_hi(inner_value)
    }
    fn TokenFieldjmp11_lo(&self) -> TokenField_jmp11_lo {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_jmp11_lo(inner_value)
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    PSW,
    PC,
    SP,
    ZRlo,
    ZRhi,
    AD_resultlo,
    AD_resulthi,
    HSI_timelo,
    HSI_timehi,
    HSI_status,
    SBUF,
    INT_MASK,
    INT_PEND,
    TIMER1lo,
    TIMER1hi,
    TIMER2lo,
    TIMER2hi,
    PORT0,
    PORT1,
    PORT2,
    SP_STAT,
    INT_PEND1,
    INT_MASK1,
    WSR,
    IOS0,
    IOS1,
    IOS2,
    ZR,
    AD_result,
    HSI_time,
    HSI_SBUF,
    INTERRUPT,
    TIMER1,
    TIMER2,
    PORT01,
    PORT2_SPS,
    INT1,
    WSR_IOS0,
    IOS12,
    ZR_AD,
    HSI,
    INT_TIMER1,
    TIMER2_PORT01,
    PORT2_INT1,
    WSR_IOS012,
    SPlo,
    SPhi,
    SPR,
    SPR1A,
    R1A,
    R1B,
    R1C,
    R1D,
    R1E,
    R1F,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R2A,
    R2B,
    R2C,
    R2D,
    R2E,
    R2F,
    R30,
    R31,
    R32,
    R33,
    R34,
    R35,
    R36,
    R37,
    R38,
    R39,
    R3A,
    R3B,
    R3C,
    R3D,
    R3E,
    R3F,
    R40,
    R41,
    R42,
    R43,
    R44,
    R45,
    R46,
    R47,
    R48,
    R49,
    R4A,
    R4B,
    R4C,
    R4D,
    R4E,
    R4F,
    R50,
    R51,
    R52,
    R53,
    R54,
    R55,
    R56,
    R57,
    R58,
    R59,
    R5A,
    R5B,
    R5C,
    R5D,
    R5E,
    R5F,
    R60,
    R61,
    R62,
    R63,
    R64,
    R65,
    R66,
    R67,
    R68,
    R69,
    R6A,
    R6B,
    R6C,
    R6D,
    R6E,
    R6F,
    R70,
    R71,
    R72,
    R73,
    R74,
    R75,
    R76,
    R77,
    R78,
    R79,
    R7A,
    R7B,
    R7C,
    R7D,
    R7E,
    R7F,
    R80,
    R81,
    R82,
    R83,
    R84,
    R85,
    R86,
    R87,
    R88,
    R89,
    R8A,
    R8B,
    R8C,
    R8D,
    R8E,
    R8F,
    R90,
    R91,
    R92,
    R93,
    R94,
    R95,
    R96,
    R97,
    R98,
    R99,
    R9A,
    R9B,
    R9C,
    R9D,
    R9E,
    R9F,
    RA0,
    RA1,
    RA2,
    RA3,
    RA4,
    RA5,
    RA6,
    RA7,
    RA8,
    RA9,
    RAA,
    RAB,
    RAC,
    RAD,
    RAE,
    RAF,
    RB0,
    RB1,
    RB2,
    RB3,
    RB4,
    RB5,
    RB6,
    RB7,
    RB8,
    RB9,
    RBA,
    RBB,
    RBC,
    RBD,
    RBE,
    RBF,
    RC0,
    RC1,
    RC2,
    RC3,
    RC4,
    RC5,
    RC6,
    RC7,
    RC8,
    RC9,
    RCA,
    RCB,
    RCC,
    RCD,
    RCE,
    RCF,
    RD0,
    RD1,
    RD2,
    RD3,
    RD4,
    RD5,
    RD6,
    RD7,
    RD8,
    RD9,
    RDA,
    RDB,
    RDC,
    RDD,
    RDE,
    RDF,
    RE0,
    RE1,
    RE2,
    RE3,
    RE4,
    RE5,
    RE6,
    RE7,
    RE8,
    RE9,
    REA,
    REB,
    REC,
    RED,
    REE,
    REF,
    RF0,
    RF1,
    RF2,
    RF3,
    RF4,
    RF5,
    RF6,
    RF7,
    RF8,
    RF9,
    RFA,
    RFB,
    RFC,
    RFD,
    RFE,
    RFF,
    R100,
    R101,
    R102,
    R103,
    R104,
    R105,
    R106,
    R107,
    R108,
    R109,
    R10A,
    R10B,
    R10C,
    R10D,
    R10E,
    R10F,
    R110,
    R111,
    R112,
    R113,
    R114,
    R115,
    R116,
    R117,
    R118,
    R119,
    R11A,
    R11B,
    R11C,
    R11D,
    R11E,
    R11F,
    R120,
    R121,
    R122,
    R123,
    R124,
    R125,
    R126,
    R127,
    R128,
    R129,
    R12A,
    R12B,
    R12C,
    R12D,
    R12E,
    R12F,
    R130,
    R131,
    R132,
    R133,
    R134,
    R135,
    R136,
    R137,
    R138,
    R139,
    R13A,
    R13B,
    R13C,
    R13D,
    R13E,
    R13F,
    R140,
    R141,
    R142,
    R143,
    R144,
    R145,
    R146,
    R147,
    R148,
    R149,
    R14A,
    R14B,
    R14C,
    R14D,
    R14E,
    R14F,
    R150,
    R151,
    R152,
    R153,
    R154,
    R155,
    R156,
    R157,
    R158,
    R159,
    R15A,
    R15B,
    R15C,
    R15D,
    R15E,
    R15F,
    R160,
    R161,
    R162,
    R163,
    R164,
    R165,
    R166,
    R167,
    R168,
    R169,
    R16A,
    R16B,
    R16C,
    R16D,
    R16E,
    R16F,
    R170,
    R171,
    R172,
    R173,
    R174,
    R175,
    R176,
    R177,
    R178,
    R179,
    R17A,
    R17B,
    R17C,
    R17D,
    R17E,
    R17F,
    R180,
    R181,
    R182,
    R183,
    R184,
    R185,
    R186,
    R187,
    R188,
    R189,
    R18A,
    R18B,
    R18C,
    R18D,
    R18E,
    R18F,
    R190,
    R191,
    R192,
    R193,
    R194,
    R195,
    R196,
    R197,
    R198,
    R199,
    R19A,
    R19B,
    R19C,
    R19D,
    R19E,
    R19F,
    R1A0,
    R1A1,
    R1A2,
    R1A3,
    R1A4,
    R1A5,
    R1A6,
    R1A7,
    R1A8,
    R1A9,
    R1AA,
    R1AB,
    R1AC,
    R1AD,
    R1AE,
    R1AF,
    R1B0,
    R1B1,
    R1B2,
    R1B3,
    R1B4,
    R1B5,
    R1B6,
    R1B7,
    R1B8,
    R1B9,
    R1BA,
    R1BB,
    R1BC,
    R1BD,
    R1BE,
    R1BF,
    R1C0,
    R1C1,
    R1C2,
    R1C3,
    R1C4,
    R1C5,
    R1C6,
    R1C7,
    R1C8,
    R1C9,
    R1CA,
    R1CB,
    R1CC,
    R1CD,
    R1CE,
    R1CF,
    R1D0,
    R1D1,
    R1D2,
    R1D3,
    R1D4,
    R1D5,
    R1D6,
    R1D7,
    R1D8,
    R1D9,
    R1DA,
    R1DB,
    R1DC,
    R1DD,
    R1DE,
    R1DF,
    R1E0,
    R1E1,
    R1E2,
    R1E3,
    R1E4,
    R1E5,
    R1E6,
    R1E7,
    R1E8,
    R1E9,
    R1EA,
    R1EB,
    R1EC,
    R1ED,
    R1EE,
    R1EF,
    R1F0,
    R1F1,
    R1F2,
    R1F3,
    R1F4,
    R1F5,
    R1F6,
    R1F7,
    R1F8,
    R1F9,
    R1FA,
    R1FB,
    R1FC,
    R1FD,
    R1FE,
    R1FF,
    RW1A,
    RW1C,
    RW1E,
    RW20,
    RW22,
    RW24,
    RW26,
    RW28,
    RW2A,
    RW2C,
    RW2E,
    RW30,
    RW32,
    RW34,
    RW36,
    RW38,
    RW3A,
    RW3C,
    RW3E,
    RW40,
    RW42,
    RW44,
    RW46,
    RW48,
    RW4A,
    RW4C,
    RW4E,
    RW50,
    RW52,
    RW54,
    RW56,
    RW58,
    RW5A,
    RW5C,
    RW5E,
    RW60,
    RW62,
    RW64,
    RW66,
    RW68,
    RW6A,
    RW6C,
    RW6E,
    RW70,
    RW72,
    RW74,
    RW76,
    RW78,
    RW7A,
    RW7C,
    RW7E,
    RW80,
    RW82,
    RW84,
    RW86,
    RW88,
    RW8A,
    RW8C,
    RW8E,
    RW90,
    RW92,
    RW94,
    RW96,
    RW98,
    RW9A,
    RW9C,
    RW9E,
    RWA0,
    RWA2,
    RWA4,
    RWA6,
    RWA8,
    RWAA,
    RWAC,
    RWAE,
    RWB0,
    RWB2,
    RWB4,
    RWB6,
    RWB8,
    RWBA,
    RWBC,
    RWBE,
    RWC0,
    RWC2,
    RWC4,
    RWC6,
    RWC8,
    RWCA,
    RWCC,
    RWCE,
    RWD0,
    RWD2,
    RWD4,
    RWD6,
    RWD8,
    RWDA,
    RWDC,
    RWDE,
    RWE0,
    RWE2,
    RWE4,
    RWE6,
    RWE8,
    RWEA,
    RWEC,
    RWEE,
    RWF0,
    RWF2,
    RWF4,
    RWF6,
    RWF8,
    RWFA,
    RWFC,
    RWFE,
    RW100,
    RW102,
    RW104,
    RW106,
    RW108,
    RW10A,
    RW10C,
    RW10E,
    RW110,
    RW112,
    RW114,
    RW116,
    RW118,
    RW11A,
    RW11C,
    RW11E,
    RW120,
    RW122,
    RW124,
    RW126,
    RW128,
    RW12A,
    RW12C,
    RW12E,
    RW130,
    RW132,
    RW134,
    RW136,
    RW138,
    RW13A,
    RW13C,
    RW13E,
    RW140,
    RW142,
    RW144,
    RW146,
    RW148,
    RW14A,
    RW14C,
    RW14E,
    RW150,
    RW152,
    RW154,
    RW156,
    RW158,
    RW15A,
    RW15C,
    RW15E,
    RW160,
    RW162,
    RW164,
    RW166,
    RW168,
    RW16A,
    RW16C,
    RW16E,
    RW170,
    RW172,
    RW174,
    RW176,
    RW178,
    RW17A,
    RW17C,
    RW17E,
    RW180,
    RW182,
    RW184,
    RW186,
    RW188,
    RW18A,
    RW18C,
    RW18E,
    RW190,
    RW192,
    RW194,
    RW196,
    RW198,
    RW19A,
    RW19C,
    RW19E,
    RW1A0,
    RW1A2,
    RW1A4,
    RW1A6,
    RW1A8,
    RW1AA,
    RW1AC,
    RW1AE,
    RW1B0,
    RW1B2,
    RW1B4,
    RW1B6,
    RW1B8,
    RW1BA,
    RW1BC,
    RW1BE,
    RW1C0,
    RW1C2,
    RW1C4,
    RW1C6,
    RW1C8,
    RW1CA,
    RW1CC,
    RW1CE,
    RW1D0,
    RW1D2,
    RW1D4,
    RW1D6,
    RW1D8,
    RW1DA,
    RW1DC,
    RW1DE,
    RW1E0,
    RW1E2,
    RW1E4,
    RW1E6,
    RW1E8,
    RW1EA,
    RW1EC,
    RW1EE,
    RW1F0,
    RW1F2,
    RW1F4,
    RW1F6,
    RW1F8,
    RW1FA,
    RW1FC,
    RW1FE,
    RL1C,
    RL20,
    RL24,
    RL28,
    RL2C,
    RL30,
    RL34,
    RL38,
    RL3C,
    RL40,
    RL44,
    RL48,
    RL4C,
    RL50,
    RL54,
    RL58,
    RL5C,
    RL60,
    RL64,
    RL68,
    RL6C,
    RL70,
    RL74,
    RL78,
    RL7C,
    RL80,
    RL84,
    RL88,
    RL8C,
    RL90,
    RL94,
    RL98,
    RL9C,
    RLA0,
    RLA4,
    RLA8,
    RLAC,
    RLB0,
    RLB4,
    RLB8,
    RLBC,
    RLC0,
    RLC4,
    RLC8,
    RLCC,
    RLD0,
    RLD4,
    RLD8,
    RLDC,
    RLE0,
    RLE4,
    RLE8,
    RLEC,
    RLF0,
    RLF4,
    RLF8,
    RLFC,
    RL100,
    RL104,
    RL108,
    RL10C,
    RL110,
    RL114,
    RL118,
    RL11C,
    RL120,
    RL124,
    RL128,
    RL12C,
    RL130,
    RL134,
    RL138,
    RL13C,
    RL140,
    RL144,
    RL148,
    RL14C,
    RL150,
    RL154,
    RL158,
    RL15C,
    RL160,
    RL164,
    RL168,
    RL16C,
    RL170,
    RL174,
    RL178,
    RL17C,
    RL180,
    RL184,
    RL188,
    RL18C,
    RL190,
    RL194,
    RL198,
    RL19C,
    RL1A0,
    RL1A4,
    RL1A8,
    RL1AC,
    RL1B0,
    RL1B4,
    RL1B8,
    RL1BC,
    RL1C0,
    RL1C4,
    RL1C8,
    RL1CC,
    RL1D0,
    RL1D4,
    RL1D8,
    RL1DC,
    RL1E0,
    RL1E4,
    RL1E8,
    RL1EC,
    RL1F0,
    RL1F4,
    RL1F8,
    RL1FC,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PSW => write!(f, "PSW"),
            Self::PC => write!(f, "PC"),
            Self::SP => write!(f, "SP"),
            Self::ZRlo => write!(f, "ZRlo"),
            Self::ZRhi => write!(f, "ZRhi"),
            Self::AD_resultlo => write!(f, "AD_resultlo"),
            Self::AD_resulthi => write!(f, "AD_resulthi"),
            Self::HSI_timelo => write!(f, "HSI_timelo"),
            Self::HSI_timehi => write!(f, "HSI_timehi"),
            Self::HSI_status => write!(f, "HSI_status"),
            Self::SBUF => write!(f, "SBUF"),
            Self::INT_MASK => write!(f, "INT_MASK"),
            Self::INT_PEND => write!(f, "INT_PEND"),
            Self::TIMER1lo => write!(f, "TIMER1lo"),
            Self::TIMER1hi => write!(f, "TIMER1hi"),
            Self::TIMER2lo => write!(f, "TIMER2lo"),
            Self::TIMER2hi => write!(f, "TIMER2hi"),
            Self::PORT0 => write!(f, "PORT0"),
            Self::PORT1 => write!(f, "PORT1"),
            Self::PORT2 => write!(f, "PORT2"),
            Self::SP_STAT => write!(f, "SP_STAT"),
            Self::INT_PEND1 => write!(f, "INT_PEND1"),
            Self::INT_MASK1 => write!(f, "INT_MASK1"),
            Self::WSR => write!(f, "WSR"),
            Self::IOS0 => write!(f, "IOS0"),
            Self::IOS1 => write!(f, "IOS1"),
            Self::IOS2 => write!(f, "IOS2"),
            Self::ZR => write!(f, "ZR"),
            Self::AD_result => write!(f, "AD_result"),
            Self::HSI_time => write!(f, "HSI_time"),
            Self::HSI_SBUF => write!(f, "HSI_SBUF"),
            Self::INTERRUPT => write!(f, "INTERRUPT"),
            Self::TIMER1 => write!(f, "TIMER1"),
            Self::TIMER2 => write!(f, "TIMER2"),
            Self::PORT01 => write!(f, "PORT01"),
            Self::PORT2_SPS => write!(f, "PORT2_SPS"),
            Self::INT1 => write!(f, "INT1"),
            Self::WSR_IOS0 => write!(f, "WSR_IOS0"),
            Self::IOS12 => write!(f, "IOS12"),
            Self::ZR_AD => write!(f, "ZR_AD"),
            Self::HSI => write!(f, "HSI"),
            Self::INT_TIMER1 => write!(f, "INT_TIMER1"),
            Self::TIMER2_PORT01 => write!(f, "TIMER2_PORT01"),
            Self::PORT2_INT1 => write!(f, "PORT2_INT1"),
            Self::WSR_IOS012 => write!(f, "WSR_IOS012"),
            Self::SPlo => write!(f, "SPlo"),
            Self::SPhi => write!(f, "SPhi"),
            Self::SPR => write!(f, "SPR"),
            Self::SPR1A => write!(f, "SPR1A"),
            Self::R1A => write!(f, "R1A"),
            Self::R1B => write!(f, "R1B"),
            Self::R1C => write!(f, "R1C"),
            Self::R1D => write!(f, "R1D"),
            Self::R1E => write!(f, "R1E"),
            Self::R1F => write!(f, "R1F"),
            Self::R20 => write!(f, "R20"),
            Self::R21 => write!(f, "R21"),
            Self::R22 => write!(f, "R22"),
            Self::R23 => write!(f, "R23"),
            Self::R24 => write!(f, "R24"),
            Self::R25 => write!(f, "R25"),
            Self::R26 => write!(f, "R26"),
            Self::R27 => write!(f, "R27"),
            Self::R28 => write!(f, "R28"),
            Self::R29 => write!(f, "R29"),
            Self::R2A => write!(f, "R2A"),
            Self::R2B => write!(f, "R2B"),
            Self::R2C => write!(f, "R2C"),
            Self::R2D => write!(f, "R2D"),
            Self::R2E => write!(f, "R2E"),
            Self::R2F => write!(f, "R2F"),
            Self::R30 => write!(f, "R30"),
            Self::R31 => write!(f, "R31"),
            Self::R32 => write!(f, "R32"),
            Self::R33 => write!(f, "R33"),
            Self::R34 => write!(f, "R34"),
            Self::R35 => write!(f, "R35"),
            Self::R36 => write!(f, "R36"),
            Self::R37 => write!(f, "R37"),
            Self::R38 => write!(f, "R38"),
            Self::R39 => write!(f, "R39"),
            Self::R3A => write!(f, "R3A"),
            Self::R3B => write!(f, "R3B"),
            Self::R3C => write!(f, "R3C"),
            Self::R3D => write!(f, "R3D"),
            Self::R3E => write!(f, "R3E"),
            Self::R3F => write!(f, "R3F"),
            Self::R40 => write!(f, "R40"),
            Self::R41 => write!(f, "R41"),
            Self::R42 => write!(f, "R42"),
            Self::R43 => write!(f, "R43"),
            Self::R44 => write!(f, "R44"),
            Self::R45 => write!(f, "R45"),
            Self::R46 => write!(f, "R46"),
            Self::R47 => write!(f, "R47"),
            Self::R48 => write!(f, "R48"),
            Self::R49 => write!(f, "R49"),
            Self::R4A => write!(f, "R4A"),
            Self::R4B => write!(f, "R4B"),
            Self::R4C => write!(f, "R4C"),
            Self::R4D => write!(f, "R4D"),
            Self::R4E => write!(f, "R4E"),
            Self::R4F => write!(f, "R4F"),
            Self::R50 => write!(f, "R50"),
            Self::R51 => write!(f, "R51"),
            Self::R52 => write!(f, "R52"),
            Self::R53 => write!(f, "R53"),
            Self::R54 => write!(f, "R54"),
            Self::R55 => write!(f, "R55"),
            Self::R56 => write!(f, "R56"),
            Self::R57 => write!(f, "R57"),
            Self::R58 => write!(f, "R58"),
            Self::R59 => write!(f, "R59"),
            Self::R5A => write!(f, "R5A"),
            Self::R5B => write!(f, "R5B"),
            Self::R5C => write!(f, "R5C"),
            Self::R5D => write!(f, "R5D"),
            Self::R5E => write!(f, "R5E"),
            Self::R5F => write!(f, "R5F"),
            Self::R60 => write!(f, "R60"),
            Self::R61 => write!(f, "R61"),
            Self::R62 => write!(f, "R62"),
            Self::R63 => write!(f, "R63"),
            Self::R64 => write!(f, "R64"),
            Self::R65 => write!(f, "R65"),
            Self::R66 => write!(f, "R66"),
            Self::R67 => write!(f, "R67"),
            Self::R68 => write!(f, "R68"),
            Self::R69 => write!(f, "R69"),
            Self::R6A => write!(f, "R6A"),
            Self::R6B => write!(f, "R6B"),
            Self::R6C => write!(f, "R6C"),
            Self::R6D => write!(f, "R6D"),
            Self::R6E => write!(f, "R6E"),
            Self::R6F => write!(f, "R6F"),
            Self::R70 => write!(f, "R70"),
            Self::R71 => write!(f, "R71"),
            Self::R72 => write!(f, "R72"),
            Self::R73 => write!(f, "R73"),
            Self::R74 => write!(f, "R74"),
            Self::R75 => write!(f, "R75"),
            Self::R76 => write!(f, "R76"),
            Self::R77 => write!(f, "R77"),
            Self::R78 => write!(f, "R78"),
            Self::R79 => write!(f, "R79"),
            Self::R7A => write!(f, "R7A"),
            Self::R7B => write!(f, "R7B"),
            Self::R7C => write!(f, "R7C"),
            Self::R7D => write!(f, "R7D"),
            Self::R7E => write!(f, "R7E"),
            Self::R7F => write!(f, "R7F"),
            Self::R80 => write!(f, "R80"),
            Self::R81 => write!(f, "R81"),
            Self::R82 => write!(f, "R82"),
            Self::R83 => write!(f, "R83"),
            Self::R84 => write!(f, "R84"),
            Self::R85 => write!(f, "R85"),
            Self::R86 => write!(f, "R86"),
            Self::R87 => write!(f, "R87"),
            Self::R88 => write!(f, "R88"),
            Self::R89 => write!(f, "R89"),
            Self::R8A => write!(f, "R8A"),
            Self::R8B => write!(f, "R8B"),
            Self::R8C => write!(f, "R8C"),
            Self::R8D => write!(f, "R8D"),
            Self::R8E => write!(f, "R8E"),
            Self::R8F => write!(f, "R8F"),
            Self::R90 => write!(f, "R90"),
            Self::R91 => write!(f, "R91"),
            Self::R92 => write!(f, "R92"),
            Self::R93 => write!(f, "R93"),
            Self::R94 => write!(f, "R94"),
            Self::R95 => write!(f, "R95"),
            Self::R96 => write!(f, "R96"),
            Self::R97 => write!(f, "R97"),
            Self::R98 => write!(f, "R98"),
            Self::R99 => write!(f, "R99"),
            Self::R9A => write!(f, "R9A"),
            Self::R9B => write!(f, "R9B"),
            Self::R9C => write!(f, "R9C"),
            Self::R9D => write!(f, "R9D"),
            Self::R9E => write!(f, "R9E"),
            Self::R9F => write!(f, "R9F"),
            Self::RA0 => write!(f, "RA0"),
            Self::RA1 => write!(f, "RA1"),
            Self::RA2 => write!(f, "RA2"),
            Self::RA3 => write!(f, "RA3"),
            Self::RA4 => write!(f, "RA4"),
            Self::RA5 => write!(f, "RA5"),
            Self::RA6 => write!(f, "RA6"),
            Self::RA7 => write!(f, "RA7"),
            Self::RA8 => write!(f, "RA8"),
            Self::RA9 => write!(f, "RA9"),
            Self::RAA => write!(f, "RAA"),
            Self::RAB => write!(f, "RAB"),
            Self::RAC => write!(f, "RAC"),
            Self::RAD => write!(f, "RAD"),
            Self::RAE => write!(f, "RAE"),
            Self::RAF => write!(f, "RAF"),
            Self::RB0 => write!(f, "RB0"),
            Self::RB1 => write!(f, "RB1"),
            Self::RB2 => write!(f, "RB2"),
            Self::RB3 => write!(f, "RB3"),
            Self::RB4 => write!(f, "RB4"),
            Self::RB5 => write!(f, "RB5"),
            Self::RB6 => write!(f, "RB6"),
            Self::RB7 => write!(f, "RB7"),
            Self::RB8 => write!(f, "RB8"),
            Self::RB9 => write!(f, "RB9"),
            Self::RBA => write!(f, "RBA"),
            Self::RBB => write!(f, "RBB"),
            Self::RBC => write!(f, "RBC"),
            Self::RBD => write!(f, "RBD"),
            Self::RBE => write!(f, "RBE"),
            Self::RBF => write!(f, "RBF"),
            Self::RC0 => write!(f, "RC0"),
            Self::RC1 => write!(f, "RC1"),
            Self::RC2 => write!(f, "RC2"),
            Self::RC3 => write!(f, "RC3"),
            Self::RC4 => write!(f, "RC4"),
            Self::RC5 => write!(f, "RC5"),
            Self::RC6 => write!(f, "RC6"),
            Self::RC7 => write!(f, "RC7"),
            Self::RC8 => write!(f, "RC8"),
            Self::RC9 => write!(f, "RC9"),
            Self::RCA => write!(f, "RCA"),
            Self::RCB => write!(f, "RCB"),
            Self::RCC => write!(f, "RCC"),
            Self::RCD => write!(f, "RCD"),
            Self::RCE => write!(f, "RCE"),
            Self::RCF => write!(f, "RCF"),
            Self::RD0 => write!(f, "RD0"),
            Self::RD1 => write!(f, "RD1"),
            Self::RD2 => write!(f, "RD2"),
            Self::RD3 => write!(f, "RD3"),
            Self::RD4 => write!(f, "RD4"),
            Self::RD5 => write!(f, "RD5"),
            Self::RD6 => write!(f, "RD6"),
            Self::RD7 => write!(f, "RD7"),
            Self::RD8 => write!(f, "RD8"),
            Self::RD9 => write!(f, "RD9"),
            Self::RDA => write!(f, "RDA"),
            Self::RDB => write!(f, "RDB"),
            Self::RDC => write!(f, "RDC"),
            Self::RDD => write!(f, "RDD"),
            Self::RDE => write!(f, "RDE"),
            Self::RDF => write!(f, "RDF"),
            Self::RE0 => write!(f, "RE0"),
            Self::RE1 => write!(f, "RE1"),
            Self::RE2 => write!(f, "RE2"),
            Self::RE3 => write!(f, "RE3"),
            Self::RE4 => write!(f, "RE4"),
            Self::RE5 => write!(f, "RE5"),
            Self::RE6 => write!(f, "RE6"),
            Self::RE7 => write!(f, "RE7"),
            Self::RE8 => write!(f, "RE8"),
            Self::RE9 => write!(f, "RE9"),
            Self::REA => write!(f, "REA"),
            Self::REB => write!(f, "REB"),
            Self::REC => write!(f, "REC"),
            Self::RED => write!(f, "RED"),
            Self::REE => write!(f, "REE"),
            Self::REF => write!(f, "REF"),
            Self::RF0 => write!(f, "RF0"),
            Self::RF1 => write!(f, "RF1"),
            Self::RF2 => write!(f, "RF2"),
            Self::RF3 => write!(f, "RF3"),
            Self::RF4 => write!(f, "RF4"),
            Self::RF5 => write!(f, "RF5"),
            Self::RF6 => write!(f, "RF6"),
            Self::RF7 => write!(f, "RF7"),
            Self::RF8 => write!(f, "RF8"),
            Self::RF9 => write!(f, "RF9"),
            Self::RFA => write!(f, "RFA"),
            Self::RFB => write!(f, "RFB"),
            Self::RFC => write!(f, "RFC"),
            Self::RFD => write!(f, "RFD"),
            Self::RFE => write!(f, "RFE"),
            Self::RFF => write!(f, "RFF"),
            Self::R100 => write!(f, "R100"),
            Self::R101 => write!(f, "R101"),
            Self::R102 => write!(f, "R102"),
            Self::R103 => write!(f, "R103"),
            Self::R104 => write!(f, "R104"),
            Self::R105 => write!(f, "R105"),
            Self::R106 => write!(f, "R106"),
            Self::R107 => write!(f, "R107"),
            Self::R108 => write!(f, "R108"),
            Self::R109 => write!(f, "R109"),
            Self::R10A => write!(f, "R10A"),
            Self::R10B => write!(f, "R10B"),
            Self::R10C => write!(f, "R10C"),
            Self::R10D => write!(f, "R10D"),
            Self::R10E => write!(f, "R10E"),
            Self::R10F => write!(f, "R10F"),
            Self::R110 => write!(f, "R110"),
            Self::R111 => write!(f, "R111"),
            Self::R112 => write!(f, "R112"),
            Self::R113 => write!(f, "R113"),
            Self::R114 => write!(f, "R114"),
            Self::R115 => write!(f, "R115"),
            Self::R116 => write!(f, "R116"),
            Self::R117 => write!(f, "R117"),
            Self::R118 => write!(f, "R118"),
            Self::R119 => write!(f, "R119"),
            Self::R11A => write!(f, "R11A"),
            Self::R11B => write!(f, "R11B"),
            Self::R11C => write!(f, "R11C"),
            Self::R11D => write!(f, "R11D"),
            Self::R11E => write!(f, "R11E"),
            Self::R11F => write!(f, "R11F"),
            Self::R120 => write!(f, "R120"),
            Self::R121 => write!(f, "R121"),
            Self::R122 => write!(f, "R122"),
            Self::R123 => write!(f, "R123"),
            Self::R124 => write!(f, "R124"),
            Self::R125 => write!(f, "R125"),
            Self::R126 => write!(f, "R126"),
            Self::R127 => write!(f, "R127"),
            Self::R128 => write!(f, "R128"),
            Self::R129 => write!(f, "R129"),
            Self::R12A => write!(f, "R12A"),
            Self::R12B => write!(f, "R12B"),
            Self::R12C => write!(f, "R12C"),
            Self::R12D => write!(f, "R12D"),
            Self::R12E => write!(f, "R12E"),
            Self::R12F => write!(f, "R12F"),
            Self::R130 => write!(f, "R130"),
            Self::R131 => write!(f, "R131"),
            Self::R132 => write!(f, "R132"),
            Self::R133 => write!(f, "R133"),
            Self::R134 => write!(f, "R134"),
            Self::R135 => write!(f, "R135"),
            Self::R136 => write!(f, "R136"),
            Self::R137 => write!(f, "R137"),
            Self::R138 => write!(f, "R138"),
            Self::R139 => write!(f, "R139"),
            Self::R13A => write!(f, "R13A"),
            Self::R13B => write!(f, "R13B"),
            Self::R13C => write!(f, "R13C"),
            Self::R13D => write!(f, "R13D"),
            Self::R13E => write!(f, "R13E"),
            Self::R13F => write!(f, "R13F"),
            Self::R140 => write!(f, "R140"),
            Self::R141 => write!(f, "R141"),
            Self::R142 => write!(f, "R142"),
            Self::R143 => write!(f, "R143"),
            Self::R144 => write!(f, "R144"),
            Self::R145 => write!(f, "R145"),
            Self::R146 => write!(f, "R146"),
            Self::R147 => write!(f, "R147"),
            Self::R148 => write!(f, "R148"),
            Self::R149 => write!(f, "R149"),
            Self::R14A => write!(f, "R14A"),
            Self::R14B => write!(f, "R14B"),
            Self::R14C => write!(f, "R14C"),
            Self::R14D => write!(f, "R14D"),
            Self::R14E => write!(f, "R14E"),
            Self::R14F => write!(f, "R14F"),
            Self::R150 => write!(f, "R150"),
            Self::R151 => write!(f, "R151"),
            Self::R152 => write!(f, "R152"),
            Self::R153 => write!(f, "R153"),
            Self::R154 => write!(f, "R154"),
            Self::R155 => write!(f, "R155"),
            Self::R156 => write!(f, "R156"),
            Self::R157 => write!(f, "R157"),
            Self::R158 => write!(f, "R158"),
            Self::R159 => write!(f, "R159"),
            Self::R15A => write!(f, "R15A"),
            Self::R15B => write!(f, "R15B"),
            Self::R15C => write!(f, "R15C"),
            Self::R15D => write!(f, "R15D"),
            Self::R15E => write!(f, "R15E"),
            Self::R15F => write!(f, "R15F"),
            Self::R160 => write!(f, "R160"),
            Self::R161 => write!(f, "R161"),
            Self::R162 => write!(f, "R162"),
            Self::R163 => write!(f, "R163"),
            Self::R164 => write!(f, "R164"),
            Self::R165 => write!(f, "R165"),
            Self::R166 => write!(f, "R166"),
            Self::R167 => write!(f, "R167"),
            Self::R168 => write!(f, "R168"),
            Self::R169 => write!(f, "R169"),
            Self::R16A => write!(f, "R16A"),
            Self::R16B => write!(f, "R16B"),
            Self::R16C => write!(f, "R16C"),
            Self::R16D => write!(f, "R16D"),
            Self::R16E => write!(f, "R16E"),
            Self::R16F => write!(f, "R16F"),
            Self::R170 => write!(f, "R170"),
            Self::R171 => write!(f, "R171"),
            Self::R172 => write!(f, "R172"),
            Self::R173 => write!(f, "R173"),
            Self::R174 => write!(f, "R174"),
            Self::R175 => write!(f, "R175"),
            Self::R176 => write!(f, "R176"),
            Self::R177 => write!(f, "R177"),
            Self::R178 => write!(f, "R178"),
            Self::R179 => write!(f, "R179"),
            Self::R17A => write!(f, "R17A"),
            Self::R17B => write!(f, "R17B"),
            Self::R17C => write!(f, "R17C"),
            Self::R17D => write!(f, "R17D"),
            Self::R17E => write!(f, "R17E"),
            Self::R17F => write!(f, "R17F"),
            Self::R180 => write!(f, "R180"),
            Self::R181 => write!(f, "R181"),
            Self::R182 => write!(f, "R182"),
            Self::R183 => write!(f, "R183"),
            Self::R184 => write!(f, "R184"),
            Self::R185 => write!(f, "R185"),
            Self::R186 => write!(f, "R186"),
            Self::R187 => write!(f, "R187"),
            Self::R188 => write!(f, "R188"),
            Self::R189 => write!(f, "R189"),
            Self::R18A => write!(f, "R18A"),
            Self::R18B => write!(f, "R18B"),
            Self::R18C => write!(f, "R18C"),
            Self::R18D => write!(f, "R18D"),
            Self::R18E => write!(f, "R18E"),
            Self::R18F => write!(f, "R18F"),
            Self::R190 => write!(f, "R190"),
            Self::R191 => write!(f, "R191"),
            Self::R192 => write!(f, "R192"),
            Self::R193 => write!(f, "R193"),
            Self::R194 => write!(f, "R194"),
            Self::R195 => write!(f, "R195"),
            Self::R196 => write!(f, "R196"),
            Self::R197 => write!(f, "R197"),
            Self::R198 => write!(f, "R198"),
            Self::R199 => write!(f, "R199"),
            Self::R19A => write!(f, "R19A"),
            Self::R19B => write!(f, "R19B"),
            Self::R19C => write!(f, "R19C"),
            Self::R19D => write!(f, "R19D"),
            Self::R19E => write!(f, "R19E"),
            Self::R19F => write!(f, "R19F"),
            Self::R1A0 => write!(f, "R1A0"),
            Self::R1A1 => write!(f, "R1A1"),
            Self::R1A2 => write!(f, "R1A2"),
            Self::R1A3 => write!(f, "R1A3"),
            Self::R1A4 => write!(f, "R1A4"),
            Self::R1A5 => write!(f, "R1A5"),
            Self::R1A6 => write!(f, "R1A6"),
            Self::R1A7 => write!(f, "R1A7"),
            Self::R1A8 => write!(f, "R1A8"),
            Self::R1A9 => write!(f, "R1A9"),
            Self::R1AA => write!(f, "R1AA"),
            Self::R1AB => write!(f, "R1AB"),
            Self::R1AC => write!(f, "R1AC"),
            Self::R1AD => write!(f, "R1AD"),
            Self::R1AE => write!(f, "R1AE"),
            Self::R1AF => write!(f, "R1AF"),
            Self::R1B0 => write!(f, "R1B0"),
            Self::R1B1 => write!(f, "R1B1"),
            Self::R1B2 => write!(f, "R1B2"),
            Self::R1B3 => write!(f, "R1B3"),
            Self::R1B4 => write!(f, "R1B4"),
            Self::R1B5 => write!(f, "R1B5"),
            Self::R1B6 => write!(f, "R1B6"),
            Self::R1B7 => write!(f, "R1B7"),
            Self::R1B8 => write!(f, "R1B8"),
            Self::R1B9 => write!(f, "R1B9"),
            Self::R1BA => write!(f, "R1BA"),
            Self::R1BB => write!(f, "R1BB"),
            Self::R1BC => write!(f, "R1BC"),
            Self::R1BD => write!(f, "R1BD"),
            Self::R1BE => write!(f, "R1BE"),
            Self::R1BF => write!(f, "R1BF"),
            Self::R1C0 => write!(f, "R1C0"),
            Self::R1C1 => write!(f, "R1C1"),
            Self::R1C2 => write!(f, "R1C2"),
            Self::R1C3 => write!(f, "R1C3"),
            Self::R1C4 => write!(f, "R1C4"),
            Self::R1C5 => write!(f, "R1C5"),
            Self::R1C6 => write!(f, "R1C6"),
            Self::R1C7 => write!(f, "R1C7"),
            Self::R1C8 => write!(f, "R1C8"),
            Self::R1C9 => write!(f, "R1C9"),
            Self::R1CA => write!(f, "R1CA"),
            Self::R1CB => write!(f, "R1CB"),
            Self::R1CC => write!(f, "R1CC"),
            Self::R1CD => write!(f, "R1CD"),
            Self::R1CE => write!(f, "R1CE"),
            Self::R1CF => write!(f, "R1CF"),
            Self::R1D0 => write!(f, "R1D0"),
            Self::R1D1 => write!(f, "R1D1"),
            Self::R1D2 => write!(f, "R1D2"),
            Self::R1D3 => write!(f, "R1D3"),
            Self::R1D4 => write!(f, "R1D4"),
            Self::R1D5 => write!(f, "R1D5"),
            Self::R1D6 => write!(f, "R1D6"),
            Self::R1D7 => write!(f, "R1D7"),
            Self::R1D8 => write!(f, "R1D8"),
            Self::R1D9 => write!(f, "R1D9"),
            Self::R1DA => write!(f, "R1DA"),
            Self::R1DB => write!(f, "R1DB"),
            Self::R1DC => write!(f, "R1DC"),
            Self::R1DD => write!(f, "R1DD"),
            Self::R1DE => write!(f, "R1DE"),
            Self::R1DF => write!(f, "R1DF"),
            Self::R1E0 => write!(f, "R1E0"),
            Self::R1E1 => write!(f, "R1E1"),
            Self::R1E2 => write!(f, "R1E2"),
            Self::R1E3 => write!(f, "R1E3"),
            Self::R1E4 => write!(f, "R1E4"),
            Self::R1E5 => write!(f, "R1E5"),
            Self::R1E6 => write!(f, "R1E6"),
            Self::R1E7 => write!(f, "R1E7"),
            Self::R1E8 => write!(f, "R1E8"),
            Self::R1E9 => write!(f, "R1E9"),
            Self::R1EA => write!(f, "R1EA"),
            Self::R1EB => write!(f, "R1EB"),
            Self::R1EC => write!(f, "R1EC"),
            Self::R1ED => write!(f, "R1ED"),
            Self::R1EE => write!(f, "R1EE"),
            Self::R1EF => write!(f, "R1EF"),
            Self::R1F0 => write!(f, "R1F0"),
            Self::R1F1 => write!(f, "R1F1"),
            Self::R1F2 => write!(f, "R1F2"),
            Self::R1F3 => write!(f, "R1F3"),
            Self::R1F4 => write!(f, "R1F4"),
            Self::R1F5 => write!(f, "R1F5"),
            Self::R1F6 => write!(f, "R1F6"),
            Self::R1F7 => write!(f, "R1F7"),
            Self::R1F8 => write!(f, "R1F8"),
            Self::R1F9 => write!(f, "R1F9"),
            Self::R1FA => write!(f, "R1FA"),
            Self::R1FB => write!(f, "R1FB"),
            Self::R1FC => write!(f, "R1FC"),
            Self::R1FD => write!(f, "R1FD"),
            Self::R1FE => write!(f, "R1FE"),
            Self::R1FF => write!(f, "R1FF"),
            Self::RW1A => write!(f, "RW1A"),
            Self::RW1C => write!(f, "RW1C"),
            Self::RW1E => write!(f, "RW1E"),
            Self::RW20 => write!(f, "RW20"),
            Self::RW22 => write!(f, "RW22"),
            Self::RW24 => write!(f, "RW24"),
            Self::RW26 => write!(f, "RW26"),
            Self::RW28 => write!(f, "RW28"),
            Self::RW2A => write!(f, "RW2A"),
            Self::RW2C => write!(f, "RW2C"),
            Self::RW2E => write!(f, "RW2E"),
            Self::RW30 => write!(f, "RW30"),
            Self::RW32 => write!(f, "RW32"),
            Self::RW34 => write!(f, "RW34"),
            Self::RW36 => write!(f, "RW36"),
            Self::RW38 => write!(f, "RW38"),
            Self::RW3A => write!(f, "RW3A"),
            Self::RW3C => write!(f, "RW3C"),
            Self::RW3E => write!(f, "RW3E"),
            Self::RW40 => write!(f, "RW40"),
            Self::RW42 => write!(f, "RW42"),
            Self::RW44 => write!(f, "RW44"),
            Self::RW46 => write!(f, "RW46"),
            Self::RW48 => write!(f, "RW48"),
            Self::RW4A => write!(f, "RW4A"),
            Self::RW4C => write!(f, "RW4C"),
            Self::RW4E => write!(f, "RW4E"),
            Self::RW50 => write!(f, "RW50"),
            Self::RW52 => write!(f, "RW52"),
            Self::RW54 => write!(f, "RW54"),
            Self::RW56 => write!(f, "RW56"),
            Self::RW58 => write!(f, "RW58"),
            Self::RW5A => write!(f, "RW5A"),
            Self::RW5C => write!(f, "RW5C"),
            Self::RW5E => write!(f, "RW5E"),
            Self::RW60 => write!(f, "RW60"),
            Self::RW62 => write!(f, "RW62"),
            Self::RW64 => write!(f, "RW64"),
            Self::RW66 => write!(f, "RW66"),
            Self::RW68 => write!(f, "RW68"),
            Self::RW6A => write!(f, "RW6A"),
            Self::RW6C => write!(f, "RW6C"),
            Self::RW6E => write!(f, "RW6E"),
            Self::RW70 => write!(f, "RW70"),
            Self::RW72 => write!(f, "RW72"),
            Self::RW74 => write!(f, "RW74"),
            Self::RW76 => write!(f, "RW76"),
            Self::RW78 => write!(f, "RW78"),
            Self::RW7A => write!(f, "RW7A"),
            Self::RW7C => write!(f, "RW7C"),
            Self::RW7E => write!(f, "RW7E"),
            Self::RW80 => write!(f, "RW80"),
            Self::RW82 => write!(f, "RW82"),
            Self::RW84 => write!(f, "RW84"),
            Self::RW86 => write!(f, "RW86"),
            Self::RW88 => write!(f, "RW88"),
            Self::RW8A => write!(f, "RW8A"),
            Self::RW8C => write!(f, "RW8C"),
            Self::RW8E => write!(f, "RW8E"),
            Self::RW90 => write!(f, "RW90"),
            Self::RW92 => write!(f, "RW92"),
            Self::RW94 => write!(f, "RW94"),
            Self::RW96 => write!(f, "RW96"),
            Self::RW98 => write!(f, "RW98"),
            Self::RW9A => write!(f, "RW9A"),
            Self::RW9C => write!(f, "RW9C"),
            Self::RW9E => write!(f, "RW9E"),
            Self::RWA0 => write!(f, "RWA0"),
            Self::RWA2 => write!(f, "RWA2"),
            Self::RWA4 => write!(f, "RWA4"),
            Self::RWA6 => write!(f, "RWA6"),
            Self::RWA8 => write!(f, "RWA8"),
            Self::RWAA => write!(f, "RWAA"),
            Self::RWAC => write!(f, "RWAC"),
            Self::RWAE => write!(f, "RWAE"),
            Self::RWB0 => write!(f, "RWB0"),
            Self::RWB2 => write!(f, "RWB2"),
            Self::RWB4 => write!(f, "RWB4"),
            Self::RWB6 => write!(f, "RWB6"),
            Self::RWB8 => write!(f, "RWB8"),
            Self::RWBA => write!(f, "RWBA"),
            Self::RWBC => write!(f, "RWBC"),
            Self::RWBE => write!(f, "RWBE"),
            Self::RWC0 => write!(f, "RWC0"),
            Self::RWC2 => write!(f, "RWC2"),
            Self::RWC4 => write!(f, "RWC4"),
            Self::RWC6 => write!(f, "RWC6"),
            Self::RWC8 => write!(f, "RWC8"),
            Self::RWCA => write!(f, "RWCA"),
            Self::RWCC => write!(f, "RWCC"),
            Self::RWCE => write!(f, "RWCE"),
            Self::RWD0 => write!(f, "RWD0"),
            Self::RWD2 => write!(f, "RWD2"),
            Self::RWD4 => write!(f, "RWD4"),
            Self::RWD6 => write!(f, "RWD6"),
            Self::RWD8 => write!(f, "RWD8"),
            Self::RWDA => write!(f, "RWDA"),
            Self::RWDC => write!(f, "RWDC"),
            Self::RWDE => write!(f, "RWDE"),
            Self::RWE0 => write!(f, "RWE0"),
            Self::RWE2 => write!(f, "RWE2"),
            Self::RWE4 => write!(f, "RWE4"),
            Self::RWE6 => write!(f, "RWE6"),
            Self::RWE8 => write!(f, "RWE8"),
            Self::RWEA => write!(f, "RWEA"),
            Self::RWEC => write!(f, "RWEC"),
            Self::RWEE => write!(f, "RWEE"),
            Self::RWF0 => write!(f, "RWF0"),
            Self::RWF2 => write!(f, "RWF2"),
            Self::RWF4 => write!(f, "RWF4"),
            Self::RWF6 => write!(f, "RWF6"),
            Self::RWF8 => write!(f, "RWF8"),
            Self::RWFA => write!(f, "RWFA"),
            Self::RWFC => write!(f, "RWFC"),
            Self::RWFE => write!(f, "RWFE"),
            Self::RW100 => write!(f, "RW100"),
            Self::RW102 => write!(f, "RW102"),
            Self::RW104 => write!(f, "RW104"),
            Self::RW106 => write!(f, "RW106"),
            Self::RW108 => write!(f, "RW108"),
            Self::RW10A => write!(f, "RW10A"),
            Self::RW10C => write!(f, "RW10C"),
            Self::RW10E => write!(f, "RW10E"),
            Self::RW110 => write!(f, "RW110"),
            Self::RW112 => write!(f, "RW112"),
            Self::RW114 => write!(f, "RW114"),
            Self::RW116 => write!(f, "RW116"),
            Self::RW118 => write!(f, "RW118"),
            Self::RW11A => write!(f, "RW11A"),
            Self::RW11C => write!(f, "RW11C"),
            Self::RW11E => write!(f, "RW11E"),
            Self::RW120 => write!(f, "RW120"),
            Self::RW122 => write!(f, "RW122"),
            Self::RW124 => write!(f, "RW124"),
            Self::RW126 => write!(f, "RW126"),
            Self::RW128 => write!(f, "RW128"),
            Self::RW12A => write!(f, "RW12A"),
            Self::RW12C => write!(f, "RW12C"),
            Self::RW12E => write!(f, "RW12E"),
            Self::RW130 => write!(f, "RW130"),
            Self::RW132 => write!(f, "RW132"),
            Self::RW134 => write!(f, "RW134"),
            Self::RW136 => write!(f, "RW136"),
            Self::RW138 => write!(f, "RW138"),
            Self::RW13A => write!(f, "RW13A"),
            Self::RW13C => write!(f, "RW13C"),
            Self::RW13E => write!(f, "RW13E"),
            Self::RW140 => write!(f, "RW140"),
            Self::RW142 => write!(f, "RW142"),
            Self::RW144 => write!(f, "RW144"),
            Self::RW146 => write!(f, "RW146"),
            Self::RW148 => write!(f, "RW148"),
            Self::RW14A => write!(f, "RW14A"),
            Self::RW14C => write!(f, "RW14C"),
            Self::RW14E => write!(f, "RW14E"),
            Self::RW150 => write!(f, "RW150"),
            Self::RW152 => write!(f, "RW152"),
            Self::RW154 => write!(f, "RW154"),
            Self::RW156 => write!(f, "RW156"),
            Self::RW158 => write!(f, "RW158"),
            Self::RW15A => write!(f, "RW15A"),
            Self::RW15C => write!(f, "RW15C"),
            Self::RW15E => write!(f, "RW15E"),
            Self::RW160 => write!(f, "RW160"),
            Self::RW162 => write!(f, "RW162"),
            Self::RW164 => write!(f, "RW164"),
            Self::RW166 => write!(f, "RW166"),
            Self::RW168 => write!(f, "RW168"),
            Self::RW16A => write!(f, "RW16A"),
            Self::RW16C => write!(f, "RW16C"),
            Self::RW16E => write!(f, "RW16E"),
            Self::RW170 => write!(f, "RW170"),
            Self::RW172 => write!(f, "RW172"),
            Self::RW174 => write!(f, "RW174"),
            Self::RW176 => write!(f, "RW176"),
            Self::RW178 => write!(f, "RW178"),
            Self::RW17A => write!(f, "RW17A"),
            Self::RW17C => write!(f, "RW17C"),
            Self::RW17E => write!(f, "RW17E"),
            Self::RW180 => write!(f, "RW180"),
            Self::RW182 => write!(f, "RW182"),
            Self::RW184 => write!(f, "RW184"),
            Self::RW186 => write!(f, "RW186"),
            Self::RW188 => write!(f, "RW188"),
            Self::RW18A => write!(f, "RW18A"),
            Self::RW18C => write!(f, "RW18C"),
            Self::RW18E => write!(f, "RW18E"),
            Self::RW190 => write!(f, "RW190"),
            Self::RW192 => write!(f, "RW192"),
            Self::RW194 => write!(f, "RW194"),
            Self::RW196 => write!(f, "RW196"),
            Self::RW198 => write!(f, "RW198"),
            Self::RW19A => write!(f, "RW19A"),
            Self::RW19C => write!(f, "RW19C"),
            Self::RW19E => write!(f, "RW19E"),
            Self::RW1A0 => write!(f, "RW1A0"),
            Self::RW1A2 => write!(f, "RW1A2"),
            Self::RW1A4 => write!(f, "RW1A4"),
            Self::RW1A6 => write!(f, "RW1A6"),
            Self::RW1A8 => write!(f, "RW1A8"),
            Self::RW1AA => write!(f, "RW1AA"),
            Self::RW1AC => write!(f, "RW1AC"),
            Self::RW1AE => write!(f, "RW1AE"),
            Self::RW1B0 => write!(f, "RW1B0"),
            Self::RW1B2 => write!(f, "RW1B2"),
            Self::RW1B4 => write!(f, "RW1B4"),
            Self::RW1B6 => write!(f, "RW1B6"),
            Self::RW1B8 => write!(f, "RW1B8"),
            Self::RW1BA => write!(f, "RW1BA"),
            Self::RW1BC => write!(f, "RW1BC"),
            Self::RW1BE => write!(f, "RW1BE"),
            Self::RW1C0 => write!(f, "RW1C0"),
            Self::RW1C2 => write!(f, "RW1C2"),
            Self::RW1C4 => write!(f, "RW1C4"),
            Self::RW1C6 => write!(f, "RW1C6"),
            Self::RW1C8 => write!(f, "RW1C8"),
            Self::RW1CA => write!(f, "RW1CA"),
            Self::RW1CC => write!(f, "RW1CC"),
            Self::RW1CE => write!(f, "RW1CE"),
            Self::RW1D0 => write!(f, "RW1D0"),
            Self::RW1D2 => write!(f, "RW1D2"),
            Self::RW1D4 => write!(f, "RW1D4"),
            Self::RW1D6 => write!(f, "RW1D6"),
            Self::RW1D8 => write!(f, "RW1D8"),
            Self::RW1DA => write!(f, "RW1DA"),
            Self::RW1DC => write!(f, "RW1DC"),
            Self::RW1DE => write!(f, "RW1DE"),
            Self::RW1E0 => write!(f, "RW1E0"),
            Self::RW1E2 => write!(f, "RW1E2"),
            Self::RW1E4 => write!(f, "RW1E4"),
            Self::RW1E6 => write!(f, "RW1E6"),
            Self::RW1E8 => write!(f, "RW1E8"),
            Self::RW1EA => write!(f, "RW1EA"),
            Self::RW1EC => write!(f, "RW1EC"),
            Self::RW1EE => write!(f, "RW1EE"),
            Self::RW1F0 => write!(f, "RW1F0"),
            Self::RW1F2 => write!(f, "RW1F2"),
            Self::RW1F4 => write!(f, "RW1F4"),
            Self::RW1F6 => write!(f, "RW1F6"),
            Self::RW1F8 => write!(f, "RW1F8"),
            Self::RW1FA => write!(f, "RW1FA"),
            Self::RW1FC => write!(f, "RW1FC"),
            Self::RW1FE => write!(f, "RW1FE"),
            Self::RL1C => write!(f, "RL1C"),
            Self::RL20 => write!(f, "RL20"),
            Self::RL24 => write!(f, "RL24"),
            Self::RL28 => write!(f, "RL28"),
            Self::RL2C => write!(f, "RL2C"),
            Self::RL30 => write!(f, "RL30"),
            Self::RL34 => write!(f, "RL34"),
            Self::RL38 => write!(f, "RL38"),
            Self::RL3C => write!(f, "RL3C"),
            Self::RL40 => write!(f, "RL40"),
            Self::RL44 => write!(f, "RL44"),
            Self::RL48 => write!(f, "RL48"),
            Self::RL4C => write!(f, "RL4C"),
            Self::RL50 => write!(f, "RL50"),
            Self::RL54 => write!(f, "RL54"),
            Self::RL58 => write!(f, "RL58"),
            Self::RL5C => write!(f, "RL5C"),
            Self::RL60 => write!(f, "RL60"),
            Self::RL64 => write!(f, "RL64"),
            Self::RL68 => write!(f, "RL68"),
            Self::RL6C => write!(f, "RL6C"),
            Self::RL70 => write!(f, "RL70"),
            Self::RL74 => write!(f, "RL74"),
            Self::RL78 => write!(f, "RL78"),
            Self::RL7C => write!(f, "RL7C"),
            Self::RL80 => write!(f, "RL80"),
            Self::RL84 => write!(f, "RL84"),
            Self::RL88 => write!(f, "RL88"),
            Self::RL8C => write!(f, "RL8C"),
            Self::RL90 => write!(f, "RL90"),
            Self::RL94 => write!(f, "RL94"),
            Self::RL98 => write!(f, "RL98"),
            Self::RL9C => write!(f, "RL9C"),
            Self::RLA0 => write!(f, "RLA0"),
            Self::RLA4 => write!(f, "RLA4"),
            Self::RLA8 => write!(f, "RLA8"),
            Self::RLAC => write!(f, "RLAC"),
            Self::RLB0 => write!(f, "RLB0"),
            Self::RLB4 => write!(f, "RLB4"),
            Self::RLB8 => write!(f, "RLB8"),
            Self::RLBC => write!(f, "RLBC"),
            Self::RLC0 => write!(f, "RLC0"),
            Self::RLC4 => write!(f, "RLC4"),
            Self::RLC8 => write!(f, "RLC8"),
            Self::RLCC => write!(f, "RLCC"),
            Self::RLD0 => write!(f, "RLD0"),
            Self::RLD4 => write!(f, "RLD4"),
            Self::RLD8 => write!(f, "RLD8"),
            Self::RLDC => write!(f, "RLDC"),
            Self::RLE0 => write!(f, "RLE0"),
            Self::RLE4 => write!(f, "RLE4"),
            Self::RLE8 => write!(f, "RLE8"),
            Self::RLEC => write!(f, "RLEC"),
            Self::RLF0 => write!(f, "RLF0"),
            Self::RLF4 => write!(f, "RLF4"),
            Self::RLF8 => write!(f, "RLF8"),
            Self::RLFC => write!(f, "RLFC"),
            Self::RL100 => write!(f, "RL100"),
            Self::RL104 => write!(f, "RL104"),
            Self::RL108 => write!(f, "RL108"),
            Self::RL10C => write!(f, "RL10C"),
            Self::RL110 => write!(f, "RL110"),
            Self::RL114 => write!(f, "RL114"),
            Self::RL118 => write!(f, "RL118"),
            Self::RL11C => write!(f, "RL11C"),
            Self::RL120 => write!(f, "RL120"),
            Self::RL124 => write!(f, "RL124"),
            Self::RL128 => write!(f, "RL128"),
            Self::RL12C => write!(f, "RL12C"),
            Self::RL130 => write!(f, "RL130"),
            Self::RL134 => write!(f, "RL134"),
            Self::RL138 => write!(f, "RL138"),
            Self::RL13C => write!(f, "RL13C"),
            Self::RL140 => write!(f, "RL140"),
            Self::RL144 => write!(f, "RL144"),
            Self::RL148 => write!(f, "RL148"),
            Self::RL14C => write!(f, "RL14C"),
            Self::RL150 => write!(f, "RL150"),
            Self::RL154 => write!(f, "RL154"),
            Self::RL158 => write!(f, "RL158"),
            Self::RL15C => write!(f, "RL15C"),
            Self::RL160 => write!(f, "RL160"),
            Self::RL164 => write!(f, "RL164"),
            Self::RL168 => write!(f, "RL168"),
            Self::RL16C => write!(f, "RL16C"),
            Self::RL170 => write!(f, "RL170"),
            Self::RL174 => write!(f, "RL174"),
            Self::RL178 => write!(f, "RL178"),
            Self::RL17C => write!(f, "RL17C"),
            Self::RL180 => write!(f, "RL180"),
            Self::RL184 => write!(f, "RL184"),
            Self::RL188 => write!(f, "RL188"),
            Self::RL18C => write!(f, "RL18C"),
            Self::RL190 => write!(f, "RL190"),
            Self::RL194 => write!(f, "RL194"),
            Self::RL198 => write!(f, "RL198"),
            Self::RL19C => write!(f, "RL19C"),
            Self::RL1A0 => write!(f, "RL1A0"),
            Self::RL1A4 => write!(f, "RL1A4"),
            Self::RL1A8 => write!(f, "RL1A8"),
            Self::RL1AC => write!(f, "RL1AC"),
            Self::RL1B0 => write!(f, "RL1B0"),
            Self::RL1B4 => write!(f, "RL1B4"),
            Self::RL1B8 => write!(f, "RL1B8"),
            Self::RL1BC => write!(f, "RL1BC"),
            Self::RL1C0 => write!(f, "RL1C0"),
            Self::RL1C4 => write!(f, "RL1C4"),
            Self::RL1C8 => write!(f, "RL1C8"),
            Self::RL1CC => write!(f, "RL1CC"),
            Self::RL1D0 => write!(f, "RL1D0"),
            Self::RL1D4 => write!(f, "RL1D4"),
            Self::RL1D8 => write!(f, "RL1D8"),
            Self::RL1DC => write!(f, "RL1DC"),
            Self::RL1E0 => write!(f, "RL1E0"),
            Self::RL1E4 => write!(f, "RL1E4"),
            Self::RL1E8 => write!(f, "RL1E8"),
            Self::RL1EC => write!(f, "RL1EC"),
            Self::RL1F0 => write!(f, "RL1F0"),
            Self::RL1F4 => write!(f, "RL1F4"),
            Self::RL1F8 => write!(f, "RL1F8"),
            Self::RL1FC => write!(f, "RL1FC"),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum DisplayElement {
    Literal(&'static str),
    Register(Register),
    Number(bool, i64),
}
impl core::fmt::Display for DisplayElement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(lit) => lit.fmt(f),
            Self::Register(reg) => reg.fmt(f),
            Self::Number(hex, value) => match (*hex, value.is_negative()) {
                (true, true) => write!(f, "-0x{:x}", value.abs()),
                (true, false) => write!(f, "0x{:x}", value),
                (false, _) => value.fmt(f),
            },
        }
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:576:1"]
#[derive(Clone, Debug)]
struct instructionVar0 {
    wreg: Tablewreg,
}
impl instructionVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("BR"),
            DisplayElement::Literal(" [ "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" ]")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 227i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:580:1"]
#[derive(Clone, Debug)]
struct instructionVar1 {
    wreg: Tablewreg,
}
impl instructionVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("CLR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 1i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:588:1"]
#[derive(Clone, Debug)]
struct instructionVar2 {
    breg: Tablebreg,
}
impl instructionVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("CLRB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 17i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:596:1"]
#[derive(Clone, Debug)]
struct instructionVar3 {}
impl instructionVar3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("CLRC")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 248i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:600:1"]
#[derive(Clone, Debug)]
struct instructionVar4 {}
impl instructionVar4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("CLRVT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 252i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:629:1"]
#[derive(Clone, Debug)]
struct instructionVar5 {
    wreg: Tablewreg,
}
impl instructionVar5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("DEC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 5i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:635:1"]
#[derive(Clone, Debug)]
struct instructionVar6 {
    breg: Tablebreg,
}
impl instructionVar6 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("DECB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 21i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:641:1"]
#[derive(Clone, Debug)]
struct instructionVar7 {}
impl instructionVar7 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("DI")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 250i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:689:1"]
#[derive(Clone, Debug)]
struct instructionVar8 {
    breg: Tablebreg,
    jmpdest: Tablejmpdest,
}
impl instructionVar8 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("DJNZ"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.jmpdest.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 224i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let jmpdest = if let Some((len, table)) = Tablejmpdest::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, jmpdest }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:695:1"]
#[derive(Clone, Debug)]
struct instructionVar9 {
    wreg: Tablewreg,
    jmpdest: Tablejmpdest,
}
impl instructionVar9 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("DJNZW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.jmpdest.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 225i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let jmpdest = if let Some((len, table)) = Tablejmpdest::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg, jmpdest }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:703:1"]
#[derive(Clone, Debug)]
struct instructionVar10 {}
impl instructionVar10 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("DPTS")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 236i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:709:1"]
#[derive(Clone, Debug)]
struct instructionVar11 {}
impl instructionVar11 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("EI")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 251i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:714:1"]
#[derive(Clone, Debug)]
struct instructionVar12 {}
impl instructionVar12 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("EPTS")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 237i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:726:1"]
#[derive(Clone, Debug)]
struct instructionVar13 {
    wreg: Tablewreg,
}
impl instructionVar13 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("EXTB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 22i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:732:1"]
#[derive(Clone, Debug)]
struct instructionVar14 {
    wreg: Tablewreg,
}
impl instructionVar14 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("INC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 7i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:738:1"]
#[derive(Clone, Debug)]
struct instructionVar15 {
    breg: Tablebreg,
}
impl instructionVar15 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("INCB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 23i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:745:1"]
#[derive(Clone, Debug)]
struct instructionVar16 {
    immed8: Tableimmed8,
}
impl instructionVar16 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("IDLPD"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 246i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:788:1"]
#[derive(Clone, Debug)]
struct instructionVar17 {
    jmpdest16: Tablejmpdest16,
}
impl instructionVar17 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("LCALL"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.jmpdest16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 239i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let jmpdest16 = if let Some((len, table)) = Tablejmpdest16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { jmpdest16 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:810:1"]
#[derive(Clone, Debug)]
struct instructionVar18 {
    jmpdest16: Tablejmpdest16,
}
impl instructionVar18 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("LJMP"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.jmpdest16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 231i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let jmpdest16 = if let Some((len, table)) = Tablejmpdest16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { jmpdest16 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:878:1"]
#[derive(Clone, Debug)]
struct instructionVar19 {
    wreg: Tablewreg,
}
impl instructionVar19 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("NEG"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 3i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:885:1"]
#[derive(Clone, Debug)]
struct instructionVar20 {
    breg: Tablebreg,
}
impl instructionVar20 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("NEGB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 19i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:892:1"]
#[derive(Clone, Debug)]
struct instructionVar21 {}
impl instructionVar21 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NOP")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 253i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:905:1"]
#[derive(Clone, Debug)]
struct instructionVar22 {
    wreg: Tablewreg,
}
impl instructionVar22 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("NOT"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:910:1"]
#[derive(Clone, Debug)]
struct instructionVar23 {
    breg: Tablebreg,
}
impl instructionVar23 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("NOTB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 18i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:938:1"]
#[derive(Clone, Debug)]
struct instructionVar24 {}
impl instructionVar24 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("POPA")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 245i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:952:1"]
#[derive(Clone, Debug)]
struct instructionVar25 {}
impl instructionVar25 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("POPF")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 243i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:966:1"]
#[derive(Clone, Debug)]
struct instructionVar26 {}
impl instructionVar26 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("PUSHA")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 244i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:976:1"]
#[derive(Clone, Debug)]
struct instructionVar27 {}
impl instructionVar27 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("PUSHF")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 242i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:982:1"]
#[derive(Clone, Debug)]
struct instructionVar28 {}
impl instructionVar28 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("RET")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 240i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:988:1"]
#[derive(Clone, Debug)]
struct instructionVar29 {}
impl instructionVar29 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("RST")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 255i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1000:1"]
#[derive(Clone, Debug)]
struct instructionVar30 {}
impl instructionVar30 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("SETC")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 249i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1219:1"]
#[derive(Clone, Debug)]
struct instructionVar31 {}
impl instructionVar31 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("SKIP")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1285:1"]
#[derive(Clone, Debug)]
struct instructionVar32 {}
impl instructionVar32 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("TRAP")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 247i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1302:1"]
#[derive(Clone, Debug)]
struct instructionVar33 {
    waop16: Tablewaop16,
    wreg: Tablewreg,
}
impl instructionVar33 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("XCH"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.waop16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 4i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let waop16 = if let Some((len, table)) = Tablewaop16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { waop16, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1331:1"]
#[derive(Clone, Debug)]
struct instructionVar34 {
    baop8: Tablebaop8,
    breg: Tablebreg,
}
impl instructionVar34 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("XCHB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.baop8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 20i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let baop8 = if let Some((len, table)) =
            Tablebaop8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { baop8, breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:502:1"]
#[derive(Clone, Debug)]
struct instructionVar35 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar35 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ADD"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 25i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:657:1"]
#[derive(Clone, Debug)]
struct instructionVar36 {
    oper8: Tableoper8,
    wreg: Tablewreg,
}
impl instructionVar36 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("DIVB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 254i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 39i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:831:1"]
#[derive(Clone, Debug)]
struct instructionVar37 {
    oper8: Tableoper8,
    wreg: Tablewreg,
}
impl instructionVar37 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MULB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 254i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 31i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:839:1"]
#[derive(Clone, Debug)]
struct instructionVar38 {
    oper8: Tableoper8,
    breg: Tablebreg,
    wreg: Tablewreg,
}
impl instructionVar38 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MULB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 254i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 23i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1004:1"]
#[derive(Clone, Debug)]
struct instructionVar39 {
    immed8: Tableimmed8,
    wreg: Tablewreg,
}
impl instructionVar39 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("SHL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), DisplayElement::Literal("#")];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 9i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c60 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() != 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c60(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1015:1"]
#[derive(Clone, Debug)]
struct instructionVar40 {
    breg: Tablebreg,
    wreg: Tablewreg,
}
impl instructionVar40 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("SHL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 9i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c58 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() == 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c58(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1070:1"]
#[derive(Clone, Debug)]
struct instructionVar41 {
    immed8: Tableimmed8,
    wreg: Tablewreg,
}
impl instructionVar41 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("SHR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), DisplayElement::Literal("#")];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 8i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c60 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() != 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c60(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1083:1"]
#[derive(Clone, Debug)]
struct instructionVar42 {
    breg: Tablebreg,
    wreg: Tablewreg,
}
impl instructionVar42 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("SHR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 8i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c58 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() == 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c58(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1095:1"]
#[derive(Clone, Debug)]
struct instructionVar43 {
    immed8: Tableimmed8,
    wreg: Tablewreg,
}
impl instructionVar43 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SHRA"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), DisplayElement::Literal("#")];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 10i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c60 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() != 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c60(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1107:1"]
#[derive(Clone, Debug)]
struct instructionVar44 {
    breg: Tablebreg,
    wreg: Tablewreg,
}
impl instructionVar44 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SHRA"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 10i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c58 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() == 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c58(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1308:1"]
#[derive(Clone, Debug)]
struct instructionVar45 {
    iwreg: Tableiwreg,
    immed8: Tableimmed8,
    wreg: Tablewreg,
}
impl instructionVar45 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("XCH"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal(", TABLE[")];
        display.extend_from_slice(&extend);
        self.iwreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 11i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaddbit8().disassembly() != 0i64 {
            return None;
        }
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                iwreg,
                immed8,
                wreg,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1318:1"]
#[derive(Clone, Debug)]
struct instructionVar46 {
    iwreg: Tableiwreg,
    immed16: Tableimmed16,
    wreg: Tablewreg,
}
impl instructionVar46 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("XCH"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.immed16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal(", TABLE[")];
        display.extend_from_slice(&extend);
        self.iwreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 11i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaddbit8().disassembly() != 1i64 {
            return None;
        }
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let immed16 = if let Some((len, table)) = Tableimmed16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                iwreg,
                immed16,
                wreg,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1337:1"]
#[derive(Clone, Debug)]
struct instructionVar47 {
    iwreg: Tableiwreg,
    immed8: Tableimmed8,
    breg: Tablebreg,
}
impl instructionVar47 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("XCHB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal(", TABLE[")];
        display.extend_from_slice(&extend);
        self.iwreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 27i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaddbit8().disassembly() != 0i64 {
            return None;
        }
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                iwreg,
                immed8,
                breg,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1347:1"]
#[derive(Clone, Debug)]
struct instructionVar48 {
    iwreg: Tableiwreg,
    immed16: Tableimmed16,
    breg: Tablebreg,
}
impl instructionVar48 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("XCHB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.immed16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal(", TABLE[")];
        display.extend_from_slice(&extend);
        self.iwreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 27i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaddbit8().disassembly() != 1i64 {
            return None;
        }
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let immed16 = if let Some((len, table)) = Tableimmed16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                iwreg,
                immed16,
                breg,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:568:1"]
#[derive(Clone, Debug)]
struct instructionVar49 {
    lreg: TokenField_lreg,
    wreg: Tablewreg,
}
impl instructionVar49 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("BMOV"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 193i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:572:1"]
#[derive(Clone, Debug)]
struct instructionVar50 {
    lreg: TokenField_lreg,
    wreg: Tablewreg,
}
impl instructionVar50 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("BMOVI"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 205i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:720:1"]
#[derive(Clone, Debug)]
struct instructionVar51 {
    lreg: TokenField_lreg,
}
impl instructionVar51 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("EXT"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 6i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:894:1"]
#[derive(Clone, Debug)]
struct instructionVar52 {
    lreg: TokenField_lreg,
    breg: Tablebreg,
}
impl instructionVar52 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("NORML"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 15i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1273:1"]
#[derive(Clone, Debug)]
struct instructionVar53 {
    dwreg: TokenField_dwreg,
    wreg: Tablewreg,
    immed8: Tableimmed8,
}
impl instructionVar53 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("TIJMP"),
            DisplayElement::Literal(" "),
            self.dwreg.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("]"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 226i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dwreg = token_parser.TokenFielddwreg();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                wreg,
                immed8,
                dwreg,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:509:1"]
#[derive(Clone, Debug)]
struct instructionVar54 {
    dwreg: TokenField_dwreg,
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar54 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ADD"),
            DisplayElement::Literal(" "),
            self.dwreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 17i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dwreg = token_parser.TokenFielddwreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                oper16,
                wreg,
                dwreg,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:516:1"]
#[derive(Clone, Debug)]
struct instructionVar55 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl instructionVar55 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ADDB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 29i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:522:1"]
#[derive(Clone, Debug)]
struct instructionVar56 {
    dbreg: TokenField_dbreg,
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl instructionVar56 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ADDB"),
            DisplayElement::Literal(" "),
            self.dbreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 21i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dbreg = token_parser.TokenFielddbreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg, dbreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:528:1"]
#[derive(Clone, Debug)]
struct instructionVar57 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar57 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ADDC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 41i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:537:1"]
#[derive(Clone, Debug)]
struct instructionVar58 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl instructionVar58 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ADDCB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 45i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:547:1"]
#[derive(Clone, Debug)]
struct instructionVar59 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar59 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("AND"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 24i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:553:1"]
#[derive(Clone, Debug)]
struct instructionVar60 {
    dwreg: TokenField_dwreg,
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar60 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("AND"),
            DisplayElement::Literal(" "),
            self.dwreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 16i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dwreg = token_parser.TokenFielddwreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                oper16,
                wreg,
                dwreg,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:558:1"]
#[derive(Clone, Debug)]
struct instructionVar61 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl instructionVar61 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ANDB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 28i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:563:1"]
#[derive(Clone, Debug)]
struct instructionVar62 {
    dbreg: TokenField_dbreg,
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl instructionVar62 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ANDB"),
            DisplayElement::Literal(" "),
            self.dbreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 20i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dbreg = token_parser.TokenFielddbreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg, dbreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:604:1"]
#[derive(Clone, Debug)]
struct instructionVar63 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar63 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("CMP"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 34i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:610:1"]
#[derive(Clone, Debug)]
struct instructionVar64 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl instructionVar64 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("CMPB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 38i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:645:1"]
#[derive(Clone, Debug)]
struct instructionVar65 {
    lreg: TokenField_lreg,
    oper16: Tableoper16,
}
impl instructionVar65 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("DIV"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 254i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 35i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:815:1"]
#[derive(Clone, Debug)]
struct instructionVar66 {
    lreg: TokenField_lreg,
    oper16: Tableoper16,
}
impl instructionVar66 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("MUL"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 254i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 27i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:823:1"]
#[derive(Clone, Debug)]
struct instructionVar67 {
    lreg: TokenField_lreg,
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar67 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("MUL"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 254i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 19i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1026:1"]
#[derive(Clone, Debug)]
struct instructionVar68 {
    dbreg: TokenField_dbreg,
    immed8: Tableimmed8,
}
impl instructionVar68 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("SHLB"),
            DisplayElement::Literal(" "),
            self.dbreg.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 25i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c60 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() != 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c60(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dbreg = token_parser.TokenFielddbreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, dbreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1037:1"]
#[derive(Clone, Debug)]
struct instructionVar69 {
    dbreg: TokenField_dbreg,
    breg: Tablebreg,
}
impl instructionVar69 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("SHLB"),
            DisplayElement::Literal(" "),
            self.dbreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 25i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c58 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() == 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c58(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dbreg = token_parser.TokenFielddbreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, dbreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1048:1"]
#[derive(Clone, Debug)]
struct instructionVar70 {
    lreg: TokenField_lreg,
    immed8: Tableimmed8,
}
impl instructionVar70 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("SHLL"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 13i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c60 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() != 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c60(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1059:1"]
#[derive(Clone, Debug)]
struct instructionVar71 {
    lreg: TokenField_lreg,
    breg: Tablebreg,
}
impl instructionVar71 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("SHLL"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 13i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c58 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() == 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c58(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1119:1"]
#[derive(Clone, Debug)]
struct instructionVar72 {
    dbreg: TokenField_dbreg,
    immed8: Tableimmed8,
}
impl instructionVar72 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("SHRAB"),
            DisplayElement::Literal(" "),
            self.dbreg.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 26i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c60 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() != 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c60(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dbreg = token_parser.TokenFielddbreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, dbreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1131:1"]
#[derive(Clone, Debug)]
struct instructionVar73 {
    dbreg: TokenField_dbreg,
    breg: Tablebreg,
}
impl instructionVar73 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("SHRAB"),
            DisplayElement::Literal(" "),
            self.dbreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 26i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c58 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() == 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c58(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dbreg = token_parser.TokenFielddbreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, dbreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1143:1"]
#[derive(Clone, Debug)]
struct instructionVar74 {
    lreg: TokenField_lreg,
    immed8: Tableimmed8,
}
impl instructionVar74 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("SHRAL"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 14i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c60 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() != 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c60(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1155:1"]
#[derive(Clone, Debug)]
struct instructionVar75 {
    lreg: TokenField_lreg,
    breg: Tablebreg,
}
impl instructionVar75 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("SHRAL"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 14i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c58 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() == 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c58(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1167:1"]
#[derive(Clone, Debug)]
struct instructionVar76 {
    dbreg: TokenField_dbreg,
    immed8: Tableimmed8,
}
impl instructionVar76 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("SHRB"),
            DisplayElement::Literal(" "),
            self.dbreg.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 24i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c60 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() != 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c60(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dbreg = token_parser.TokenFielddbreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, dbreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1179:1"]
#[derive(Clone, Debug)]
struct instructionVar77 {
    dbreg: TokenField_dbreg,
    breg: Tablebreg,
}
impl instructionVar77 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("SHRB"),
            DisplayElement::Literal(" "),
            self.dbreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 24i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c58 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() == 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c58(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dbreg = token_parser.TokenFielddbreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, dbreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1191:1"]
#[derive(Clone, Debug)]
struct instructionVar78 {
    lreg: TokenField_lreg,
    immed8: Tableimmed8,
}
impl instructionVar78 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("SHRL"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 12i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c60 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() != 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c60(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1203:1"]
#[derive(Clone, Debug)]
struct instructionVar79 {
    lreg: TokenField_lreg,
    breg: Tablebreg,
}
impl instructionVar79 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("SHRL"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 12i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let mut sub_pattern_c58 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldhighb().disassembly() == 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c58(tokens_current, &mut context_instance)?;
        block_1_len = block_1_len.max(sub_len);
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:617:1"]
#[derive(Clone, Debug)]
struct instructionVar80 {
    dlreg: TokenField_dlreg,
    lreg: TokenField_lreg,
}
impl instructionVar80 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("CMPL"),
            DisplayElement::Literal(" "),
            self.dlreg.display(),
            DisplayElement::Literal(", "),
            self.lreg.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop8().disassembly() != 197i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dlreg = token_parser.TokenFielddlreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { lreg, dlreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:669:1"]
#[derive(Clone, Debug)]
struct instructionVar81 {
    lreg: TokenField_lreg,
    oper16: Tableoper16,
}
impl instructionVar81 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("DIVU"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 35i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:679:1"]
#[derive(Clone, Debug)]
struct instructionVar82 {
    oper8: Tableoper8,
    wreg: Tablewreg,
}
impl instructionVar82 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("DIVUB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 39i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:794:1"]
#[derive(Clone, Debug)]
struct instructionVar83 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar83 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("LD"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 40i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:798:1"]
#[derive(Clone, Debug)]
struct instructionVar84 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl instructionVar84 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("LDB"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 44i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:802:1"]
#[derive(Clone, Debug)]
struct instructionVar85 {
    oper8: Tableoper8,
    wreg: Tablewreg,
}
impl instructionVar85 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("LDBSE"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 47i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:806:1"]
#[derive(Clone, Debug)]
struct instructionVar86 {
    oper8: Tableoper8,
    wreg: Tablewreg,
}
impl instructionVar86 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("LDBZE"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 43i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:863:1"]
#[derive(Clone, Debug)]
struct instructionVar87 {
    oper8: Tableoper8,
    wreg: Tablewreg,
}
impl instructionVar87 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MULUB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 31i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:871:1"]
#[derive(Clone, Debug)]
struct instructionVar88 {
    oper8: Tableoper8,
    breg: Tablebreg,
    wreg: Tablewreg,
}
impl instructionVar88 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MULUB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 23i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:915:1"]
#[derive(Clone, Debug)]
struct instructionVar89 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar89 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("OR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 32i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:923:1"]
#[derive(Clone, Debug)]
struct instructionVar90 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl instructionVar90 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ORB"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 36i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:931:1"]
#[derive(Clone, Debug)]
struct instructionVar91 {
    oper16: Tableoper16,
}
impl instructionVar91 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("POP"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 51i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:960:1"]
#[derive(Clone, Debug)]
struct instructionVar92 {
    oper16: Tableoper16,
}
impl instructionVar92 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("PUSH"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 50i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1224:1"]
#[derive(Clone, Debug)]
struct instructionVar93 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar93 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ST"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 48i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1228:1"]
#[derive(Clone, Debug)]
struct instructionVar94 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl instructionVar94 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("STB"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 49i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1233:1"]
#[derive(Clone, Debug)]
struct instructionVar95 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar95 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("SUB"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 26i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1246:1"]
#[derive(Clone, Debug)]
struct instructionVar96 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl instructionVar96 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SUBB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 30i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1256:1"]
#[derive(Clone, Debug)]
struct instructionVar97 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar97 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SUBC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 42i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1264:1"]
#[derive(Clone, Debug)]
struct instructionVar98 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl instructionVar98 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SUBCB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 46i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1293:1"]
#[derive(Clone, Debug)]
struct instructionVar99 {
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar99 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("XOR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 33i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1359:1"]
#[derive(Clone, Debug)]
struct instructionVar100 {
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl instructionVar100 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("XORB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 37i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:757:1"]
#[derive(Clone, Debug)]
struct instructionVar101 {
    bitno: TokenField_bitno,
    breg: Tablebreg,
    jmpdest: Tablejmpdest,
}
impl instructionVar101 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JBC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(", "),
            self.bitno.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.jmpdest.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop5().disassembly() != 6i64 {
            return None;
        }
        let bitno = token_parser.TokenFieldbitno();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let jmpdest = if let Some((len, table)) = Tablejmpdest::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                breg,
                jmpdest,
                bitno,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:762:1"]
#[derive(Clone, Debug)]
struct instructionVar102 {
    bitno: TokenField_bitno,
    breg: Tablebreg,
    jmpdest: Tablejmpdest,
}
impl instructionVar102 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JBS"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(", "),
            self.bitno.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.jmpdest.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop5().disassembly() != 7i64 {
            return None;
        }
        let bitno = token_parser.TokenFieldbitno();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let jmpdest = if let Some((len, table)) = Tablejmpdest::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                breg,
                jmpdest,
                bitno,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:784:1"]
#[derive(Clone, Debug)]
struct instructionVar103 {
    cc: Tablecc,
    jmpdest: Tablejmpdest,
}
impl instructionVar103 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("J")];
        display.extend_from_slice(&extend);
        self.cc.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.jmpdest.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop4().disassembly() != 13i64 {
            return None;
        }
        let cc = if let Some((len, table)) =
            Tablecc::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let jmpdest = if let Some((len, table)) = Tablejmpdest::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { cc, jmpdest }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:847:1"]
#[derive(Clone, Debug)]
struct instructionVar104 {
    lreg: TokenField_lreg,
    oper16: Tableoper16,
}
impl instructionVar104 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("MULU"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 27i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:855:1"]
#[derive(Clone, Debug)]
struct instructionVar105 {
    lreg: TokenField_lreg,
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar105 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("MULU"),
            DisplayElement::Literal(" "),
            self.lreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 19i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lreg = token_parser.TokenFieldlreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper16, wreg, lreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1240:1"]
#[derive(Clone, Debug)]
struct instructionVar106 {
    dwreg: TokenField_dwreg,
    oper16: Tableoper16,
    wreg: Tablewreg,
}
impl instructionVar106 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("SUB"),
            DisplayElement::Literal(" "),
            self.dwreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.wreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 18i64 {
            return None;
        }
        let oper16 = if let Some((len, table)) = Tableoper16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let wreg = if let Some((len, table)) =
            Tablewreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dwreg = token_parser.TokenFielddwreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                oper16,
                wreg,
                dwreg,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1251:1"]
#[derive(Clone, Debug)]
struct instructionVar107 {
    dbreg: TokenField_dbreg,
    oper8: Tableoper8,
    breg: Tablebreg,
}
impl instructionVar107 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("SUBB"),
            DisplayElement::Literal(" "),
            self.dbreg.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.breg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.oper8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6().disassembly() != 22i64 {
            return None;
        }
        let oper8 = if let Some((len, table)) =
            Tableoper8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let breg = if let Some((len, table)) =
            Tablebreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dbreg = token_parser.TokenFielddbreg();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oper8, breg, dbreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:994:1"]
#[derive(Clone, Debug)]
struct instructionVar108 {
    jmpdest11: Tablejmpdest11,
}
impl instructionVar108 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SCALL"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.jmpdest11.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop16().disassembly() != 5i64 {
            return None;
        }
        let jmpdest11 = if let Some((len, table)) = Tablejmpdest11::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { jmpdest11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:1215:1"]
#[derive(Clone, Debug)]
struct instructionVar109 {
    jmpdest11: Tablejmpdest11,
}
impl instructionVar109 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SJMP"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.jmpdest11.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop16().disassembly() != 4i64 {
            return None;
        }
        let jmpdest11 = if let Some((len, table)) = Tablejmpdest11::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { jmpdest11 }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(instructionVar0),
    Var1(instructionVar1),
    Var2(instructionVar2),
    Var3(instructionVar3),
    Var4(instructionVar4),
    Var5(instructionVar5),
    Var6(instructionVar6),
    Var7(instructionVar7),
    Var8(instructionVar8),
    Var9(instructionVar9),
    Var10(instructionVar10),
    Var11(instructionVar11),
    Var12(instructionVar12),
    Var13(instructionVar13),
    Var14(instructionVar14),
    Var15(instructionVar15),
    Var16(instructionVar16),
    Var17(instructionVar17),
    Var18(instructionVar18),
    Var19(instructionVar19),
    Var20(instructionVar20),
    Var21(instructionVar21),
    Var22(instructionVar22),
    Var23(instructionVar23),
    Var24(instructionVar24),
    Var25(instructionVar25),
    Var26(instructionVar26),
    Var27(instructionVar27),
    Var28(instructionVar28),
    Var29(instructionVar29),
    Var30(instructionVar30),
    Var31(instructionVar31),
    Var32(instructionVar32),
    Var33(instructionVar33),
    Var34(instructionVar34),
    Var35(instructionVar35),
    Var36(instructionVar36),
    Var37(instructionVar37),
    Var38(instructionVar38),
    Var39(instructionVar39),
    Var40(instructionVar40),
    Var41(instructionVar41),
    Var42(instructionVar42),
    Var43(instructionVar43),
    Var44(instructionVar44),
    Var45(instructionVar45),
    Var46(instructionVar46),
    Var47(instructionVar47),
    Var48(instructionVar48),
    Var49(instructionVar49),
    Var50(instructionVar50),
    Var51(instructionVar51),
    Var52(instructionVar52),
    Var53(instructionVar53),
    Var54(instructionVar54),
    Var55(instructionVar55),
    Var56(instructionVar56),
    Var57(instructionVar57),
    Var58(instructionVar58),
    Var59(instructionVar59),
    Var60(instructionVar60),
    Var61(instructionVar61),
    Var62(instructionVar62),
    Var63(instructionVar63),
    Var64(instructionVar64),
    Var65(instructionVar65),
    Var66(instructionVar66),
    Var67(instructionVar67),
    Var68(instructionVar68),
    Var69(instructionVar69),
    Var70(instructionVar70),
    Var71(instructionVar71),
    Var72(instructionVar72),
    Var73(instructionVar73),
    Var74(instructionVar74),
    Var75(instructionVar75),
    Var76(instructionVar76),
    Var77(instructionVar77),
    Var78(instructionVar78),
    Var79(instructionVar79),
    Var80(instructionVar80),
    Var81(instructionVar81),
    Var82(instructionVar82),
    Var83(instructionVar83),
    Var84(instructionVar84),
    Var85(instructionVar85),
    Var86(instructionVar86),
    Var87(instructionVar87),
    Var88(instructionVar88),
    Var89(instructionVar89),
    Var90(instructionVar90),
    Var91(instructionVar91),
    Var92(instructionVar92),
    Var93(instructionVar93),
    Var94(instructionVar94),
    Var95(instructionVar95),
    Var96(instructionVar96),
    Var97(instructionVar97),
    Var98(instructionVar98),
    Var99(instructionVar99),
    Var100(instructionVar100),
    Var101(instructionVar101),
    Var102(instructionVar102),
    Var103(instructionVar103),
    Var104(instructionVar104),
    Var105(instructionVar105),
    Var106(instructionVar106),
    Var107(instructionVar107),
    Var108(instructionVar108),
    Var109(instructionVar109),
}
impl Tableinstruction {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var2(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var3(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var4(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var5(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var6(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var7(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var8(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var9(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var10(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var11(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var12(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var13(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var14(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var15(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var16(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var17(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var18(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var19(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var20(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var21(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var22(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var23(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var24(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var25(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var26(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var27(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var28(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var29(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var30(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var31(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var32(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var33(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var34(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var35(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var36(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var37(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var38(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var39(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var40(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var41(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var42(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var43(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var44(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var45(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var46(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var47(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var48(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var49(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var50(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var51(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var52(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var53(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var54(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var55(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var56(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var57(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var58(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var59(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var60(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var61(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var62(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var63(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var64(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var65(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var66(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var67(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var68(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var69(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var70(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var71(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var72(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var73(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var74(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var75(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var76(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var77(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var78(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var79(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var80(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var81(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var82(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var83(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var84(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var85(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var86(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var87(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var88(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var89(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var90(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var91(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var92(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var93(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var94(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var95(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var96(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var97(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var98(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var99(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var100(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var101(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var102(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var103(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var104(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var105(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var106(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var107(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var108(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var109(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) = instructionVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar8::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar9::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar10::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar11::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar12::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar13::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar14::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar15::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar16::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var16(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar17::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var17(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar18::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var18(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar19::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var19(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar20::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var20(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar21::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var21(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar22::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var22(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar23::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var23(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar24::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var24(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar25::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var25(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar26::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var26(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar27::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var27(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar28::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var28(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar29::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var29(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar30::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var30(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar31::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var31(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar32::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var32(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar33::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var33(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar34::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var34(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar35::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var35(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar36::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var36(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar37::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var37(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar38::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var38(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar39::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var39(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar40::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var40(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar41::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var41(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar42::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var42(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar43::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var43(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar44::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var44(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar45::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var45(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar46::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var46(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar47::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var47(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar48::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var48(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar49::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var49(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar50::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var50(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar51::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var51(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar52::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var52(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar53::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var53(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar54::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var54(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar55::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var55(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar56::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var56(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar57::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var57(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar58::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var58(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar59::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var59(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar60::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var60(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar61::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var61(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar62::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var62(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar63::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var63(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar64::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var64(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar65::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var65(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar66::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var66(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar67::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var67(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar68::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var68(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar69::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var69(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar70::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var70(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar71::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var71(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar72::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var72(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar73::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var73(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar74::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var74(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar75::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var75(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar76::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var76(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar77::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var77(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar78::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var78(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar79::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var79(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar80::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var80(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar81::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var81(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar82::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var82(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar83::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var83(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar84::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var84(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar85::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var85(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar86::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var86(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar87::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var87(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar88::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var88(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar89::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var89(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar90::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var90(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar91::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var91(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar92::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var92(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar93::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var93(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar94::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var94(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar95::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var95(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar96::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var96(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar97::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var97(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar98::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var98(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar99::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var99(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar100::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var100(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar101::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var101(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar102::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var102(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar103::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var103(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar104::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var104(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar105::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var105(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar106::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var106(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar107::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var107(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar108::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var108(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar109::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var109(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:317:1"]
#[derive(Clone, Debug)]
struct immed8Var0 {
    imm8: TokenField_imm8,
}
impl immed8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.imm8.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[derive(Clone, Debug)]
enum Tableimmed8 {
    Var0(immed8Var0),
}
impl Tableimmed8 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            immed8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:319:1"]
#[derive(Clone, Debug)]
struct simmed8Var0 {
    simm8: TokenField_simm8,
}
impl simmed8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.simm8.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let simm8 = token_parser.TokenFieldsimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[derive(Clone, Debug)]
enum Tablesimmed8 {
    Var0(simmed8Var0),
}
impl Tablesimmed8 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            simmed8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:321:1"]
#[derive(Clone, Debug)]
struct immed16Var0 {
    imm16: TokenField_imm16,
}
impl immed16Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.imm16.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[derive(Clone, Debug)]
enum Tableimmed16 {
    Var0(immed16Var0),
}
impl Tableimmed16 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            immed16Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:323:1"]
#[derive(Clone, Debug)]
struct baop8Var0 {
    baop: TokenField_baop,
}
impl baop8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.baop.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbaop().disassembly() != 0i64 {
            return None;
        }
        let baop = token_parser.TokenFieldbaop();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { baop }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:324:1"]
#[derive(Clone, Debug)]
struct baop8Var1 {
    baop: TokenField_baop,
}
impl baop8Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.baop.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbaop().disassembly() != 1i64 {
            return None;
        }
        let baop = token_parser.TokenFieldbaop();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { baop }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:325:1"]
#[derive(Clone, Debug)]
struct baop8Var2 {
    baop: TokenField_baop,
}
impl baop8Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.baop.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let baop = token_parser.TokenFieldbaop();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { baop }))
    }
}
#[derive(Clone, Debug)]
enum Tablebaop8 {
    Var0(baop8Var0),
    Var1(baop8Var1),
    Var2(baop8Var2),
}
impl Tablebaop8 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var2(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            baop8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            baop8Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            baop8Var2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:327:1"]
#[derive(Clone, Debug)]
struct waop16Var0 {
    waop: TokenField_waop,
}
impl waop16Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.waop.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldwaop().disassembly() != 0i64 {
            return None;
        }
        let waop = token_parser.TokenFieldwaop();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { waop }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:328:1"]
#[derive(Clone, Debug)]
struct waop16Var1 {
    waop: TokenField_waop,
}
impl waop16Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.waop.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let waop = token_parser.TokenFieldwaop();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { waop }))
    }
}
#[derive(Clone, Debug)]
enum Tablewaop16 {
    Var0(waop16Var0),
    Var1(waop16Var1),
}
impl Tablewaop16 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            waop16Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            waop16Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:330:1"]
#[derive(Clone, Debug)]
struct iwregVar0 {
    iwreg7: TokenField_iwreg7,
}
impl iwregVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.iwreg7.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldiwreg7().disassembly() != 0i64 {
            return None;
        }
        let iwreg7 = token_parser.TokenFieldiwreg7();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg7 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:331:1"]
#[derive(Clone, Debug)]
struct iwregVar1 {
    iwreg7: TokenField_iwreg7,
}
impl iwregVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.iwreg7.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let iwreg7 = token_parser.TokenFieldiwreg7();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg7 }))
    }
}
#[derive(Clone, Debug)]
enum Tableiwreg {
    Var0(iwregVar0),
    Var1(iwregVar1),
}
impl Tableiwreg {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            iwregVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            iwregVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:333:1"]
#[derive(Clone, Debug)]
struct bregVar0 {
    breg8: TokenField_breg8,
}
impl bregVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.breg8.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbreg8().disassembly() != 0i64 {
            return None;
        }
        let breg8 = token_parser.TokenFieldbreg8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:334:1"]
#[derive(Clone, Debug)]
struct bregVar1 {
    breg8: TokenField_breg8,
}
impl bregVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.breg8.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbreg8().disassembly() != 1i64 {
            return None;
        }
        let breg8 = token_parser.TokenFieldbreg8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:335:1"]
#[derive(Clone, Debug)]
struct bregVar2 {
    breg8: TokenField_breg8,
}
impl bregVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.breg8.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let breg8 = token_parser.TokenFieldbreg8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { breg8 }))
    }
}
#[derive(Clone, Debug)]
enum Tablebreg {
    Var0(bregVar0),
    Var1(bregVar1),
    Var2(bregVar2),
}
impl Tablebreg {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var2(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            bregVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            bregVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            bregVar2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:337:1"]
#[derive(Clone, Debug)]
struct wregVar0 {
    wreg8: TokenField_wreg8,
}
impl wregVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.wreg8.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldwreg8().disassembly() != 0i64 {
            return None;
        }
        let wreg8 = token_parser.TokenFieldwreg8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:338:1"]
#[derive(Clone, Debug)]
struct wregVar1 {
    wreg8: TokenField_wreg8,
}
impl wregVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.wreg8.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let wreg8 = token_parser.TokenFieldwreg8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { wreg8 }))
    }
}
#[derive(Clone, Debug)]
enum Tablewreg {
    Var0(wregVar0),
    Var1(wregVar1),
}
impl Tablewreg {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            wregVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            wregVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:342:1"]
#[derive(Clone, Debug)]
struct oper8Var0 {
    baop8: Tablebaop8,
}
impl oper8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.baop8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaa().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let baop8 = if let Some((len, table)) =
            Tablebaop8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { baop8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:343:1"]
#[derive(Clone, Debug)]
struct oper8Var1 {
    immed8: Tableimmed8,
}
impl oper8Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("#")];
        display.extend_from_slice(&extend);
        self.immed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaa().disassembly() != 1i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let immed8 = if let Some((len, table)) = Tableimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:344:1"]
#[derive(Clone, Debug)]
struct oper8Var2 {
    iwreg: Tableiwreg,
}
impl oper8Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("[")];
        display.extend_from_slice(&extend);
        self.iwreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaa().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaddbit8().disassembly() != 0i64 {
            return None;
        }
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:348:1"]
#[derive(Clone, Debug)]
struct oper8Var3 {
    iwreg: Tableiwreg,
}
impl oper8Var3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("[")];
        display.extend_from_slice(&extend);
        self.iwreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]+")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaa().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaddbit8().disassembly() != 1i64 {
            return None;
        }
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:353:1"]
#[derive(Clone, Debug)]
struct oper8Var4 {
    iwreg: Tableiwreg,
    simmed8: Tablesimmed8,
}
impl oper8Var4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.simmed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("[")];
        display.extend_from_slice(&extend);
        self.iwreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaa().disassembly() != 3i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaddbit8().disassembly() != 0i64 {
            return None;
        }
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let simmed8 = if let Some((len, table)) = Tablesimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg, simmed8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:358:1"]
#[derive(Clone, Debug)]
struct oper8Var5 {
    iwreg: Tableiwreg,
    immed16: Tableimmed16,
}
impl oper8Var5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.immed16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal(", LOOKUP[")];
        display.extend_from_slice(&extend);
        self.iwreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaa().disassembly() != 3i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaddbit8().disassembly() != 1i64 {
            return None;
        }
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let immed16 = if let Some((len, table)) = Tableimmed16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg, immed16 }))
    }
}
#[derive(Clone, Debug)]
enum Tableoper8 {
    Var0(oper8Var0),
    Var1(oper8Var1),
    Var2(oper8Var2),
    Var3(oper8Var3),
    Var4(oper8Var4),
    Var5(oper8Var5),
}
impl Tableoper8 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var2(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var3(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var4(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var5(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            oper8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            oper8Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            oper8Var2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            oper8Var3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) =
            oper8Var4::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) =
            oper8Var5::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:365:1"]
#[derive(Clone, Debug)]
struct oper16Var0 {
    waop16: Tablewaop16,
}
impl oper16Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.waop16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaa().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let waop16 = if let Some((len, table)) = Tablewaop16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { waop16 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:366:1"]
#[derive(Clone, Debug)]
struct oper16Var1 {
    immed16: Tableimmed16,
}
impl oper16Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("#")];
        display.extend_from_slice(&extend);
        self.immed16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaa().disassembly() != 1i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let immed16 = if let Some((len, table)) = Tableimmed16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { immed16 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:367:1"]
#[derive(Clone, Debug)]
struct oper16Var2 {
    iwreg: Tableiwreg,
}
impl oper16Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("[")];
        display.extend_from_slice(&extend);
        self.iwreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaa().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaddbit8().disassembly() != 0i64 {
            return None;
        }
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:371:1"]
#[derive(Clone, Debug)]
struct oper16Var3 {
    iwreg: Tableiwreg,
}
impl oper16Var3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("[")];
        display.extend_from_slice(&extend);
        self.iwreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]+")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaa().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaddbit8().disassembly() != 1i64 {
            return None;
        }
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:376:1"]
#[derive(Clone, Debug)]
struct oper16Var4 {
    iwreg: Tableiwreg,
    simmed8: Tablesimmed8,
}
impl oper16Var4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.simmed8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("[")];
        display.extend_from_slice(&extend);
        self.iwreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaa().disassembly() != 3i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaddbit8().disassembly() != 0i64 {
            return None;
        }
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let simmed8 = if let Some((len, table)) = Tablesimmed8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg, simmed8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:381:1"]
#[derive(Clone, Debug)]
struct oper16Var5 {
    iwreg: Tableiwreg,
    immed16: Tableimmed16,
}
impl oper16Var5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.immed16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal(", TABLE[")];
        display.extend_from_slice(&extend);
        self.iwreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaa().disassembly() != 3i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaddbit8().disassembly() != 1i64 {
            return None;
        }
        let iwreg = if let Some((len, table)) =
            Tableiwreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let immed16 = if let Some((len, table)) = Tableimmed16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iwreg, immed16 }))
    }
}
#[derive(Clone, Debug)]
enum Tableoper16 {
    Var0(oper16Var0),
    Var1(oper16Var1),
    Var2(oper16Var2),
    Var3(oper16Var3),
    Var4(oper16Var4),
    Var5(oper16Var5),
}
impl Tableoper16 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var2(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var3(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var4(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var5(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            oper16Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            oper16Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            oper16Var2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            oper16Var3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) =
            oper16Var4::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) =
            oper16Var5::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:388:1"]
#[derive(Clone, Debug)]
struct jmpdestVar0 {
    simm8: TokenField_simm8,
}
impl jmpdestVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut reloc: i64 = 0;
        reloc = i64::try_from(inst_next)
            .unwrap()
            .wrapping_add(self.simm8.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, reloc)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let mut reloc: i64 = 0;
        let simm8 = token_parser.TokenFieldsimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[derive(Clone, Debug)]
enum Tablejmpdest {
    Var0(jmpdestVar0),
}
impl Tablejmpdest {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            jmpdestVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:391:1"]
#[derive(Clone, Debug)]
struct jmpdest11Var0 {
    jmp11_hi: TokenField_jmp11_hi,
    jmp11_lo: TokenField_jmp11_lo,
}
impl jmpdest11Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut reloc: i64 = 0;
        reloc = i64::try_from(inst_next).unwrap().wrapping_add(
            (self
                .jmp11_hi
                .disassembly()
                .checked_shl(u32::try_from(8i64).unwrap())
                .unwrap_or(0)
                | self.jmp11_lo.disassembly()),
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, reloc)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let mut reloc: i64 = 0;
        let jmp11_hi = token_parser.TokenFieldjmp11_hi();
        let jmp11_lo = token_parser.TokenFieldjmp11_lo();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { jmp11_hi, jmp11_lo }))
    }
}
#[derive(Clone, Debug)]
enum Tablejmpdest11 {
    Var0(jmpdest11Var0),
}
impl Tablejmpdest11 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            jmpdest11Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:392:1"]
#[derive(Clone, Debug)]
struct jmpdest16Var0 {
    disp16: TokenField_disp16,
}
impl jmpdest16Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut reloc: i64 = 0;
        reloc = i64::try_from(inst_next)
            .unwrap()
            .wrapping_add(self.disp16.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, reloc)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let mut reloc: i64 = 0;
        let disp16 = token_parser.TokenFielddisp16();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp16 }))
    }
}
#[derive(Clone, Debug)]
enum Tablejmpdest16 {
    Var0(jmpdest16Var0),
}
impl Tablejmpdest16 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            jmpdest16Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:767:1"]
#[derive(Clone, Debug)]
struct ccVar0 {}
impl ccVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NST")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:768:1"]
#[derive(Clone, Debug)]
struct ccVar1 {}
impl ccVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NH")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 1i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:769:1"]
#[derive(Clone, Debug)]
struct ccVar2 {}
impl ccVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("GT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:770:1"]
#[derive(Clone, Debug)]
struct ccVar3 {}
impl ccVar3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NC")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 3i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:771:1"]
#[derive(Clone, Debug)]
struct ccVar4 {}
impl ccVar4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NVT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 4i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:772:1"]
#[derive(Clone, Debug)]
struct ccVar5 {}
impl ccVar5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NV")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 5i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:773:1"]
#[derive(Clone, Debug)]
struct ccVar6 {}
impl ccVar6 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("GE")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 6i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:774:1"]
#[derive(Clone, Debug)]
struct ccVar7 {}
impl ccVar7 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NE")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 7i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:775:1"]
#[derive(Clone, Debug)]
struct ccVar8 {}
impl ccVar8 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ST")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 8i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:776:1"]
#[derive(Clone, Debug)]
struct ccVar9 {}
impl ccVar9 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("H")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 9i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:777:1"]
#[derive(Clone, Debug)]
struct ccVar10 {}
impl ccVar10 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("LE")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 10i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:778:1"]
#[derive(Clone, Debug)]
struct ccVar11 {}
impl ccVar11 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("C")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 11i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:779:1"]
#[derive(Clone, Debug)]
struct ccVar12 {}
impl ccVar12 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("VT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 12i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:780:1"]
#[derive(Clone, Debug)]
struct ccVar13 {}
impl ccVar13 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("V")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 13i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:781:1"]
#[derive(Clone, Debug)]
struct ccVar14 {}
impl ccVar14 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("LT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 14i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/MCS96/data/languages/MCS96.sinc:782:1"]
#[derive(Clone, Debug)]
struct ccVar15 {}
impl ccVar15 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("E")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcond().disassembly() != 15i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablecc {
    Var0(ccVar0),
    Var1(ccVar1),
    Var2(ccVar2),
    Var3(ccVar3),
    Var4(ccVar4),
    Var5(ccVar5),
    Var6(ccVar6),
    Var7(ccVar7),
    Var8(ccVar8),
    Var9(ccVar9),
    Var10(ccVar10),
    Var11(ccVar11),
    Var12(ccVar12),
    Var13(ccVar13),
    Var14(ccVar14),
    Var15(ccVar15),
}
impl Tablecc {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var2(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var3(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var4(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var5(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var6(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var7(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var8(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var9(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var10(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var11(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var12(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var13(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var14(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var15(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            ccVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar4::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar5::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar6::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar7::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar8::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar9::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar10::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar11::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar12::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar13::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar14::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ccVar15::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        None
    }
}
pub fn parse_instruction<T>(
    tokens: &[u8],
    context: &mut T,
    inst_start: u16,
    global_set: &mut impl GlobalSetTrait,
) -> Option<(u16, Vec<DisplayElement>)>
where
    T: ContextTrait + Clone,
{
    let (inst_len, instruction) =
        Tableinstruction::parse(tokens, context, inst_start)?;
    let inst_next = inst_start + inst_len;
    let mut display = vec![];
    instruction.display_extend(
        &mut display,
        context,
        inst_start,
        inst_next,
        global_set,
    );
    Some((inst_next, display))
}
