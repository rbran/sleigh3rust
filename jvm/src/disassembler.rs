pub type AddrType = u32;
macro_rules! impl_read_to_type {
    ($ unsigned_type : ty , $ signed_type : ty , $ len : literal , $ read_unsigned : ident , $ read_signed : ident , $ write_unsigned : ident , $ write_signed : ident) => {
        fn $read_unsigned<const BIG_ENDIAN: bool>(
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
        fn $read_signed<const BIG_ENDIAN: bool>(
            data: [u8; $len],
            start_bit: usize,
            len_bits: usize,
        ) -> $signed_type {
            const TYPE_BITS: usize = <$signed_type>::BITS as usize;
            assert!(len_bits > 1);
            assert!(TYPE_BITS / 8 == $len);
            let data = $read_unsigned::<BIG_ENDIAN>(data, start_bit, len_bits);
            let value_mask = <$unsigned_type>::try_from(<$signed_type>::MAX)
                .unwrap()
                >> (TYPE_BITS - len_bits);
            let sign_mask = !value_mask;
            let value_part = data & value_mask;
            let sign_part = data & sign_mask;
            if sign_part != 0 {
                let neg_value = (!value_part + 1) & value_mask;
                <$signed_type>::try_from(neg_value)
                    .unwrap()
                    .checked_neg()
                    .unwrap()
            } else {
                <$signed_type>::try_from(value_part).unwrap()
            }
        }
        fn $write_unsigned<const BIG_ENDIAN: bool>(
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
        fn $write_signed<const BIG_ENDIAN: bool>(
            value: $signed_type,
            mem: $signed_type,
            start_bit: usize,
            len_bits: usize,
        ) -> [u8; $len] {
            const TYPE_BITS: usize = <$unsigned_type>::BITS as usize;
            assert!(len_bits > 0);
            assert!(len_bits + start_bit <= TYPE_BITS);
            let value: $unsigned_type = if value < 0 {
                <$unsigned_type>::MAX
                    - <$unsigned_type>::try_from(value.abs() - 1).unwrap()
            } else {
                <$unsigned_type>::try_from(value).unwrap()
            };
            let mem: $unsigned_type = if mem < 0 {
                <$unsigned_type>::MAX
                    - <$unsigned_type>::try_from(mem.abs() - 1).unwrap()
            } else {
                <$unsigned_type>::try_from(value).unwrap()
            };
            let mask = <$unsigned_type>::MAX >> (TYPE_BITS - len_bits);
            let value = value & mask;
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
impl_read_to_type!(
    ethnum::u256,
    ethnum::i256,
    32,
    read_u256,
    read_i256,
    write_u256,
    write_i256
);
pub trait GlobalSetTrait {
    fn set_switch_low(&mut self, address: Option<u32>, value: i64);
    fn set_switch_high(&mut self, address: Option<u32>, value: i64);
    fn set_switch_num(&mut self, address: Option<u32>, value: i64);
    fn set_in_table_switch(&mut self, address: Option<u32>, value: i64);
    fn set_in_lookup_switch(&mut self, address: Option<u32>, value: i64);
    fn set_alignmentPad(&mut self, address: Option<u32>, value: i64);
    fn set_padVal(&mut self, address: Option<u32>, value: i64);
}
pub trait MemoryRead {
    type AddressType;
    fn read(&self, addr: Self::AddressType, buf: &mut [u8]);
}
pub trait MemoryWrite {
    type AddressType;
    fn write(&mut self, addr: Self::AddressType, buf: &[u8]);
}
pub trait ContextregisterTrait:
    MemoryRead<AddressType = u32> + MemoryWrite<AddressType = u32>
{
    fn read_switch_low_raw(&self) -> u32 {
        let mut work_value = [0u8; 4u64 as usize];
        self.read(12u64 as u32, &mut work_value[0..4]);
        let value = read_u32::<true>(work_value, 0u64 as usize, 32u64 as usize);
        u32::try_from(value).unwrap()
    }
    fn write_switch_low_raw(&mut self, param: u32) {
        let mut mem = [0u8; 4];
        self.read(12u64 as u32, &mut mem[0..4]);
        let mem = u32::from_be_bytes(mem);
        let mem =
            write_u32::<true>(param as u32, mem, 0u64 as usize, 32u64 as usize);
        self.write(12u64 as u32, &mem[0..4]);
    }
    fn read_switch_low_disassembly(&self) -> i64 {
        i64::try_from(self.read_switch_low_raw()).unwrap()
    }
    fn write_switch_low_disassembly(&mut self, param: i64) {
        self.write_switch_low_raw(param as u32)
    }
    fn read_switch_low_execution(&self) -> u32 {
        self.read_switch_low_raw()
    }
    fn write_switch_low_execution(&mut self, param: u32) {
        self.write_switch_low_raw(param)
    }
    fn switch_low_display(&self) -> DisplayElement {
        meaning_number(true, self.read_switch_low_raw())
    }
    fn read_switch_high_raw(&self) -> u32 {
        let mut work_value = [0u8; 4u64 as usize];
        self.read(8u64 as u32, &mut work_value[0..4]);
        let value = read_u32::<true>(work_value, 0u64 as usize, 32u64 as usize);
        u32::try_from(value).unwrap()
    }
    fn write_switch_high_raw(&mut self, param: u32) {
        let mut mem = [0u8; 4];
        self.read(8u64 as u32, &mut mem[0..4]);
        let mem = u32::from_be_bytes(mem);
        let mem =
            write_u32::<true>(param as u32, mem, 0u64 as usize, 32u64 as usize);
        self.write(8u64 as u32, &mem[0..4]);
    }
    fn read_switch_high_disassembly(&self) -> i64 {
        i64::try_from(self.read_switch_high_raw()).unwrap()
    }
    fn write_switch_high_disassembly(&mut self, param: i64) {
        self.write_switch_high_raw(param as u32)
    }
    fn read_switch_high_execution(&self) -> u32 {
        self.read_switch_high_raw()
    }
    fn write_switch_high_execution(&mut self, param: u32) {
        self.write_switch_high_raw(param)
    }
    fn switch_high_display(&self) -> DisplayElement {
        meaning_number(true, self.read_switch_high_raw())
    }
    fn read_switch_num_raw(&self) -> u32 {
        let mut work_value = [0u8; 4u64 as usize];
        self.read(4u64 as u32, &mut work_value[0..4]);
        let value = read_u32::<true>(work_value, 0u64 as usize, 32u64 as usize);
        u32::try_from(value).unwrap()
    }
    fn write_switch_num_raw(&mut self, param: u32) {
        let mut mem = [0u8; 4];
        self.read(4u64 as u32, &mut mem[0..4]);
        let mem = u32::from_be_bytes(mem);
        let mem =
            write_u32::<true>(param as u32, mem, 0u64 as usize, 32u64 as usize);
        self.write(4u64 as u32, &mem[0..4]);
    }
    fn read_switch_num_disassembly(&self) -> i64 {
        i64::try_from(self.read_switch_num_raw()).unwrap()
    }
    fn write_switch_num_disassembly(&mut self, param: i64) {
        self.write_switch_num_raw(param as u32)
    }
    fn read_switch_num_execution(&self) -> u32 {
        self.read_switch_num_raw()
    }
    fn write_switch_num_execution(&mut self, param: u32) {
        self.write_switch_num_raw(param)
    }
    fn switch_num_display(&self) -> DisplayElement {
        meaning_number(true, self.read_switch_num_raw())
    }
    fn read_in_table_switch_raw(&self) -> u8 {
        let mut work_value = [0u8; 1u64 as usize];
        self.read(3u64 as u32, &mut work_value[0..1]);
        let value = read_u8::<true>(work_value, 0u64 as usize, 2u64 as usize);
        u8::try_from(value).unwrap()
    }
    fn write_in_table_switch_raw(&mut self, param: u8) {
        let mut mem = [0u8; 1];
        self.read(3u64 as u32, &mut mem[0..1]);
        let mem = u8::from_be_bytes(mem);
        let mem =
            write_u8::<true>(param as u8, mem, 0u64 as usize, 2u64 as usize);
        self.write(3u64 as u32, &mem[0..1]);
    }
    fn read_in_table_switch_disassembly(&self) -> i64 {
        i64::try_from(self.read_in_table_switch_raw()).unwrap()
    }
    fn write_in_table_switch_disassembly(&mut self, param: i64) {
        self.write_in_table_switch_raw(param as u8)
    }
    fn read_in_table_switch_execution(&self) -> u8 {
        self.read_in_table_switch_raw()
    }
    fn write_in_table_switch_execution(&mut self, param: u8) {
        self.write_in_table_switch_raw(param)
    }
    fn in_table_switch_display(&self) -> DisplayElement {
        meaning_number(true, self.read_in_table_switch_raw())
    }
    fn read_in_lookup_switch_raw(&self) -> u8 {
        let mut work_value = [0u8; 1u64 as usize];
        self.read(3u64 as u32, &mut work_value[0..1]);
        let value = read_u8::<true>(work_value, 2u64 as usize, 2u64 as usize);
        u8::try_from(value).unwrap()
    }
    fn write_in_lookup_switch_raw(&mut self, param: u8) {
        let mut mem = [0u8; 1];
        self.read(3u64 as u32, &mut mem[0..1]);
        let mem = u8::from_be_bytes(mem);
        let mem =
            write_u8::<true>(param as u8, mem, 2u64 as usize, 2u64 as usize);
        self.write(3u64 as u32, &mem[0..1]);
    }
    fn read_in_lookup_switch_disassembly(&self) -> i64 {
        i64::try_from(self.read_in_lookup_switch_raw()).unwrap()
    }
    fn write_in_lookup_switch_disassembly(&mut self, param: i64) {
        self.write_in_lookup_switch_raw(param as u8)
    }
    fn read_in_lookup_switch_execution(&self) -> u8 {
        self.read_in_lookup_switch_raw()
    }
    fn write_in_lookup_switch_execution(&mut self, param: u8) {
        self.write_in_lookup_switch_raw(param)
    }
    fn in_lookup_switch_display(&self) -> DisplayElement {
        meaning_number(true, self.read_in_lookup_switch_raw())
    }
    fn read_alignmentPad_raw(&self) -> u8 {
        let mut work_value = [0u8; 1u64 as usize];
        self.read(3u64 as u32, &mut work_value[0..1]);
        let value = read_u8::<true>(work_value, 4u64 as usize, 2u64 as usize);
        u8::try_from(value).unwrap()
    }
    fn write_alignmentPad_raw(&mut self, param: u8) {
        let mut mem = [0u8; 1];
        self.read(3u64 as u32, &mut mem[0..1]);
        let mem = u8::from_be_bytes(mem);
        let mem =
            write_u8::<true>(param as u8, mem, 4u64 as usize, 2u64 as usize);
        self.write(3u64 as u32, &mem[0..1]);
    }
    fn read_alignmentPad_disassembly(&self) -> i64 {
        i64::try_from(self.read_alignmentPad_raw()).unwrap()
    }
    fn write_alignmentPad_disassembly(&mut self, param: i64) {
        self.write_alignmentPad_raw(param as u8)
    }
    fn read_alignmentPad_execution(&self) -> u8 {
        self.read_alignmentPad_raw()
    }
    fn write_alignmentPad_execution(&mut self, param: u8) {
        self.write_alignmentPad_raw(param)
    }
    fn alignmentPad_display(&self) -> DisplayElement {
        meaning_number(true, self.read_alignmentPad_raw())
    }
    fn read_padVal_raw(&self) -> u8 {
        let mut work_value = [0u8; 1u64 as usize];
        self.read(3u64 as u32, &mut work_value[0..1]);
        let value = read_u8::<true>(work_value, 4u64 as usize, 2u64 as usize);
        u8::try_from(value).unwrap()
    }
    fn write_padVal_raw(&mut self, param: u8) {
        let mut mem = [0u8; 1];
        self.read(3u64 as u32, &mut mem[0..1]);
        let mem = u8::from_be_bytes(mem);
        let mem =
            write_u8::<true>(param as u8, mem, 4u64 as usize, 2u64 as usize);
        self.write(3u64 as u32, &mem[0..1]);
    }
    fn read_padVal_disassembly(&self) -> i64 {
        i64::try_from(self.read_padVal_raw()).unwrap()
    }
    fn write_padVal_disassembly(&mut self, param: i64) {
        self.write_padVal_raw(param as u8)
    }
    fn read_padVal_execution(&self) -> u8 {
        self.read_padVal_raw()
    }
    fn write_padVal_execution(&mut self, param: u8) {
        self.write_padVal_raw(param)
    }
    fn padVal_display(&self) -> DisplayElement {
        meaning_number(true, self.read_padVal_raw())
    }
}
pub trait ContextTrait {
    type Typeregister: ContextregisterTrait;
    fn register(&self) -> &Self::Typeregister;
    fn register_mut(&mut self) -> &mut Self::Typeregister;
}
#[derive(Debug, Clone, Copy, Default)]
pub struct ContextregisterStruct {
    pub chunk_0x0: [u8; 16u64 as usize],
}
impl ContextregisterTrait for ContextregisterStruct {}
impl MemoryRead for ContextregisterStruct {
    type AddressType = u32;
    fn read(&self, addr: Self::AddressType, buf: &mut [u8]) {
        let addr = <u64>::try_from(addr).unwrap();
        let buf_len = <u64>::try_from(buf.len()).unwrap();
        let addr_end = addr + buf_len;
        match (addr, addr_end) {
            (0u64..=15u64, 0u64..=16u64) => {
                let start = addr - 0u64;
                let end = usize::try_from(start + buf_len).unwrap();
                let start = usize::try_from(start).unwrap();
                buf.copy_from_slice(&self.chunk_0x0[start..end]);
            }
            _ => panic!("undefined mem {}:{}", addr, buf.len()),
        }
    }
}
impl MemoryWrite for ContextregisterStruct {
    type AddressType = u32;
    fn write(&mut self, addr: Self::AddressType, buf: &[u8]) {
        let addr = <u64>::try_from(addr).unwrap();
        let buf_len = <u64>::try_from(buf.len()).unwrap();
        let addr_end = addr + buf_len;
        match (addr, addr_end) {
            (0u64..=15u64, 0u64..=16u64) => {
                let start = addr - 0u64;
                let end = usize::try_from(start + buf_len).unwrap();
                let start = usize::try_from(start).unwrap();
                self.chunk_0x0[start..end].copy_from_slice(buf);
            }
            _ => panic!("undefined mem {}:{}", addr, buf.len()),
        }
    }
}
#[derive(Debug, Clone, Copy, Default)]
pub struct SpacesStruct {
    pub register: ContextregisterStruct,
}
impl ContextTrait for SpacesStruct {
    type Typeregister = ContextregisterStruct;
    fn register(&self) -> &Self::Typeregister {
        &self.register
    }
    fn register_mut(&mut self) -> &mut Self::Typeregister {
        &mut self.register
    }
}
fn meaning_number<T>(hex: bool, num: T) -> DisplayElement
where
    i64: TryFrom<T>,
    <i64 as TryFrom<T>>::Error: core::fmt::Debug,
{
    DisplayElement::Number(hex, i64::try_from(num).unwrap())
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op(u8);
impl TokenField_op {
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
struct TokenField_w_op(u16);
impl TokenField_w_op {
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
struct TokenField_n(u8);
impl TokenField_n {
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
struct TokenField_m(u8);
impl TokenField_m {
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
struct TokenField_atype(u8);
impl TokenField_atype {
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
struct TokenField_byte(u8);
impl TokenField_byte {
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
struct TokenField_byte1(u8);
impl TokenField_byte1 {
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
struct TokenField_byte2(u8);
impl TokenField_byte2 {
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
struct TokenField_byte3(u8);
impl TokenField_byte3 {
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
struct TokenField_byte4(u8);
impl TokenField_byte4 {
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
struct TokenField_sbyte(i8);
impl TokenField_sbyte {
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
struct TokenField_branch(i8);
impl TokenField_branch {
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
struct TokenField_branchbyte1(i8);
impl TokenField_branchbyte1 {
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
struct TokenField_branchbyte2(u8);
impl TokenField_branchbyte2 {
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
struct TokenField_branchbyte3(u8);
impl TokenField_branchbyte3 {
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
struct TokenField_branchbyte4(u8);
impl TokenField_branchbyte4 {
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
struct TokenField_index(u8);
impl TokenField_index {
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
struct TokenField_indexbyte1(u8);
impl TokenField_indexbyte1 {
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
struct TokenField_indexbyte2(u8);
impl TokenField_indexbyte2 {
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
struct TokenField_constant(u8);
impl TokenField_constant {
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
struct TokenField_constantbyte1(u8);
impl TokenField_constantbyte1 {
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
struct TokenField_constantbyte2(u8);
impl TokenField_constantbyte2 {
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
struct TokenField_nargs(u8);
impl TokenField_nargs {
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
struct TokenField_method(u8);
impl TokenField_method {
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
struct TokenField_defaultbyte1(u8);
impl TokenField_defaultbyte1 {
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
struct TokenField_defaultbyte2(u8);
impl TokenField_defaultbyte2 {
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
struct TokenField_defaultbyte3(u8);
impl TokenField_defaultbyte3 {
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
struct TokenField_defaultbyte4(u8);
impl TokenField_defaultbyte4 {
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
struct TokenField_highbyte1(u8);
impl TokenField_highbyte1 {
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
struct TokenField_highbyte2(u8);
impl TokenField_highbyte2 {
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
struct TokenField_highbyte3(u8);
impl TokenField_highbyte3 {
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
struct TokenField_highbyte4(u8);
impl TokenField_highbyte4 {
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
struct TokenField_lowbyte1(u8);
impl TokenField_lowbyte1 {
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
struct TokenField_lowbyte2(u8);
impl TokenField_lowbyte2 {
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
struct TokenField_lowbyte3(u8);
impl TokenField_lowbyte3 {
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
struct TokenField_lowbyte4(u8);
impl TokenField_lowbyte4 {
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
struct TokenField_npairsbyte1(u8);
impl TokenField_npairsbyte1 {
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
struct TokenField_npairsbyte2(u8);
impl TokenField_npairsbyte2 {
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
struct TokenField_npairsbyte3(u8);
impl TokenField_npairsbyte3 {
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
struct TokenField_npairsbyte4(u8);
impl TokenField_npairsbyte4 {
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
struct TokenField_dimensions(u8);
impl TokenField_dimensions {
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
struct TokenField_blank1(u8);
impl TokenField_blank1 {
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
struct TokenField_blank2(u8);
impl TokenField_blank2 {
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
struct TokenField_count(u8);
impl TokenField_count {
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
struct TokenField_pad(u8);
impl TokenField_pad {
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
struct TokenField_pad1(u8);
impl TokenField_pad1 {
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
struct TokenField_pad2(u8);
impl TokenField_pad2 {
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
struct TokenField_pad3(u8);
impl TokenField_pad3 {
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
struct TokenField_wide_op(u8);
impl TokenField_wide_op {
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
struct TokenField_matchbyte1(u8);
impl TokenField_matchbyte1 {
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
struct TokenField_matchbyte2(u8);
impl TokenField_matchbyte2 {
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
struct TokenField_matchbyte3(u8);
impl TokenField_matchbyte3 {
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
struct TokenField_matchbyte4(u8);
impl TokenField_matchbyte4 {
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
struct TokenField_offsetbyte1(u8);
impl TokenField_offsetbyte1 {
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
struct TokenField_offsetbyte2(u8);
impl TokenField_offsetbyte2 {
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
struct TokenField_offsetbyte3(u8);
impl TokenField_offsetbyte3 {
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
struct TokenField_offsetbyte4(u8);
impl TokenField_offsetbyte4 {
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
    fn TokenFieldop(&self) -> TokenField_op {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op(inner_value)
    }
    fn TokenFieldw_op(&self) -> TokenField_w_op {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<true>(work_value, 0u64 as usize, 16u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_w_op(inner_value)
    }
    fn TokenFieldn(&self) -> TokenField_n {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_n(inner_value)
    }
    fn TokenFieldm(&self) -> TokenField_m {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 4u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_m(inner_value)
    }
    fn TokenFieldatype(&self) -> TokenField_atype {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_atype(inner_value)
    }
    fn TokenFieldbyte(&self) -> TokenField_byte {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_byte(inner_value)
    }
    fn TokenFieldbyte1(&self) -> TokenField_byte1 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_byte1(inner_value)
    }
    fn TokenFieldbyte2(&self) -> TokenField_byte2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_byte2(inner_value)
    }
    fn TokenFieldbyte3(&self) -> TokenField_byte3 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_byte3(inner_value)
    }
    fn TokenFieldbyte4(&self) -> TokenField_byte4 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_byte4(inner_value)
    }
    fn TokenFieldsbyte(&self) -> TokenField_sbyte {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_sbyte(inner_value)
    }
    fn TokenFieldbranch(&self) -> TokenField_branch {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_branch(inner_value)
    }
    fn TokenFieldbranchbyte1(&self) -> TokenField_branchbyte1 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_branchbyte1(inner_value)
    }
    fn TokenFieldbranchbyte2(&self) -> TokenField_branchbyte2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_branchbyte2(inner_value)
    }
    fn TokenFieldbranchbyte3(&self) -> TokenField_branchbyte3 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_branchbyte3(inner_value)
    }
    fn TokenFieldbranchbyte4(&self) -> TokenField_branchbyte4 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_branchbyte4(inner_value)
    }
    fn TokenFieldindex(&self) -> TokenField_index {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_index(inner_value)
    }
    fn TokenFieldindexbyte1(&self) -> TokenField_indexbyte1 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_indexbyte1(inner_value)
    }
    fn TokenFieldindexbyte2(&self) -> TokenField_indexbyte2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_indexbyte2(inner_value)
    }
    fn TokenFieldconstant(&self) -> TokenField_constant {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_constant(inner_value)
    }
    fn TokenFieldconstantbyte1(&self) -> TokenField_constantbyte1 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_constantbyte1(inner_value)
    }
    fn TokenFieldconstantbyte2(&self) -> TokenField_constantbyte2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_constantbyte2(inner_value)
    }
    fn TokenFieldnargs(&self) -> TokenField_nargs {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_nargs(inner_value)
    }
    fn TokenFieldmethod(&self) -> TokenField_method {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_method(inner_value)
    }
    fn TokenFielddefaultbyte1(&self) -> TokenField_defaultbyte1 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_defaultbyte1(inner_value)
    }
    fn TokenFielddefaultbyte2(&self) -> TokenField_defaultbyte2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_defaultbyte2(inner_value)
    }
    fn TokenFielddefaultbyte3(&self) -> TokenField_defaultbyte3 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_defaultbyte3(inner_value)
    }
    fn TokenFielddefaultbyte4(&self) -> TokenField_defaultbyte4 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_defaultbyte4(inner_value)
    }
    fn TokenFieldhighbyte1(&self) -> TokenField_highbyte1 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_highbyte1(inner_value)
    }
    fn TokenFieldhighbyte2(&self) -> TokenField_highbyte2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_highbyte2(inner_value)
    }
    fn TokenFieldhighbyte3(&self) -> TokenField_highbyte3 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_highbyte3(inner_value)
    }
    fn TokenFieldhighbyte4(&self) -> TokenField_highbyte4 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_highbyte4(inner_value)
    }
    fn TokenFieldlowbyte1(&self) -> TokenField_lowbyte1 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_lowbyte1(inner_value)
    }
    fn TokenFieldlowbyte2(&self) -> TokenField_lowbyte2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_lowbyte2(inner_value)
    }
    fn TokenFieldlowbyte3(&self) -> TokenField_lowbyte3 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_lowbyte3(inner_value)
    }
    fn TokenFieldlowbyte4(&self) -> TokenField_lowbyte4 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_lowbyte4(inner_value)
    }
    fn TokenFieldnpairsbyte1(&self) -> TokenField_npairsbyte1 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_npairsbyte1(inner_value)
    }
    fn TokenFieldnpairsbyte2(&self) -> TokenField_npairsbyte2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_npairsbyte2(inner_value)
    }
    fn TokenFieldnpairsbyte3(&self) -> TokenField_npairsbyte3 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_npairsbyte3(inner_value)
    }
    fn TokenFieldnpairsbyte4(&self) -> TokenField_npairsbyte4 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_npairsbyte4(inner_value)
    }
    fn TokenFielddimensions(&self) -> TokenField_dimensions {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_dimensions(inner_value)
    }
    fn TokenFieldblank1(&self) -> TokenField_blank1 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_blank1(inner_value)
    }
    fn TokenFieldblank2(&self) -> TokenField_blank2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_blank2(inner_value)
    }
    fn TokenFieldcount(&self) -> TokenField_count {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_count(inner_value)
    }
    fn TokenFieldpad(&self) -> TokenField_pad {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_pad(inner_value)
    }
    fn TokenFieldpad1(&self) -> TokenField_pad1 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_pad1(inner_value)
    }
    fn TokenFieldpad2(&self) -> TokenField_pad2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_pad2(inner_value)
    }
    fn TokenFieldpad3(&self) -> TokenField_pad3 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_pad3(inner_value)
    }
    fn TokenFieldwide_op(&self) -> TokenField_wide_op {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_wide_op(inner_value)
    }
    fn TokenFieldmatchbyte1(&self) -> TokenField_matchbyte1 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_matchbyte1(inner_value)
    }
    fn TokenFieldmatchbyte2(&self) -> TokenField_matchbyte2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_matchbyte2(inner_value)
    }
    fn TokenFieldmatchbyte3(&self) -> TokenField_matchbyte3 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_matchbyte3(inner_value)
    }
    fn TokenFieldmatchbyte4(&self) -> TokenField_matchbyte4 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_matchbyte4(inner_value)
    }
    fn TokenFieldoffsetbyte1(&self) -> TokenField_offsetbyte1 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_offsetbyte1(inner_value)
    }
    fn TokenFieldoffsetbyte2(&self) -> TokenField_offsetbyte2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_offsetbyte2(inner_value)
    }
    fn TokenFieldoffsetbyte3(&self) -> TokenField_offsetbyte3 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_offsetbyte3(inner_value)
    }
    fn TokenFieldoffsetbyte4(&self) -> TokenField_offsetbyte4 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_offsetbyte4(inner_value)
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    cat2_return_value,
    return_value,
    SP,
    PC,
    switch_target,
    return_address,
    call_target,
    LVA,
    switch_ctrl,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::cat2_return_value => write!(f, "cat2_return_value"),
            Self::return_value => write!(f, "return_value"),
            Self::SP => write!(f, "SP"),
            Self::PC => write!(f, "PC"),
            Self::switch_target => write!(f, "switch_target"),
            Self::return_address => write!(f, "return_address"),
            Self::call_target => write!(f, "call_target"),
            Self::LVA => write!(f, "LVA"),
            Self::switch_ctrl => write!(f, "switch_ctrl"),
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:222:1"]
#[derive(Clone, Debug)]
struct instructionVar0 {}
impl instructionVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("aaload")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 50i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:233:1"]
#[derive(Clone, Debug)]
struct instructionVar1 {}
impl instructionVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("aastore")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 83i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:246:1"]
#[derive(Clone, Debug)]
struct instructionVar2 {}
impl instructionVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("aconst_null")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 1i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:251:1"]
#[derive(Clone, Debug)]
struct instructionVar3 {
    index: TokenField_index,
}
impl instructionVar3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("aload"),
            DisplayElement::Literal(" "),
            self.index.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 25i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let index = token_parser.TokenFieldindex();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:258:1"]
#[derive(Clone, Debug)]
struct instructionVar4 {}
impl instructionVar4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("aload_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 42i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:265:1"]
#[derive(Clone, Debug)]
struct instructionVar5 {}
impl instructionVar5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("aload_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 43i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:272:1"]
#[derive(Clone, Debug)]
struct instructionVar6 {}
impl instructionVar6 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("aload_2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 44i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:279:1"]
#[derive(Clone, Debug)]
struct instructionVar7 {}
impl instructionVar7 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("aload_3")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 45i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:286:1"]
#[derive(Clone, Debug)]
struct instructionVar8 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar8 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("anewarray"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 189i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:296:1"]
#[derive(Clone, Debug)]
struct instructionVar9 {}
impl instructionVar9 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("areturn")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 176i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:302:1"]
#[derive(Clone, Debug)]
struct instructionVar10 {}
impl instructionVar10 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("arraylength")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 190i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:311:1"]
#[derive(Clone, Debug)]
struct instructionVar11 {
    index: TokenField_index,
}
impl instructionVar11 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("astore"),
            DisplayElement::Literal(" "),
            self.index.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 58i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let index = token_parser.TokenFieldindex();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:319:1"]
#[derive(Clone, Debug)]
struct instructionVar12 {}
impl instructionVar12 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("astore_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 75i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:327:1"]
#[derive(Clone, Debug)]
struct instructionVar13 {}
impl instructionVar13 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("astore_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 76i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:335:1"]
#[derive(Clone, Debug)]
struct instructionVar14 {}
impl instructionVar14 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("astore_2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 77i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:344:1"]
#[derive(Clone, Debug)]
struct instructionVar15 {}
impl instructionVar15 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("astore_3")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 78i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:352:1"]
#[derive(Clone, Debug)]
struct instructionVar16 {}
impl instructionVar16 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("athrow")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 191i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:362:1"]
#[derive(Clone, Debug)]
struct instructionVar17 {}
impl instructionVar17 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("baload")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 51i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:375:1"]
#[derive(Clone, Debug)]
struct instructionVar18 {}
impl instructionVar18 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("bastore")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 84i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:387:1"]
#[derive(Clone, Debug)]
struct instructionVar19 {
    sbyte: TokenField_sbyte,
}
impl instructionVar19 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("bipush"),
            DisplayElement::Literal(" "),
            self.sbyte.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 16i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let sbyte = token_parser.TokenFieldsbyte();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sbyte }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:393:1"]
#[derive(Clone, Debug)]
struct instructionVar20 {}
impl instructionVar20 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("caload")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 52i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:406:1"]
#[derive(Clone, Debug)]
struct instructionVar21 {}
impl instructionVar21 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("castore")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 85i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:418:1"]
#[derive(Clone, Debug)]
struct instructionVar22 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar22 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("checkcast"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 192i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:440:1"]
#[derive(Clone, Debug)]
struct instructionVar23 {}
impl instructionVar23 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("d2f")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 144i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:451:1"]
#[derive(Clone, Debug)]
struct instructionVar24 {}
impl instructionVar24 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("d2i")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 142i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:461:1"]
#[derive(Clone, Debug)]
struct instructionVar25 {}
impl instructionVar25 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("d2l")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 143i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:470:1"]
#[derive(Clone, Debug)]
struct instructionVar26 {}
impl instructionVar26 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dadd")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 99i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:481:1"]
#[derive(Clone, Debug)]
struct instructionVar27 {}
impl instructionVar27 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("daload")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 49i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:493:1"]
#[derive(Clone, Debug)]
struct instructionVar28 {}
impl instructionVar28 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dastore")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 82i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:505:1"]
#[derive(Clone, Debug)]
struct instructionVar29 {}
impl instructionVar29 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dcmpg")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 152i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:516:1"]
#[derive(Clone, Debug)]
struct instructionVar30 {}
impl instructionVar30 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dcmpl")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 151i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:527:1"]
#[derive(Clone, Debug)]
struct instructionVar31 {}
impl instructionVar31 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dconst_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 14i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:534:1"]
#[derive(Clone, Debug)]
struct instructionVar32 {}
impl instructionVar32 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dconst_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 15i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:541:1"]
#[derive(Clone, Debug)]
struct instructionVar33 {}
impl instructionVar33 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ddiv")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 111i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:552:1"]
#[derive(Clone, Debug)]
struct instructionVar34 {
    index: TokenField_index,
}
impl instructionVar34 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("dload"),
            DisplayElement::Literal("\t"),
            self.index.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 24i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let index = token_parser.TokenFieldindex();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:559:1"]
#[derive(Clone, Debug)]
struct instructionVar35 {}
impl instructionVar35 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dload_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 38i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:566:1"]
#[derive(Clone, Debug)]
struct instructionVar36 {}
impl instructionVar36 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dload_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 39i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:573:1"]
#[derive(Clone, Debug)]
struct instructionVar37 {}
impl instructionVar37 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dload_2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 40i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:580:1"]
#[derive(Clone, Debug)]
struct instructionVar38 {}
impl instructionVar38 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dload_3")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 41i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:587:1"]
#[derive(Clone, Debug)]
struct instructionVar39 {}
impl instructionVar39 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dmul")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 107i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:598:1"]
#[derive(Clone, Debug)]
struct instructionVar40 {}
impl instructionVar40 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dneg")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 119i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:607:1"]
#[derive(Clone, Debug)]
struct instructionVar41 {}
impl instructionVar41 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("drem")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 115i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:618:1"]
#[derive(Clone, Debug)]
struct instructionVar42 {}
impl instructionVar42 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dreturn")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 175i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:624:1"]
#[derive(Clone, Debug)]
struct instructionVar43 {
    index: TokenField_index,
}
impl instructionVar43 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("dstore"),
            DisplayElement::Literal("\t"),
            self.index.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 57i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let index = token_parser.TokenFieldindex();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:632:1"]
#[derive(Clone, Debug)]
struct instructionVar44 {}
impl instructionVar44 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dstore_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 71i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:640:1"]
#[derive(Clone, Debug)]
struct instructionVar45 {}
impl instructionVar45 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dstore_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 72i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:648:1"]
#[derive(Clone, Debug)]
struct instructionVar46 {}
impl instructionVar46 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dstore_2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 73i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:656:1"]
#[derive(Clone, Debug)]
struct instructionVar47 {}
impl instructionVar47 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dstore_3")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 74i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:664:1"]
#[derive(Clone, Debug)]
struct instructionVar48 {}
impl instructionVar48 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dsub")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 103i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:675:1"]
#[derive(Clone, Debug)]
struct instructionVar49 {}
impl instructionVar49 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("dup")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 89i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:681:1"]
#[derive(Clone, Debug)]
struct instructionVar50 {}
impl instructionVar50 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dup_x1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 90i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:692:1"]
#[derive(Clone, Debug)]
struct instructionVar51 {}
impl instructionVar51 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dup_x2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 91i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:706:1"]
#[derive(Clone, Debug)]
struct instructionVar52 {}
impl instructionVar52 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dup2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 92i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:718:1"]
#[derive(Clone, Debug)]
struct instructionVar53 {}
impl instructionVar53 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dup2_x1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 93i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:733:1"]
#[derive(Clone, Debug)]
struct instructionVar54 {}
impl instructionVar54 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("dup2_x2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 94i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:751:1"]
#[derive(Clone, Debug)]
struct instructionVar55 {}
impl instructionVar55 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("f2d")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 141i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:761:1"]
#[derive(Clone, Debug)]
struct instructionVar56 {}
impl instructionVar56 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("f2i")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 139i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:771:1"]
#[derive(Clone, Debug)]
struct instructionVar57 {}
impl instructionVar57 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("f2l")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 140i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:782:1"]
#[derive(Clone, Debug)]
struct instructionVar58 {}
impl instructionVar58 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fadd")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 98i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:793:1"]
#[derive(Clone, Debug)]
struct instructionVar59 {}
impl instructionVar59 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("faload")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 48i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:805:1"]
#[derive(Clone, Debug)]
struct instructionVar60 {}
impl instructionVar60 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fastore")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 81i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:817:1"]
#[derive(Clone, Debug)]
struct instructionVar61 {}
impl instructionVar61 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fcmpg")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 150i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:828:1"]
#[derive(Clone, Debug)]
struct instructionVar62 {}
impl instructionVar62 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fcmpl")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 149i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:839:1"]
#[derive(Clone, Debug)]
struct instructionVar63 {}
impl instructionVar63 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fconst_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 11i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:846:1"]
#[derive(Clone, Debug)]
struct instructionVar64 {}
impl instructionVar64 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fconst_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 12i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:853:1"]
#[derive(Clone, Debug)]
struct instructionVar65 {}
impl instructionVar65 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fconst_2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 13i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:860:1"]
#[derive(Clone, Debug)]
struct instructionVar66 {}
impl instructionVar66 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fdiv")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 110i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:871:1"]
#[derive(Clone, Debug)]
struct instructionVar67 {
    index: TokenField_index,
}
impl instructionVar67 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("fload"),
            DisplayElement::Literal("\t"),
            self.index.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 23i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let index = token_parser.TokenFieldindex();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:878:1"]
#[derive(Clone, Debug)]
struct instructionVar68 {}
impl instructionVar68 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fload_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 34i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:885:1"]
#[derive(Clone, Debug)]
struct instructionVar69 {}
impl instructionVar69 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fload_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 35i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:892:1"]
#[derive(Clone, Debug)]
struct instructionVar70 {}
impl instructionVar70 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fload_2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 36i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:899:1"]
#[derive(Clone, Debug)]
struct instructionVar71 {}
impl instructionVar71 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fload_3")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 37i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:906:1"]
#[derive(Clone, Debug)]
struct instructionVar72 {}
impl instructionVar72 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fmul")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 106i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:917:1"]
#[derive(Clone, Debug)]
struct instructionVar73 {}
impl instructionVar73 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fneg")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 118i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:926:1"]
#[derive(Clone, Debug)]
struct instructionVar74 {}
impl instructionVar74 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("frem")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 114i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:937:1"]
#[derive(Clone, Debug)]
struct instructionVar75 {}
impl instructionVar75 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("freturn")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 174i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:943:1"]
#[derive(Clone, Debug)]
struct instructionVar76 {
    index: TokenField_index,
}
impl instructionVar76 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("fstore"),
            DisplayElement::Literal("\t"),
            self.index.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 56i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let index = token_parser.TokenFieldindex();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:951:1"]
#[derive(Clone, Debug)]
struct instructionVar77 {}
impl instructionVar77 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fstore_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 67i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:959:1"]
#[derive(Clone, Debug)]
struct instructionVar78 {}
impl instructionVar78 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fstore_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 68i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:966:1"]
#[derive(Clone, Debug)]
struct instructionVar79 {}
impl instructionVar79 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fstore_2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 69i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:973:1"]
#[derive(Clone, Debug)]
struct instructionVar80 {}
impl instructionVar80 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fstore_3")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 70i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:981:1"]
#[derive(Clone, Debug)]
struct instructionVar81 {}
impl instructionVar81 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fsub")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 102i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:992:1"]
#[derive(Clone, Debug)]
struct instructionVar82 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar82 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("getfield"),
            DisplayElement::Literal("\t"),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 180i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c21(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:998:1"]
#[derive(Clone, Debug)]
struct instructionVar83 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar83 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("getstatic"),
            DisplayElement::Literal(" \t"),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 178i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1003:1"]
#[derive(Clone, Debug)]
struct instructionVar84 {
    Branch: TableBranch,
}
impl instructionVar84 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("goto"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 167i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1008:1"]
#[derive(Clone, Debug)]
struct instructionVar85 {
    Branch_w: TableBranch_w,
}
impl instructionVar85 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("goto_w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Branch_w.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 200i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch_w = if let Some((len, table)) = TableBranch_w::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch_w }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1013:1"]
#[derive(Clone, Debug)]
struct instructionVar86 {}
impl instructionVar86 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("i2b")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 145i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1023:1"]
#[derive(Clone, Debug)]
struct instructionVar87 {}
impl instructionVar87 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("i2c")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 146i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1033:1"]
#[derive(Clone, Debug)]
struct instructionVar88 {}
impl instructionVar88 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("i2d")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 135i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1044:1"]
#[derive(Clone, Debug)]
struct instructionVar89 {}
impl instructionVar89 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("i2f")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 134i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1053:1"]
#[derive(Clone, Debug)]
struct instructionVar90 {}
impl instructionVar90 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("i2l")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 133i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1062:1"]
#[derive(Clone, Debug)]
struct instructionVar91 {}
impl instructionVar91 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("i2s")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 147i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1072:1"]
#[derive(Clone, Debug)]
struct instructionVar92 {}
impl instructionVar92 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iadd")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 96i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1083:1"]
#[derive(Clone, Debug)]
struct instructionVar93 {}
impl instructionVar93 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iaload")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 46i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1095:1"]
#[derive(Clone, Debug)]
struct instructionVar94 {}
impl instructionVar94 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iand")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 126i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1106:1"]
#[derive(Clone, Debug)]
struct instructionVar95 {}
impl instructionVar95 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iastore")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 79i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1118:1"]
#[derive(Clone, Debug)]
struct instructionVar96 {}
impl instructionVar96 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iconst_m1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c16 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 2i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c16(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1124:1"]
#[derive(Clone, Debug)]
struct instructionVar97 {}
impl instructionVar97 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iconst_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 3i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1130:1"]
#[derive(Clone, Debug)]
struct instructionVar98 {}
impl instructionVar98 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iconst_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 4i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1136:1"]
#[derive(Clone, Debug)]
struct instructionVar99 {}
impl instructionVar99 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iconst_2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 5i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1142:1"]
#[derive(Clone, Debug)]
struct instructionVar100 {}
impl instructionVar100 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iconst_3")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 6i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1148:1"]
#[derive(Clone, Debug)]
struct instructionVar101 {}
impl instructionVar101 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iconst_4")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 7i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1154:1"]
#[derive(Clone, Debug)]
struct instructionVar102 {}
impl instructionVar102 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iconst_5")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 8i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1160:1"]
#[derive(Clone, Debug)]
struct instructionVar103 {}
impl instructionVar103 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("idiv")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 108i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1171:1"]
#[derive(Clone, Debug)]
struct instructionVar104 {
    Branch: TableBranch,
}
impl instructionVar104 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("if_acmpeq"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c25 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 165i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c25(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1180:1"]
#[derive(Clone, Debug)]
struct instructionVar105 {
    Branch: TableBranch,
}
impl instructionVar105 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("if_acmpne"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c25 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 166i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c25(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1189:1"]
#[derive(Clone, Debug)]
struct instructionVar106 {
    Branch: TableBranch,
}
impl instructionVar106 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("if_icmpeq"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c25 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 159i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c25(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1198:1"]
#[derive(Clone, Debug)]
struct instructionVar107 {
    Branch: TableBranch,
}
impl instructionVar107 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("if_icmpne"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c25 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 160i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c25(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1207:1"]
#[derive(Clone, Debug)]
struct instructionVar108 {
    Branch: TableBranch,
}
impl instructionVar108 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("if_icmplt"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c25 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 161i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c25(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1216:1"]
#[derive(Clone, Debug)]
struct instructionVar109 {
    Branch: TableBranch,
}
impl instructionVar109 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("if_icmpge"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c25 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 162i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c25(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1225:1"]
#[derive(Clone, Debug)]
struct instructionVar110 {
    Branch: TableBranch,
}
impl instructionVar110 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("if_icmpgt"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c25 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 163i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c25(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1234:1"]
#[derive(Clone, Debug)]
struct instructionVar111 {
    Branch: TableBranch,
}
impl instructionVar111 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("if_icmple"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c25 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 164i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c25(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1243:1"]
#[derive(Clone, Debug)]
struct instructionVar112 {
    Branch: TableBranch,
}
impl instructionVar112 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ifeq"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 153i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c20(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1250:1"]
#[derive(Clone, Debug)]
struct instructionVar113 {
    Branch: TableBranch,
}
impl instructionVar113 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ifne"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 154i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c20(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1257:1"]
#[derive(Clone, Debug)]
struct instructionVar114 {
    Branch: TableBranch,
}
impl instructionVar114 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("iflt"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 155i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c20(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1264:1"]
#[derive(Clone, Debug)]
struct instructionVar115 {
    Branch: TableBranch,
}
impl instructionVar115 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ifge"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 156i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c20(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1271:1"]
#[derive(Clone, Debug)]
struct instructionVar116 {
    Branch: TableBranch,
}
impl instructionVar116 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ifgt"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 157i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c20(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1278:1"]
#[derive(Clone, Debug)]
struct instructionVar117 {
    Branch: TableBranch,
}
impl instructionVar117 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ifle"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 158i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c20(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1285:1"]
#[derive(Clone, Debug)]
struct instructionVar118 {
    Branch: TableBranch,
}
impl instructionVar118 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ifnonnull"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c25 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 199i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c25(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1292:1"]
#[derive(Clone, Debug)]
struct instructionVar119 {
    Branch: TableBranch,
}
impl instructionVar119 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ifnull"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 198i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1299:1"]
#[derive(Clone, Debug)]
struct instructionVar120 {
    index: TokenField_index,
    constant: TokenField_constant,
}
impl instructionVar120 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("iinc"),
            DisplayElement::Literal(" "),
            self.index.display(),
            DisplayElement::Literal(", "),
            self.constant.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c27 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 132i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c27(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let index = token_parser.TokenFieldindex();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let constant = token_parser.TokenFieldconstant();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index, constant }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1307:1"]
#[derive(Clone, Debug)]
struct instructionVar121 {
    index: TokenField_index,
}
impl instructionVar121 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("iload"),
            DisplayElement::Literal(" "),
            self.index.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 21i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let index = token_parser.TokenFieldindex();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1314:1"]
#[derive(Clone, Debug)]
struct instructionVar122 {}
impl instructionVar122 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iload_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 26i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1321:1"]
#[derive(Clone, Debug)]
struct instructionVar123 {}
impl instructionVar123 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iload_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 27i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1328:1"]
#[derive(Clone, Debug)]
struct instructionVar124 {}
impl instructionVar124 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iload_2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 28i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1334:1"]
#[derive(Clone, Debug)]
struct instructionVar125 {}
impl instructionVar125 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iload_3")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 29i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1341:1"]
#[derive(Clone, Debug)]
struct instructionVar126 {}
impl instructionVar126 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("imul")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 104i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1352:1"]
#[derive(Clone, Debug)]
struct instructionVar127 {}
impl instructionVar127 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ineg")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 116i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1361:1"]
#[derive(Clone, Debug)]
struct instructionVar128 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar128 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("instanceof"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 193i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1371:1"]
#[derive(Clone, Debug)]
struct instructionVar129 {
    blank1: TokenField_blank1,
    blank2: TokenField_blank2,
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar129 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("invokedynamic"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
            DisplayElement::Literal(", "),
            self.blank1.display(),
            DisplayElement::Literal(", "),
            self.blank2.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c42 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 186i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c42(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let blank1 = token_parser.TokenFieldblank1();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let blank2 = token_parser.TokenFieldblank2();
        pattern_len += block_4_len;
        tokens_current =
            &tokens_current[usize::try_from(block_4_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
                blank1,
                blank2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1376:1"]
#[derive(Clone, Debug)]
struct instructionVar130 {
    count: TokenField_count,
    blank1: TokenField_blank1,
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar130 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("invokeinterface"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
            DisplayElement::Literal(", "),
            self.count.display(),
            DisplayElement::Literal(", "),
            self.blank1.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c43 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 185i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c43(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let count = token_parser.TokenFieldcount();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let blank1 = token_parser.TokenFieldblank1();
        pattern_len += block_4_len;
        tokens_current =
            &tokens_current[usize::try_from(block_4_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
                count,
                blank1,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1381:1"]
#[derive(Clone, Debug)]
struct instructionVar131 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar131 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("invokespecial"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c26 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 183i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c26(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1386:1"]
#[derive(Clone, Debug)]
struct instructionVar132 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar132 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("invokestatic"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c25 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 184i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c25(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1391:1"]
#[derive(Clone, Debug)]
struct instructionVar133 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar133 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("invokevirtual"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c26 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 182i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c26(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1397:1"]
#[derive(Clone, Debug)]
struct instructionVar134 {}
impl instructionVar134 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ior")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 128i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1409:1"]
#[derive(Clone, Debug)]
struct instructionVar135 {}
impl instructionVar135 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("irem")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 112i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1421:1"]
#[derive(Clone, Debug)]
struct instructionVar136 {}
impl instructionVar136 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ireturn")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 172i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1427:1"]
#[derive(Clone, Debug)]
struct instructionVar137 {}
impl instructionVar137 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ishl")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 120i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1439:1"]
#[derive(Clone, Debug)]
struct instructionVar138 {}
impl instructionVar138 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ishr")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 122i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1451:1"]
#[derive(Clone, Debug)]
struct instructionVar139 {
    index: TokenField_index,
}
impl instructionVar139 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("istore"),
            DisplayElement::Literal(" "),
            self.index.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 54i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let index = token_parser.TokenFieldindex();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1459:1"]
#[derive(Clone, Debug)]
struct instructionVar140 {}
impl instructionVar140 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("istore_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 59i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1467:1"]
#[derive(Clone, Debug)]
struct instructionVar141 {}
impl instructionVar141 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("istore_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 60i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1475:1"]
#[derive(Clone, Debug)]
struct instructionVar142 {}
impl instructionVar142 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("istore_2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 61i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1483:1"]
#[derive(Clone, Debug)]
struct instructionVar143 {}
impl instructionVar143 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("istore_3")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 62i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1491:1"]
#[derive(Clone, Debug)]
struct instructionVar144 {}
impl instructionVar144 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("isub")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 100i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1503:1"]
#[derive(Clone, Debug)]
struct instructionVar145 {}
impl instructionVar145 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("iushr")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 124i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1515:1"]
#[derive(Clone, Debug)]
struct instructionVar146 {}
impl instructionVar146 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ixor")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 130i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1527:1"]
#[derive(Clone, Debug)]
struct instructionVar147 {
    Branch: TableBranch,
}
impl instructionVar147 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("jsr"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 168i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch = if let Some((len, table)) = TableBranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1534:1"]
#[derive(Clone, Debug)]
struct instructionVar148 {
    Branch_w: TableBranch_w,
}
impl instructionVar148 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("jsr_w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Branch_w.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 201i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c21(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Branch_w = if let Some((len, table)) = TableBranch_w::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch_w }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1541:1"]
#[derive(Clone, Debug)]
struct instructionVar149 {}
impl instructionVar149 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("l2d")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 138i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1550:1"]
#[derive(Clone, Debug)]
struct instructionVar150 {}
impl instructionVar150 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("l2f")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 137i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1559:1"]
#[derive(Clone, Debug)]
struct instructionVar151 {}
impl instructionVar151 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("l2i")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 136i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1568:1"]
#[derive(Clone, Debug)]
struct instructionVar152 {}
impl instructionVar152 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ladd")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 97i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1579:1"]
#[derive(Clone, Debug)]
struct instructionVar153 {}
impl instructionVar153 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("laload")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 47i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1592:1"]
#[derive(Clone, Debug)]
struct instructionVar154 {}
impl instructionVar154 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("land")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 127i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1603:1"]
#[derive(Clone, Debug)]
struct instructionVar155 {}
impl instructionVar155 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lastore")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 80i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1616:1"]
#[derive(Clone, Debug)]
struct instructionVar156 {}
impl instructionVar156 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lcmp")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 148i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1627:1"]
#[derive(Clone, Debug)]
struct instructionVar157 {}
impl instructionVar157 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lconst_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 9i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1633:1"]
#[derive(Clone, Debug)]
struct instructionVar158 {}
impl instructionVar158 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lconst_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 10i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1639:1"]
#[derive(Clone, Debug)]
struct instructionVar159 {
    index: TokenField_index,
}
impl instructionVar159 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("ldc"),
            DisplayElement::Literal("\t"),
            self.index.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c16 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 18i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c16(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let index = token_parser.TokenFieldindex();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1644:1"]
#[derive(Clone, Debug)]
struct instructionVar160 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar160 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("ldc_w"),
            DisplayElement::Literal("\t"),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 19i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1649:1"]
#[derive(Clone, Debug)]
struct instructionVar161 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar161 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("ldc2_w"),
            DisplayElement::Literal("\t"),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 20i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1654:1"]
#[derive(Clone, Debug)]
struct instructionVar162 {}
impl instructionVar162 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ldiv")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 109i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1665:1"]
#[derive(Clone, Debug)]
struct instructionVar163 {
    index: TokenField_index,
}
impl instructionVar163 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("lload"),
            DisplayElement::Literal("\t"),
            self.index.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 22i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let index = token_parser.TokenFieldindex();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1672:1"]
#[derive(Clone, Debug)]
struct instructionVar164 {}
impl instructionVar164 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lload_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 30i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1679:1"]
#[derive(Clone, Debug)]
struct instructionVar165 {}
impl instructionVar165 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lload_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 31i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1686:1"]
#[derive(Clone, Debug)]
struct instructionVar166 {}
impl instructionVar166 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lload_2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 32i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1693:1"]
#[derive(Clone, Debug)]
struct instructionVar167 {}
impl instructionVar167 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lload_3")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 33i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1700:1"]
#[derive(Clone, Debug)]
struct instructionVar168 {}
impl instructionVar168 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lmul")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 105i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1711:1"]
#[derive(Clone, Debug)]
struct instructionVar169 {}
impl instructionVar169 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lneg")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 117i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1730:1"]
#[derive(Clone, Debug)]
struct instructionVar170 {
    LookupSwitch_match: TableLookupSwitch_match,
    instruction: Box<Tableinstruction>,
}
impl instructionVar170 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.LookupSwitch_match.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.instruction.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let tmp = context_instance
            .register()
            .read_switch_num_disassembly()
            .wrapping_sub(1i64);
        context_instance
            .register_mut()
            .write_switch_num_disassembly(tmp);
        let mut sub_pattern_c39 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 0u64 as u32;
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 1i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c39(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let LookupSwitch_match = if let Some((len, table)) =
            TableLookupSwitch_match::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let instruction = if let Some((len, table)) = Tableinstruction::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u32);
            Box::new(table)
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
                LookupSwitch_match,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1735:1"]
#[derive(Clone, Debug)]
struct instructionVar171 {
    LookupSwitch_match: TableLookupSwitch_match,
}
impl instructionVar171 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("LookupSwitch_match")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let tmp = 0i64;
        context_instance
            .register_mut()
            .write_in_lookup_switch_disassembly(tmp);
        let mut sub_pattern_c27 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 0u64 as u32;
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 1i64
            {
                return None;
            }
            if context_instance.register().read_switch_num_disassembly() != 1i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c27(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let LookupSwitch_match = if let Some((len, table)) =
            TableLookupSwitch_match::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { LookupSwitch_match }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1757:1"]
#[derive(Clone, Debug)]
struct instructionVar172 {
    dolookupswitch: Tabledolookupswitch,
    instruction: Box<Tableinstruction>,
}
impl instructionVar172 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("lookupswitch"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.dolookupswitch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.instruction.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_in_lookup_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_table_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 171i64 {
            return None;
        }
        if context_instance.register().read_alignmentPad_disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let dolookupswitch = if let Some((len, table)) =
            Tabledolookupswitch::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let instruction = if let Some((len, table)) = Tableinstruction::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u32);
            Box::new(table)
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
                dolookupswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1761:1"]
#[derive(Clone, Debug)]
struct instructionVar173 {
    dolookupswitch: Tabledolookupswitch,
    instruction: Box<Tableinstruction>,
}
impl instructionVar173 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("lookupswitch"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.dolookupswitch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.instruction.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_in_lookup_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_table_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 171i64 {
            return None;
        }
        if context_instance.register().read_alignmentPad_disassembly() != 1i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad1 = token_parser.TokenFieldpad1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let dolookupswitch = if let Some((len, table)) =
            Tabledolookupswitch::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 0u64 as u32;
        let instruction = if let Some((len, table)) = Tableinstruction::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_3_len = block_3_len.max(len as u32);
            Box::new(table)
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
                dolookupswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1765:1"]
#[derive(Clone, Debug)]
struct instructionVar174 {
    dolookupswitch: Tabledolookupswitch,
    instruction: Box<Tableinstruction>,
}
impl instructionVar174 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("lookupswitch"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.dolookupswitch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.instruction.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_in_lookup_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_table_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 171i64 {
            return None;
        }
        if context_instance.register().read_alignmentPad_disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad1 = token_parser.TokenFieldpad1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad2 = token_parser.TokenFieldpad2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 0u64 as u32;
        let dolookupswitch = if let Some((len, table)) =
            Tabledolookupswitch::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_3_len = block_3_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 0u64 as u32;
        let instruction = if let Some((len, table)) = Tableinstruction::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_4_len = block_4_len.max(len as u32);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_4_len;
        tokens_current =
            &tokens_current[usize::try_from(block_4_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                dolookupswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2066:1"]
#[derive(Clone, Debug)]
struct instructionVar175 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar175 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = (self
            .indexbyte1
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | self.indexbyte2.disassembly());
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("wide_iload"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50197i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = (token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldindexbyte2().disassembly());
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2073:1"]
#[derive(Clone, Debug)]
struct instructionVar176 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar176 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = (self
            .indexbyte1
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | self.indexbyte2.disassembly());
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("wide_fload"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50199i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = (token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldindexbyte2().disassembly());
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2080:1"]
#[derive(Clone, Debug)]
struct instructionVar177 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar177 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = (self
            .indexbyte1
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | self.indexbyte2.disassembly());
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("wide_aload"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50201i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = (token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldindexbyte2().disassembly());
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2087:1"]
#[derive(Clone, Debug)]
struct instructionVar178 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar178 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = (self
            .indexbyte1
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | self.indexbyte2.disassembly());
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("wide_lload"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50198i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = (token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldindexbyte2().disassembly());
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2094:1"]
#[derive(Clone, Debug)]
struct instructionVar179 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar179 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = (self
            .indexbyte1
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | self.indexbyte2.disassembly());
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("wide_dload"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50200i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = (token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldindexbyte2().disassembly());
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2103:1"]
#[derive(Clone, Debug)]
struct instructionVar180 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar180 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = (self
            .indexbyte1
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | self.indexbyte2.disassembly());
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("wide_istore"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50230i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = (token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldindexbyte2().disassembly());
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2111:1"]
#[derive(Clone, Debug)]
struct instructionVar181 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar181 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = (self
            .indexbyte1
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | self.indexbyte2.disassembly());
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("wide_fstore"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50232i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = (token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldindexbyte2().disassembly());
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2119:1"]
#[derive(Clone, Debug)]
struct instructionVar182 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar182 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = (self
            .indexbyte1
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | self.indexbyte2.disassembly());
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("wide_astore"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50234i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = (token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldindexbyte2().disassembly());
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2127:1"]
#[derive(Clone, Debug)]
struct instructionVar183 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar183 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = (self
            .indexbyte1
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | self.indexbyte2.disassembly());
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("wide_lstore"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50231i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = (token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldindexbyte2().disassembly());
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2135:1"]
#[derive(Clone, Debug)]
struct instructionVar184 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar184 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = (self
            .indexbyte1
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | self.indexbyte2.disassembly());
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("wide_dstore"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50233i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = (token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldindexbyte2().disassembly());
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2144:1"]
#[derive(Clone, Debug)]
struct instructionVar185 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar185 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = (self
            .indexbyte1
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | self.indexbyte2.disassembly());
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("wide_ret"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50345i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c21(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = (token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldindexbyte2().disassembly());
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1769:1"]
#[derive(Clone, Debug)]
struct instructionVar186 {
    dolookupswitch: Tabledolookupswitch,
    instruction: Box<Tableinstruction>,
}
impl instructionVar186 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("lookupswitch"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.dolookupswitch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.instruction.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_in_lookup_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_table_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 171i64 {
            return None;
        }
        if context_instance.register().read_alignmentPad_disassembly() != 3i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad1 = token_parser.TokenFieldpad1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad2 = token_parser.TokenFieldpad2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad3 = token_parser.TokenFieldpad3();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 0u64 as u32;
        let dolookupswitch = if let Some((len, table)) =
            Tabledolookupswitch::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_4_len = block_4_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_4_len;
        tokens_current =
            &tokens_current[usize::try_from(block_4_len).unwrap()..];
        let mut block_5_len = 0u64 as u32;
        let instruction = if let Some((len, table)) = Tableinstruction::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_5_len = block_5_len.max(len as u32);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_5_len;
        tokens_current =
            &tokens_current[usize::try_from(block_5_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                dolookupswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1775:1"]
#[derive(Clone, Debug)]
struct instructionVar187 {}
impl instructionVar187 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("lor")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 129i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1786:1"]
#[derive(Clone, Debug)]
struct instructionVar188 {}
impl instructionVar188 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lrem")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 113i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1797:1"]
#[derive(Clone, Debug)]
struct instructionVar189 {}
impl instructionVar189 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lreturn")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 173i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1803:1"]
#[derive(Clone, Debug)]
struct instructionVar190 {}
impl instructionVar190 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lshl")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 121i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1814:1"]
#[derive(Clone, Debug)]
struct instructionVar191 {}
impl instructionVar191 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lshr")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 123i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1825:1"]
#[derive(Clone, Debug)]
struct instructionVar192 {
    index: TokenField_index,
}
impl instructionVar192 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("lstore"),
            DisplayElement::Literal("\t"),
            self.index.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 55i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let index = token_parser.TokenFieldindex();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1833:1"]
#[derive(Clone, Debug)]
struct instructionVar193 {}
impl instructionVar193 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lstore_0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 63i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1841:1"]
#[derive(Clone, Debug)]
struct instructionVar194 {}
impl instructionVar194 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lstore_1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 64i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1849:1"]
#[derive(Clone, Debug)]
struct instructionVar195 {}
impl instructionVar195 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lstore_2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 65i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1857:1"]
#[derive(Clone, Debug)]
struct instructionVar196 {}
impl instructionVar196 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lstore_3")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 66i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1865:1"]
#[derive(Clone, Debug)]
struct instructionVar197 {}
impl instructionVar197 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lsub")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 101i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1876:1"]
#[derive(Clone, Debug)]
struct instructionVar198 {}
impl instructionVar198 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lushr")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 125i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1887:1"]
#[derive(Clone, Debug)]
struct instructionVar199 {}
impl instructionVar199 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("lxor")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 131i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1898:1"]
#[derive(Clone, Debug)]
struct instructionVar200 {}
impl instructionVar200 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("monitorenter")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 194i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1905:1"]
#[derive(Clone, Debug)]
struct instructionVar201 {}
impl instructionVar201 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("monitorexit")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 195i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1912:1"]
#[derive(Clone, Debug)]
struct instructionVar202 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar202 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("multianewarray"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c27 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 197i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c27(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let dimensions = token_parser.TokenFielddimensions();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1917:1"]
#[derive(Clone, Debug)]
struct instructionVar203 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar203 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("new"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c16 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 187i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c16(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1924:1"]
#[derive(Clone, Debug)]
struct instructionVar204 {
    atype: TokenField_atype,
}
impl instructionVar204 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("newarray"),
            DisplayElement::Literal(" "),
            self.atype.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 188i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c21(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let atype = token_parser.TokenFieldatype();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { atype }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1934:1"]
#[derive(Clone, Debug)]
struct instructionVar205 {}
impl instructionVar205 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("nop")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 0i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1938:1"]
#[derive(Clone, Debug)]
struct instructionVar206 {}
impl instructionVar206 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("pop")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 87i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1943:1"]
#[derive(Clone, Debug)]
struct instructionVar207 {}
impl instructionVar207 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("pop2")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 88i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1948:1"]
#[derive(Clone, Debug)]
struct instructionVar208 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar208 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("putfield"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 181i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1953:1"]
#[derive(Clone, Debug)]
struct instructionVar209 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instructionVar209 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        calc_index = self
            .indexbyte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.indexbyte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("putstatic"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 179i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldindexbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1958:1"]
#[derive(Clone, Debug)]
struct instructionVar210 {
    index: TokenField_index,
}
impl instructionVar210 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("ret"),
            DisplayElement::Literal(" "),
            self.index.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 169i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let index = token_parser.TokenFieldindex();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1965:1"]
#[derive(Clone, Debug)]
struct instructionVar211 {}
impl instructionVar211 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("return")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 177i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1970:1"]
#[derive(Clone, Debug)]
struct instructionVar212 {}
impl instructionVar212 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("saload")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 53i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1983:1"]
#[derive(Clone, Debug)]
struct instructionVar213 {}
impl instructionVar213 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("sastore")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 86i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1995:1"]
#[derive(Clone, Debug)]
struct instructionVar214 {
    byte1: TokenField_byte1,
    byte2: TokenField_byte2,
}
impl instructionVar214 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_short: i64 = 0;
        calc_short = self
            .byte1
            .disassembly()
            .checked_shl(
                u32::try_from((8i64 | self.byte2.disassembly())).unwrap(),
            )
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("sipush"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_short),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_short: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 17i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c20(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let byte1 = token_parser.TokenFieldbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_short = token_parser
            .TokenFieldbyte1()
            .disassembly()
            .checked_shl(
                u32::try_from(
                    (8i64 | token_parser.TokenFieldbyte2().disassembly()),
                )
                .unwrap(),
            )
            .unwrap_or(0);
        let byte2 = token_parser.TokenFieldbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { byte1, byte2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2001:1"]
#[derive(Clone, Debug)]
struct instructionVar215 {}
impl instructionVar215 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("swap")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 95i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2023:1"]
#[derive(Clone, Debug)]
struct instructionVar216 {
    Switch_offset: TableSwitch_offset,
    instruction: Box<Tableinstruction>,
}
impl instructionVar216 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.Switch_offset.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.instruction.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let tmp = context_instance
            .register()
            .read_switch_num_disassembly()
            .wrapping_sub(1i64);
        context_instance
            .register_mut()
            .write_switch_num_disassembly(tmp);
        let mut sub_pattern_c34 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 0u64 as u32;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 1i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c34(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Switch_offset = if let Some((len, table)) =
            TableSwitch_offset::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let instruction = if let Some((len, table)) = Tableinstruction::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u32);
            Box::new(table)
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
                Switch_offset,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2028:1"]
#[derive(Clone, Debug)]
struct instructionVar217 {
    Switch_offset: TableSwitch_offset,
}
impl instructionVar217 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("Switch_offset")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u32;
        let tmp = 0i64;
        context_instance
            .register_mut()
            .write_in_table_switch_disassembly(tmp);
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 0u64 as u32;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 1i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance.register().read_switch_num_disassembly() != 0i64
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Switch_offset = if let Some((len, table)) =
            TableSwitch_offset::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Switch_offset }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2048:1"]
#[derive(Clone, Debug)]
struct instructionVar218 {
    dotableswitch: Tabledotableswitch,
    instruction: Box<Tableinstruction>,
}
impl instructionVar218 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("tableswitch"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.dotableswitch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.instruction.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_in_table_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_lookup_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 170i64 {
            return None;
        }
        if context_instance.register().read_alignmentPad_disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let dotableswitch = if let Some((len, table)) =
            Tabledotableswitch::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let instruction = if let Some((len, table)) = Tableinstruction::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u32);
            Box::new(table)
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
                dotableswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2052:1"]
#[derive(Clone, Debug)]
struct instructionVar219 {
    dotableswitch: Tabledotableswitch,
    instruction: Box<Tableinstruction>,
}
impl instructionVar219 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("tableswitch"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.dotableswitch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.instruction.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_in_table_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_lookup_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 170i64 {
            return None;
        }
        if context_instance.register().read_alignmentPad_disassembly() != 1i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad1 = token_parser.TokenFieldpad1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let dotableswitch = if let Some((len, table)) =
            Tabledotableswitch::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 0u64 as u32;
        let instruction = if let Some((len, table)) = Tableinstruction::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_3_len = block_3_len.max(len as u32);
            Box::new(table)
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
                dotableswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2056:1"]
#[derive(Clone, Debug)]
struct instructionVar220 {
    dotableswitch: Tabledotableswitch,
    instruction: Box<Tableinstruction>,
}
impl instructionVar220 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("tableswitch"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.dotableswitch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.instruction.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_in_table_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_lookup_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 170i64 {
            return None;
        }
        if context_instance.register().read_alignmentPad_disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad1 = token_parser.TokenFieldpad1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad2 = token_parser.TokenFieldpad2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 0u64 as u32;
        let dotableswitch = if let Some((len, table)) =
            Tabledotableswitch::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_3_len = block_3_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 0u64 as u32;
        let instruction = if let Some((len, table)) = Tableinstruction::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_4_len = block_4_len.max(len as u32);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_4_len;
        tokens_current =
            &tokens_current[usize::try_from(block_4_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                dotableswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2060:1"]
#[derive(Clone, Debug)]
struct instructionVar221 {
    dotableswitch: Tabledotableswitch,
    instruction: Box<Tableinstruction>,
}
impl instructionVar221 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("tableswitch"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.dotableswitch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.instruction.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_in_table_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_lookup_switch_disassembly()
            != 0i64
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 170i64 {
            return None;
        }
        if context_instance.register().read_alignmentPad_disassembly() != 3i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad1 = token_parser.TokenFieldpad1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad2 = token_parser.TokenFieldpad2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad3 = token_parser.TokenFieldpad3();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 0u64 as u32;
        let dotableswitch = if let Some((len, table)) =
            Tabledotableswitch::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_4_len = block_4_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_4_len;
        tokens_current =
            &tokens_current[usize::try_from(block_4_len).unwrap()..];
        let mut block_5_len = 0u64 as u32;
        let instruction = if let Some((len, table)) = Tableinstruction::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_5_len = block_5_len.max(len as u32);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_5_len;
        tokens_current =
            &tokens_current[usize::try_from(block_5_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                dotableswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2152:1"]
#[derive(Clone, Debug)]
struct instructionVar222 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
    constantbyte1: TokenField_constantbyte1,
    constantbyte2: TokenField_constantbyte2,
}
impl instructionVar222 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_index: i64 = 0;
        let mut calc_constant: i64 = 0;
        calc_index = (self
            .indexbyte1
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | self.indexbyte2.disassembly());
        calc_constant = (self
            .constantbyte1
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | self.constantbyte2.disassembly());
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("iinc_w"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
            DisplayElement::Literal(", "),
            DisplayElement::Number(true, calc_constant),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_index: i64 = 0;
        let mut calc_constant: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                != 0i64
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50308i64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c29(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = (token_parser
            .TokenFieldindexbyte1()
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldindexbyte2().disassembly());
        let indexbyte2 = token_parser.TokenFieldindexbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let constantbyte1 = token_parser.TokenFieldconstantbyte1();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_constant = (token_parser
            .TokenFieldconstantbyte1()
            .disassembly()
            .checked_shl(u32::try_from(8i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldconstantbyte2().disassembly());
        let constantbyte2 = token_parser.TokenFieldconstantbyte2();
        pattern_len += block_4_len;
        tokens_current =
            &tokens_current[usize::try_from(block_4_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
                constantbyte1,
                constantbyte2,
            },
        ))
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
    Var110(instructionVar110),
    Var111(instructionVar111),
    Var112(instructionVar112),
    Var113(instructionVar113),
    Var114(instructionVar114),
    Var115(instructionVar115),
    Var116(instructionVar116),
    Var117(instructionVar117),
    Var118(instructionVar118),
    Var119(instructionVar119),
    Var120(instructionVar120),
    Var121(instructionVar121),
    Var122(instructionVar122),
    Var123(instructionVar123),
    Var124(instructionVar124),
    Var125(instructionVar125),
    Var126(instructionVar126),
    Var127(instructionVar127),
    Var128(instructionVar128),
    Var129(instructionVar129),
    Var130(instructionVar130),
    Var131(instructionVar131),
    Var132(instructionVar132),
    Var133(instructionVar133),
    Var134(instructionVar134),
    Var135(instructionVar135),
    Var136(instructionVar136),
    Var137(instructionVar137),
    Var138(instructionVar138),
    Var139(instructionVar139),
    Var140(instructionVar140),
    Var141(instructionVar141),
    Var142(instructionVar142),
    Var143(instructionVar143),
    Var144(instructionVar144),
    Var145(instructionVar145),
    Var146(instructionVar146),
    Var147(instructionVar147),
    Var148(instructionVar148),
    Var149(instructionVar149),
    Var150(instructionVar150),
    Var151(instructionVar151),
    Var152(instructionVar152),
    Var153(instructionVar153),
    Var154(instructionVar154),
    Var155(instructionVar155),
    Var156(instructionVar156),
    Var157(instructionVar157),
    Var158(instructionVar158),
    Var159(instructionVar159),
    Var160(instructionVar160),
    Var161(instructionVar161),
    Var162(instructionVar162),
    Var163(instructionVar163),
    Var164(instructionVar164),
    Var165(instructionVar165),
    Var166(instructionVar166),
    Var167(instructionVar167),
    Var168(instructionVar168),
    Var169(instructionVar169),
    Var170(instructionVar170),
    Var171(instructionVar171),
    Var172(instructionVar172),
    Var173(instructionVar173),
    Var174(instructionVar174),
    Var175(instructionVar175),
    Var176(instructionVar176),
    Var177(instructionVar177),
    Var178(instructionVar178),
    Var179(instructionVar179),
    Var180(instructionVar180),
    Var181(instructionVar181),
    Var182(instructionVar182),
    Var183(instructionVar183),
    Var184(instructionVar184),
    Var185(instructionVar185),
    Var186(instructionVar186),
    Var187(instructionVar187),
    Var188(instructionVar188),
    Var189(instructionVar189),
    Var190(instructionVar190),
    Var191(instructionVar191),
    Var192(instructionVar192),
    Var193(instructionVar193),
    Var194(instructionVar194),
    Var195(instructionVar195),
    Var196(instructionVar196),
    Var197(instructionVar197),
    Var198(instructionVar198),
    Var199(instructionVar199),
    Var200(instructionVar200),
    Var201(instructionVar201),
    Var202(instructionVar202),
    Var203(instructionVar203),
    Var204(instructionVar204),
    Var205(instructionVar205),
    Var206(instructionVar206),
    Var207(instructionVar207),
    Var208(instructionVar208),
    Var209(instructionVar209),
    Var210(instructionVar210),
    Var211(instructionVar211),
    Var212(instructionVar212),
    Var213(instructionVar213),
    Var214(instructionVar214),
    Var215(instructionVar215),
    Var216(instructionVar216),
    Var217(instructionVar217),
    Var218(instructionVar218),
    Var219(instructionVar219),
    Var220(instructionVar220),
    Var221(instructionVar221),
    Var222(instructionVar222),
}
impl Tableinstruction {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
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
            Self::Var110(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var111(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var112(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var113(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var114(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var115(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var116(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var117(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var118(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var119(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var120(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var121(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var122(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var123(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var124(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var125(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var126(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var127(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var128(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var129(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var130(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var131(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var132(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var133(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var134(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var135(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var136(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var137(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var138(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var139(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var140(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var141(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var142(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var143(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var144(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var145(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var146(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var147(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var148(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var149(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var150(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var151(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var152(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var153(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var154(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var155(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var156(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var157(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var158(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var159(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var160(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var161(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var162(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var163(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var164(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var165(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var166(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var167(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var168(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var169(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var170(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var171(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var172(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var173(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var174(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var175(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var176(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var177(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var178(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var179(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var180(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var181(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var182(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var183(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var184(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var185(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var186(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var187(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var188(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var189(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var190(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var191(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var192(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var193(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var194(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var195(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var196(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var197(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var198(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var199(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var200(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var201(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var202(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var203(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var204(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var205(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var206(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var207(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var208(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var209(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var210(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var211(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var212(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var213(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var214(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var215(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var216(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var217(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var218(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var219(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var220(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var221(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var222(x) => x.display_extend(
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
        inst_start: u32,
    ) -> Option<(u32, Self)>
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
        if let Some((inst_len, parsed)) = instructionVar110::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var110(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar111::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var111(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar112::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var112(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar113::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var113(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar114::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var114(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar115::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var115(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar116::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var116(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar117::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var117(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar118::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var118(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar119::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var119(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar120::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var120(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar121::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var121(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar122::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var122(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar123::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var123(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar124::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var124(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar125::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var125(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar126::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var126(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar127::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var127(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar128::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var128(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar129::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var129(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar130::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var130(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar131::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var131(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar132::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var132(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar133::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var133(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar134::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var134(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar135::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var135(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar136::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var136(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar137::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var137(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar138::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var138(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar139::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var139(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar140::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var140(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar141::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var141(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar142::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var142(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar143::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var143(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar144::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var144(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar145::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var145(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar146::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var146(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar147::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var147(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar148::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var148(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar149::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var149(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar150::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var150(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar151::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var151(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar152::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var152(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar153::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var153(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar154::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var154(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar155::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var155(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar156::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var156(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar157::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var157(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar158::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var158(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar159::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var159(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar160::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var160(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar161::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var161(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar162::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var162(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar163::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var163(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar164::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var164(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar165::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var165(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar166::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var166(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar167::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var167(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar168::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var168(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar169::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var169(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar170::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var170(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar171::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var171(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar172::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var172(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar173::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var173(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar174::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var174(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar175::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var175(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar176::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var176(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar177::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var177(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar178::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var178(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar179::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var179(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar180::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var180(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar181::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var181(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar182::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var182(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar183::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var183(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar184::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var184(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar185::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var185(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar186::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var186(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar187::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var187(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar188::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var188(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar189::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var189(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar190::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var190(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar191::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var191(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar192::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var192(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar193::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var193(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar194::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var194(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar195::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var195(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar196::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var196(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar197::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var197(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar198::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var198(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar199::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var199(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar200::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var200(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar201::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var201(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar202::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var202(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar203::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var203(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar204::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var204(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar205::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var205(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar206::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var206(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar207::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var207(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar208::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var208(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar209::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var209(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar210::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var210(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar211::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var211(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar212::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var212(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar213::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var213(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar214::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var214(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar215::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var215(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar216::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var216(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar217::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var217(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar218::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var218(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar219::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var219(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar220::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var220(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar221::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var221(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar222::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var222(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:204:1"]
#[derive(Clone, Debug)]
struct BranchVar0 {
    branchbyte1: TokenField_branchbyte1,
    branchbyte2: TokenField_branchbyte2,
}
impl BranchVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_addr: i64 = 0;
        calc_addr = i64::try_from(inst_start).unwrap().wrapping_add(
            self.branchbyte1
                .disassembly()
                .checked_shl(
                    u32::try_from((8i64 | self.branchbyte2.disassembly()))
                        .unwrap(),
                )
                .unwrap_or(0),
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_addr)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_addr: i64 = 0;
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let branchbyte1 = token_parser.TokenFieldbranchbyte1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_addr = i64::try_from(inst_start).unwrap().wrapping_add(
            token_parser
                .TokenFieldbranchbyte1()
                .disassembly()
                .checked_shl(
                    u32::try_from(
                        (8i64
                            | token_parser
                                .TokenFieldbranchbyte2()
                                .disassembly()),
                    )
                    .unwrap(),
                )
                .unwrap_or(0),
        );
        let branchbyte2 = token_parser.TokenFieldbranchbyte2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                branchbyte1,
                branchbyte2,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableBranch {
    Var0(BranchVar0),
}
impl TableBranch {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            BranchVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:209:1"]
#[derive(Clone, Debug)]
struct Branch_wVar0 {
    branchbyte1: TokenField_branchbyte1,
    branchbyte2: TokenField_branchbyte2,
    branchbyte3: TokenField_branchbyte3,
    branchbyte4: TokenField_branchbyte4,
}
impl Branch_wVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_addr: i64 = 0;
        calc_addr = i64::try_from(inst_start).unwrap().wrapping_add(
            (((self
                .branchbyte1
                .disassembly()
                .checked_shl(u32::try_from(24i64).unwrap())
                .unwrap_or(0)
                | self
                    .branchbyte2
                    .disassembly()
                    .checked_shl(u32::try_from(16i64).unwrap())
                    .unwrap_or(0))
                | self
                    .branchbyte3
                    .disassembly()
                    .checked_shl(u32::try_from(8i64).unwrap())
                    .unwrap_or(0))
                | self.branchbyte4.disassembly()),
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_addr)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_addr: i64 = 0;
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let branchbyte1 = token_parser.TokenFieldbranchbyte1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let branchbyte2 = token_parser.TokenFieldbranchbyte2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let branchbyte3 = token_parser.TokenFieldbranchbyte3();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_addr = i64::try_from(inst_start).unwrap().wrapping_add(
            (((token_parser
                .TokenFieldbranchbyte1()
                .disassembly()
                .checked_shl(u32::try_from(24i64).unwrap())
                .unwrap_or(0)
                | token_parser
                    .TokenFieldbranchbyte2()
                    .disassembly()
                    .checked_shl(u32::try_from(16i64).unwrap())
                    .unwrap_or(0))
                | token_parser
                    .TokenFieldbranchbyte3()
                    .disassembly()
                    .checked_shl(u32::try_from(8i64).unwrap())
                    .unwrap_or(0))
                | token_parser.TokenFieldbranchbyte4().disassembly()),
        );
        let branchbyte4 = token_parser.TokenFieldbranchbyte4();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                branchbyte1,
                branchbyte2,
                branchbyte3,
                branchbyte4,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableBranch_w {
    Var0(Branch_wVar0),
}
impl TableBranch_w {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            Branch_wVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:214:1"]
#[derive(Clone, Debug)]
struct DefaultVar0 {
    defaultbyte1: TokenField_defaultbyte1,
    defaultbyte2: TokenField_defaultbyte2,
    defaultbyte3: TokenField_defaultbyte3,
    defaultbyte4: TokenField_defaultbyte4,
}
impl DefaultVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_addr: i64 = 0;
        calc_addr = i64::try_from(inst_start).unwrap().wrapping_add(
            (((self
                .defaultbyte1
                .disassembly()
                .checked_shl(u32::try_from(24i64).unwrap())
                .unwrap_or(0)
                | self
                    .defaultbyte2
                    .disassembly()
                    .checked_shl(u32::try_from(16i64).unwrap())
                    .unwrap_or(0))
                | self
                    .defaultbyte3
                    .disassembly()
                    .checked_shl(u32::try_from(8i64).unwrap())
                    .unwrap_or(0))
                | self.defaultbyte4.disassembly()),
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("default"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_addr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_addr: i64 = 0;
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let defaultbyte1 = token_parser.TokenFielddefaultbyte1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let defaultbyte2 = token_parser.TokenFielddefaultbyte2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let defaultbyte3 = token_parser.TokenFielddefaultbyte3();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_addr = i64::try_from(inst_start).unwrap().wrapping_add(
            (((token_parser
                .TokenFielddefaultbyte1()
                .disassembly()
                .checked_shl(u32::try_from(24i64).unwrap())
                .unwrap_or(0)
                | token_parser
                    .TokenFielddefaultbyte2()
                    .disassembly()
                    .checked_shl(u32::try_from(16i64).unwrap())
                    .unwrap_or(0))
                | token_parser
                    .TokenFielddefaultbyte3()
                    .disassembly()
                    .checked_shl(u32::try_from(8i64).unwrap())
                    .unwrap_or(0))
                | token_parser.TokenFielddefaultbyte4().disassembly()),
        );
        let defaultbyte4 = token_parser.TokenFielddefaultbyte4();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                defaultbyte1,
                defaultbyte2,
                defaultbyte3,
                defaultbyte4,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableDefault {
    Var0(DefaultVar0),
}
impl TableDefault {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            DefaultVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1725:1"]
#[derive(Clone, Debug)]
struct LookupSwitch_matchVar0 {
    matchbyte1: TokenField_matchbyte1,
    matchbyte2: TokenField_matchbyte2,
    matchbyte3: TokenField_matchbyte3,
    matchbyte4: TokenField_matchbyte4,
    offsetbyte1: TokenField_offsetbyte1,
    offsetbyte2: TokenField_offsetbyte2,
    offsetbyte3: TokenField_offsetbyte3,
    offsetbyte4: TokenField_offsetbyte4,
}
impl LookupSwitch_matchVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_match: i64 = 0;
        let mut calc__offset: i64 = 0;
        calc_match = (((self
            .matchbyte1
            .disassembly()
            .checked_shl(u32::try_from(24i64).unwrap())
            .unwrap_or(0)
            | self
                .matchbyte2
                .disassembly()
                .checked_shl(u32::try_from(16i64).unwrap())
                .unwrap_or(0))
            | self
                .matchbyte3
                .disassembly()
                .checked_shl(u32::try_from(8i64).unwrap())
                .unwrap_or(0))
            | self.matchbyte4.disassembly());
        calc__offset = i64::try_from(inst_start).unwrap().wrapping_add(
            (((self
                .offsetbyte1
                .disassembly()
                .checked_shl(u32::try_from(24i64).unwrap())
                .unwrap_or(0)
                | self
                    .offsetbyte2
                    .disassembly()
                    .checked_shl(u32::try_from(16i64).unwrap())
                    .unwrap_or(0))
                | self
                    .offsetbyte3
                    .disassembly()
                    .checked_shl(u32::try_from(8i64).unwrap())
                    .unwrap_or(0))
                | self.offsetbyte4.disassembly()),
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Number(true, calc_match),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc__offset),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_match: i64 = 0;
        let mut calc__offset: i64 = 0;
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let matchbyte1 = token_parser.TokenFieldmatchbyte1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let matchbyte2 = token_parser.TokenFieldmatchbyte2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let matchbyte3 = token_parser.TokenFieldmatchbyte3();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_match = (((token_parser
            .TokenFieldmatchbyte1()
            .disassembly()
            .checked_shl(u32::try_from(24i64).unwrap())
            .unwrap_or(0)
            | token_parser
                .TokenFieldmatchbyte2()
                .disassembly()
                .checked_shl(u32::try_from(16i64).unwrap())
                .unwrap_or(0))
            | token_parser
                .TokenFieldmatchbyte3()
                .disassembly()
                .checked_shl(u32::try_from(8i64).unwrap())
                .unwrap_or(0))
            | token_parser.TokenFieldmatchbyte4().disassembly());
        let matchbyte4 = token_parser.TokenFieldmatchbyte4();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let offsetbyte1 = token_parser.TokenFieldoffsetbyte1();
        pattern_len += block_4_len;
        tokens_current =
            &tokens_current[usize::try_from(block_4_len).unwrap()..];
        let mut block_5_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let offsetbyte2 = token_parser.TokenFieldoffsetbyte2();
        pattern_len += block_5_len;
        tokens_current =
            &tokens_current[usize::try_from(block_5_len).unwrap()..];
        let mut block_6_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let offsetbyte3 = token_parser.TokenFieldoffsetbyte3();
        pattern_len += block_6_len;
        tokens_current =
            &tokens_current[usize::try_from(block_6_len).unwrap()..];
        let mut block_7_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc__offset = i64::try_from(inst_start).unwrap().wrapping_add(
            (((token_parser
                .TokenFieldoffsetbyte1()
                .disassembly()
                .checked_shl(u32::try_from(24i64).unwrap())
                .unwrap_or(0)
                | token_parser
                    .TokenFieldoffsetbyte2()
                    .disassembly()
                    .checked_shl(u32::try_from(16i64).unwrap())
                    .unwrap_or(0))
                | token_parser
                    .TokenFieldoffsetbyte3()
                    .disassembly()
                    .checked_shl(u32::try_from(8i64).unwrap())
                    .unwrap_or(0))
                | token_parser.TokenFieldoffsetbyte4().disassembly()),
        );
        let offsetbyte4 = token_parser.TokenFieldoffsetbyte4();
        pattern_len += block_7_len;
        tokens_current =
            &tokens_current[usize::try_from(block_7_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                matchbyte1,
                matchbyte2,
                matchbyte3,
                matchbyte4,
                offsetbyte1,
                offsetbyte2,
                offsetbyte3,
                offsetbyte4,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableLookupSwitch_match {
    Var0(LookupSwitch_matchVar0),
}
impl TableLookupSwitch_match {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) = LookupSwitch_matchVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1741:1"]
#[derive(Clone, Debug)]
struct padSwitchVar0 {}
impl padSwitchVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(""), DisplayElement::Literal("")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if context_instance.register().read_alignmentPad_disassembly() != 3i64 {
            return None;
        }
        let op = token_parser.TokenFieldop();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad1 = token_parser.TokenFieldpad1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad2 = token_parser.TokenFieldpad2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad3 = token_parser.TokenFieldpad3();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1742:1"]
#[derive(Clone, Debug)]
struct padSwitchVar1 {}
impl padSwitchVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(""), DisplayElement::Literal("")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if context_instance.register().read_alignmentPad_disassembly() != 2i64 {
            return None;
        }
        let op = token_parser.TokenFieldop();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad1 = token_parser.TokenFieldpad1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad2 = token_parser.TokenFieldpad2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1743:1"]
#[derive(Clone, Debug)]
struct padSwitchVar2 {}
impl padSwitchVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(""), DisplayElement::Literal("")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if context_instance.register().read_alignmentPad_disassembly() != 1i64 {
            return None;
        }
        let op = token_parser.TokenFieldop();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let pad1 = token_parser.TokenFieldpad1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1744:1"]
#[derive(Clone, Debug)]
struct padSwitchVar3 {}
impl padSwitchVar3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(""), DisplayElement::Literal("")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if context_instance.register().read_alignmentPad_disassembly() != 0i64 {
            return None;
        }
        let op = token_parser.TokenFieldop();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TablepadSwitch {
    Var0(padSwitchVar0),
    Var1(padSwitchVar1),
    Var2(padSwitchVar2),
    Var3(padSwitchVar3),
}
impl TablepadSwitch {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
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
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            padSwitchVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            padSwitchVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            padSwitchVar2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            padSwitchVar3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:1747:1"]
#[derive(Clone, Debug)]
struct dolookupswitchVar0 {
    npairsbyte1: TokenField_npairsbyte1,
    npairsbyte2: TokenField_npairsbyte2,
    npairsbyte3: TokenField_npairsbyte3,
    npairsbyte4: TokenField_npairsbyte4,
    defaultbyte1: TokenField_defaultbyte1,
    defaultbyte2: TokenField_defaultbyte2,
    defaultbyte3: TokenField_defaultbyte3,
    defaultbyte4: TokenField_defaultbyte4,
}
impl dolookupswitchVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_npairs: i64 = 0;
        let mut calc__default: i64 = 0;
        calc_npairs = (((self
            .npairsbyte1
            .disassembly()
            .checked_shl(u32::try_from(24i64).unwrap())
            .unwrap_or(0)
            | self
                .npairsbyte2
                .disassembly()
                .checked_shl(u32::try_from(16i64).unwrap())
                .unwrap_or(0))
            | self
                .npairsbyte3
                .disassembly()
                .checked_shl(u32::try_from(8i64).unwrap())
                .unwrap_or(0))
            | self.npairsbyte4.disassembly());
        calc__default = i64::try_from(inst_start).unwrap().wrapping_add(
            (((self
                .defaultbyte1
                .disassembly()
                .checked_shl(u32::try_from(24i64).unwrap())
                .unwrap_or(0)
                | self
                    .defaultbyte2
                    .disassembly()
                    .checked_shl(u32::try_from(16i64).unwrap())
                    .unwrap_or(0))
                | self
                    .defaultbyte3
                    .disassembly()
                    .checked_shl(u32::try_from(8i64).unwrap())
                    .unwrap_or(0))
                | self.defaultbyte4.disassembly()),
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Number(true, calc__default),
            DisplayElement::Literal(", "),
            DisplayElement::Number(true, calc_npairs),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_npairs: i64 = 0;
        let mut calc__default: i64 = 0;
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let defaultbyte1 = token_parser.TokenFielddefaultbyte1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let defaultbyte2 = token_parser.TokenFielddefaultbyte2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let defaultbyte3 = token_parser.TokenFielddefaultbyte3();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let defaultbyte4 = token_parser.TokenFielddefaultbyte4();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let npairsbyte1 = token_parser.TokenFieldnpairsbyte1();
        pattern_len += block_4_len;
        tokens_current =
            &tokens_current[usize::try_from(block_4_len).unwrap()..];
        let mut block_5_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let npairsbyte2 = token_parser.TokenFieldnpairsbyte2();
        pattern_len += block_5_len;
        tokens_current =
            &tokens_current[usize::try_from(block_5_len).unwrap()..];
        let mut block_6_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let npairsbyte3 = token_parser.TokenFieldnpairsbyte3();
        pattern_len += block_6_len;
        tokens_current =
            &tokens_current[usize::try_from(block_6_len).unwrap()..];
        let mut block_7_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_npairs = (((token_parser
            .TokenFieldnpairsbyte1()
            .disassembly()
            .checked_shl(u32::try_from(24i64).unwrap())
            .unwrap_or(0)
            | token_parser
                .TokenFieldnpairsbyte2()
                .disassembly()
                .checked_shl(u32::try_from(16i64).unwrap())
                .unwrap_or(0))
            | token_parser
                .TokenFieldnpairsbyte3()
                .disassembly()
                .checked_shl(u32::try_from(8i64).unwrap())
                .unwrap_or(0))
            | token_parser.TokenFieldnpairsbyte4().disassembly());
        calc__default = i64::try_from(inst_start).unwrap().wrapping_add(
            (((token_parser
                .TokenFielddefaultbyte1()
                .disassembly()
                .checked_shl(u32::try_from(24i64).unwrap())
                .unwrap_or(0)
                | token_parser
                    .TokenFielddefaultbyte2()
                    .disassembly()
                    .checked_shl(u32::try_from(16i64).unwrap())
                    .unwrap_or(0))
                | token_parser
                    .TokenFielddefaultbyte3()
                    .disassembly()
                    .checked_shl(u32::try_from(8i64).unwrap())
                    .unwrap_or(0))
                | token_parser.TokenFielddefaultbyte4().disassembly()),
        );
        let tmp = calc_npairs;
        context_instance
            .register_mut()
            .write_switch_num_disassembly(tmp);
        let tmp = 1i64;
        context_instance
            .register_mut()
            .write_in_lookup_switch_disassembly(tmp);
        let npairsbyte4 = token_parser.TokenFieldnpairsbyte4();
        pattern_len += block_7_len;
        tokens_current =
            &tokens_current[usize::try_from(block_7_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                defaultbyte1,
                defaultbyte2,
                defaultbyte3,
                defaultbyte4,
                npairsbyte1,
                npairsbyte2,
                npairsbyte3,
                npairsbyte4,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tabledolookupswitch {
    Var0(dolookupswitchVar0),
}
impl Tabledolookupswitch {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) = dolookupswitchVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2016:1"]
#[derive(Clone, Debug)]
struct Switch_offsetVar0 {
    offsetbyte1: TokenField_offsetbyte1,
    offsetbyte2: TokenField_offsetbyte2,
    offsetbyte3: TokenField_offsetbyte3,
    offsetbyte4: TokenField_offsetbyte4,
}
impl Switch_offsetVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc__offset: i64 = 0;
        calc__offset = i64::try_from(inst_start).unwrap().wrapping_add(
            (((self
                .offsetbyte1
                .disassembly()
                .checked_shl(u32::try_from(24i64).unwrap())
                .unwrap_or(0)
                | self
                    .offsetbyte2
                    .disassembly()
                    .checked_shl(u32::try_from(16i64).unwrap())
                    .unwrap_or(0))
                | self
                    .offsetbyte3
                    .disassembly()
                    .checked_shl(u32::try_from(8i64).unwrap())
                    .unwrap_or(0))
                | self.offsetbyte4.disassembly()),
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc__offset)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc__offset: i64 = 0;
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let offsetbyte1 = token_parser.TokenFieldoffsetbyte1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let offsetbyte2 = token_parser.TokenFieldoffsetbyte2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let offsetbyte3 = token_parser.TokenFieldoffsetbyte3();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc__offset = i64::try_from(inst_start).unwrap().wrapping_add(
            (((token_parser
                .TokenFieldoffsetbyte1()
                .disassembly()
                .checked_shl(u32::try_from(24i64).unwrap())
                .unwrap_or(0)
                | token_parser
                    .TokenFieldoffsetbyte2()
                    .disassembly()
                    .checked_shl(u32::try_from(16i64).unwrap())
                    .unwrap_or(0))
                | token_parser
                    .TokenFieldoffsetbyte3()
                    .disassembly()
                    .checked_shl(u32::try_from(8i64).unwrap())
                    .unwrap_or(0))
                | token_parser.TokenFieldoffsetbyte4().disassembly()),
        );
        let offsetbyte4 = token_parser.TokenFieldoffsetbyte4();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                offsetbyte1,
                offsetbyte2,
                offsetbyte3,
                offsetbyte4,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableSwitch_offset {
    Var0(Switch_offsetVar0),
}
impl TableSwitch_offset {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) = Switch_offsetVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec:2032:1"]
#[derive(Clone, Debug)]
struct dotableswitchVar0 {
    lowbyte1: TokenField_lowbyte1,
    lowbyte2: TokenField_lowbyte2,
    lowbyte3: TokenField_lowbyte3,
    lowbyte4: TokenField_lowbyte4,
    highbyte1: TokenField_highbyte1,
    highbyte2: TokenField_highbyte2,
    highbyte3: TokenField_highbyte3,
    highbyte4: TokenField_highbyte4,
    Default: TableDefault,
}
impl dotableswitchVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_low: i64 = 0;
        let mut calc_high: i64 = 0;
        calc_low = (((self
            .lowbyte1
            .disassembly()
            .checked_shl(u32::try_from(24i64).unwrap())
            .unwrap_or(0)
            | self
                .lowbyte2
                .disassembly()
                .checked_shl(u32::try_from(16i64).unwrap())
                .unwrap_or(0))
            | self
                .lowbyte3
                .disassembly()
                .checked_shl(u32::try_from(8i64).unwrap())
                .unwrap_or(0))
            | self.lowbyte4.disassembly());
        calc_high = (((self
            .highbyte1
            .disassembly()
            .checked_shl(u32::try_from(24i64).unwrap())
            .unwrap_or(0)
            | self
                .highbyte2
                .disassembly()
                .checked_shl(u32::try_from(16i64).unwrap())
                .unwrap_or(0))
            | self
                .highbyte3
                .disassembly()
                .checked_shl(u32::try_from(8i64).unwrap())
                .unwrap_or(0))
            | self.highbyte4.disassembly());
        self.Default.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(", "),
            DisplayElement::Number(true, calc_low),
            DisplayElement::Literal(", "),
            DisplayElement::Number(true, calc_high),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_low: i64 = 0;
        let mut calc_high: i64 = 0;
        let mut block_0_len = 0u64 as u32;
        let Default = if let Some((len, table)) = TableDefault::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lowbyte1 = token_parser.TokenFieldlowbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lowbyte2 = token_parser.TokenFieldlowbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let lowbyte3 = token_parser.TokenFieldlowbyte3();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_low = (((token_parser
            .TokenFieldlowbyte1()
            .disassembly()
            .checked_shl(u32::try_from(24i64).unwrap())
            .unwrap_or(0)
            | token_parser
                .TokenFieldlowbyte2()
                .disassembly()
                .checked_shl(u32::try_from(16i64).unwrap())
                .unwrap_or(0))
            | token_parser
                .TokenFieldlowbyte3()
                .disassembly()
                .checked_shl(u32::try_from(8i64).unwrap())
                .unwrap_or(0))
            | token_parser.TokenFieldlowbyte4().disassembly());
        let lowbyte4 = token_parser.TokenFieldlowbyte4();
        pattern_len += block_4_len;
        tokens_current =
            &tokens_current[usize::try_from(block_4_len).unwrap()..];
        let mut block_5_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let highbyte1 = token_parser.TokenFieldhighbyte1();
        pattern_len += block_5_len;
        tokens_current =
            &tokens_current[usize::try_from(block_5_len).unwrap()..];
        let mut block_6_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let highbyte2 = token_parser.TokenFieldhighbyte2();
        pattern_len += block_6_len;
        tokens_current =
            &tokens_current[usize::try_from(block_6_len).unwrap()..];
        let mut block_7_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let highbyte3 = token_parser.TokenFieldhighbyte3();
        pattern_len += block_7_len;
        tokens_current =
            &tokens_current[usize::try_from(block_7_len).unwrap()..];
        let mut block_8_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_high = (((token_parser
            .TokenFieldhighbyte1()
            .disassembly()
            .checked_shl(u32::try_from(24i64).unwrap())
            .unwrap_or(0)
            | token_parser
                .TokenFieldhighbyte2()
                .disassembly()
                .checked_shl(u32::try_from(16i64).unwrap())
                .unwrap_or(0))
            | token_parser
                .TokenFieldhighbyte3()
                .disassembly()
                .checked_shl(u32::try_from(8i64).unwrap())
                .unwrap_or(0))
            | token_parser.TokenFieldhighbyte4().disassembly());
        let tmp = calc_low;
        context_instance
            .register_mut()
            .write_switch_low_disassembly(tmp);
        let tmp = calc_high.wrapping_sub(calc_low);
        context_instance
            .register_mut()
            .write_switch_num_disassembly(tmp);
        let tmp = calc_high;
        context_instance
            .register_mut()
            .write_switch_high_disassembly(tmp);
        let tmp = 1i64;
        context_instance
            .register_mut()
            .write_in_table_switch_disassembly(tmp);
        let highbyte4 = token_parser.TokenFieldhighbyte4();
        pattern_len += block_8_len;
        tokens_current =
            &tokens_current[usize::try_from(block_8_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                Default,
                lowbyte1,
                lowbyte2,
                lowbyte3,
                lowbyte4,
                highbyte1,
                highbyte2,
                highbyte3,
                highbyte4,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tabledotableswitch {
    Var0(dotableswitchVar0),
}
impl Tabledotableswitch {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) = dotableswitchVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
pub fn parse_instruction<T>(
    tokens: &[u8],
    context: &mut T,
    inst_start: u32,
    global_set: &mut impl GlobalSetTrait,
) -> Option<(u32, Vec<DisplayElement>)>
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
