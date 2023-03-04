use sleigh4rust::*;
pub type AddrType = u16;
pub trait GlobalSetTrait {
    fn set_doublebyte(&mut self, address: Option<u16>, value: i128);
}
#[derive(Default)]
pub struct GlobalSetDefault<C: ContextTrait>(
    pub std::collections::HashMap<AddrType, C>,
);
impl<C: ContextTrait> GlobalSetTrait for GlobalSetDefault<C> {
    fn set_doublebyte(&mut self, inst_start: Option<AddrType>, value: i128) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_doublebyte_disassembly(value)
                .unwrap();
            context
        });
    }
}
pub trait ContextregisterTrait:
    MemoryRead<AddressType = u16> + MemoryWrite
{
    fn read_doublebyte_raw(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u8::<true>(3, 0, 1)?;
        Ok(u8::try_from(work_value).unwrap())
    }
    fn write_doublebyte_raw(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u8::<true>(u8::from(param), 3, 0, 1)
    }
    fn read_doublebyte_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_doublebyte_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_doublebyte_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_doublebyte_raw(param as u8)
    }
    fn read_doublebyte_execution(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        self.read_doublebyte_raw()
    }
    fn write_doublebyte_execution(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_doublebyte_raw(param)
    }
    fn doublebyte_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_doublebyte_raw()?))
    }
}
pub trait ContextTrait: Default {
    type Typeregister: ContextregisterTrait;
    fn register(&self) -> &Self::Typeregister;
    fn register_mut(&mut self) -> &mut Self::Typeregister;
}
#[derive(Debug, Clone, Copy)]
pub struct ContextregisterStructDebug {
    pub chunk_0x0: [Option<bool>; 32],
}
impl Default for ContextregisterStructDebug {
    fn default() -> Self {
        Self {
            chunk_0x0: [None; 32],
        }
    }
}
impl ContextregisterStructDebug {
    fn read_bits(
        &self,
        addr: <Self as MemoryRead>::AddressType,
        buf: &mut [u8],
        mask: &[u8],
    ) -> Result<(), MemoryReadError<<Self as MemoryRead>::AddressType>> {
        assert_eq!(buf.len(), mask.len());
        let buf_len =
            <<Self as MemoryRead>::AddressType>::try_from(buf.len()).unwrap();
        let addr_end = addr + ((buf_len + 7) / 8);
        match (addr, addr_end) {
            (0..=3, 0..=4) => {
                let byte_offset = usize::try_from(addr - 0).unwrap();
                for ((buf_byte, mask_byte), byte) in
                    buf.iter_mut().zip(mask.iter()).zip(byte_offset..)
                {
                    for bit in (0..8)
                        .into_iter()
                        .filter(|bit| ((*mask_byte >> bit) & 1) != 0)
                    {
                        *buf_byte |= (self.chunk_0x0[(byte * 8) + bit].unwrap()
                            as u8)
                            << bit;
                    }
                }
            }
            (addr_start, addr_end) => {
                return Err(MemoryReadError::UnableToReadMemory(
                    addr_start, addr_end,
                ))
            }
        }
        Ok(())
    }
    fn write_bits(
        &mut self,
        addr: <Self as MemoryRead>::AddressType,
        buf: &[u8],
        mask: &[u8],
    ) -> Result<(), MemoryWriteError<<Self as MemoryRead>::AddressType>> {
        assert_eq!(buf.len(), mask.len());
        let buf_len =
            <<Self as MemoryRead>::AddressType>::try_from(buf.len()).unwrap();
        let addr_end = addr + ((buf_len + 7) / 8);
        match (addr, addr_end) {
            (0..=3, 0..=4) => {
                let byte_offset = usize::try_from(addr - 0).unwrap();
                for ((buf_byte, mask_byte), byte) in
                    buf.iter().zip(mask.iter()).zip(byte_offset..)
                {
                    for bit in (0..8)
                        .into_iter()
                        .filter(|bit| ((*mask_byte >> bit) & 1) != 0)
                    {
                        self.chunk_0x0[(byte * 8) + bit] =
                            Some(*buf_byte & (1 << bit) != 0);
                    }
                }
            }
            (addr_start, addr_end) => {
                return Err(MemoryWriteError::UnableToWriteMemory(
                    addr_start, addr_end,
                ))
            }
        }
        Ok(())
    }
}
impl ContextregisterTrait for ContextregisterStructDebug {}
impl MemoryRead for ContextregisterStructDebug {
    type AddressType = u16;
    fn read(
        &self,
        addr: <Self as MemoryRead>::AddressType,
        buf: &mut [u8],
    ) -> Result<(), MemoryReadError<<Self as MemoryRead>::AddressType>> {
        let mut inner_buf = vec![0xFF; buf.len()];
        self.read_bits(addr, buf, &mut inner_buf)
    }
    fn read_u8<const BIG_ENDIAN: bool>(
        &self,
        data_addr: <Self as MemoryRead>::AddressType,
        varnode_lsb: usize,
        data_bits: usize,
    ) -> Result<u8, MemoryReadError<<Self as MemoryRead>::AddressType>> {
        const TYPE_BITS: usize = <u8>::BITS as usize;
        const TYPE_BYTES: usize = TYPE_BITS / 8;
        assert!(data_bits > 0);
        let data_lsb = varnode_lsb % 8;
        let read_bytes = (data_bits + data_lsb + 7) / 8;
        assert!(read_bytes <= TYPE_BYTES);
        let data_start = if BIG_ENDIAN {
            TYPE_BYTES - read_bytes
        } else {
            0
        };
        let data_end = data_start + read_bytes;
        let mut data = [0u8; TYPE_BYTES];
        let mask = (<u8>::MAX >> (TYPE_BITS - data_bits)) << data_lsb;
        let mask = if BIG_ENDIAN {
            mask.to_be_bytes()
        } else {
            mask.to_le_bytes()
        };
        self.read_bits(
            data_addr,
            &mut data[data_start..data_end],
            &mask[data_start..data_end],
        )?;
        let data = if BIG_ENDIAN {
            <u8>::from_be_bytes(data)
        } else {
            <u8>::from_le_bytes(data)
        };
        let value_mask = <u8>::MAX >> (TYPE_BITS - data_bits);
        Ok((data >> data_lsb) & value_mask)
    }
    fn read_u16<const BIG_ENDIAN: bool>(
        &self,
        data_addr: <Self as MemoryRead>::AddressType,
        varnode_lsb: usize,
        data_bits: usize,
    ) -> Result<u16, MemoryReadError<<Self as MemoryRead>::AddressType>> {
        const TYPE_BITS: usize = <u16>::BITS as usize;
        const TYPE_BYTES: usize = TYPE_BITS / 8;
        assert!(data_bits > 0);
        let data_lsb = varnode_lsb % 8;
        let read_bytes = (data_bits + data_lsb + 7) / 8;
        assert!(read_bytes <= TYPE_BYTES);
        let data_start = if BIG_ENDIAN {
            TYPE_BYTES - read_bytes
        } else {
            0
        };
        let data_end = data_start + read_bytes;
        let mut data = [0u8; TYPE_BYTES];
        let mask = (<u16>::MAX >> (TYPE_BITS - data_bits)) << data_lsb;
        let mask = if BIG_ENDIAN {
            mask.to_be_bytes()
        } else {
            mask.to_le_bytes()
        };
        self.read_bits(
            data_addr,
            &mut data[data_start..data_end],
            &mask[data_start..data_end],
        )?;
        let data = if BIG_ENDIAN {
            <u16>::from_be_bytes(data)
        } else {
            <u16>::from_le_bytes(data)
        };
        let value_mask = <u16>::MAX >> (TYPE_BITS - data_bits);
        Ok((data >> data_lsb) & value_mask)
    }
    fn read_u32<const BIG_ENDIAN: bool>(
        &self,
        data_addr: <Self as MemoryRead>::AddressType,
        varnode_lsb: usize,
        data_bits: usize,
    ) -> Result<u32, MemoryReadError<<Self as MemoryRead>::AddressType>> {
        const TYPE_BITS: usize = <u32>::BITS as usize;
        const TYPE_BYTES: usize = TYPE_BITS / 8;
        assert!(data_bits > 0);
        let data_lsb = varnode_lsb % 8;
        let read_bytes = (data_bits + data_lsb + 7) / 8;
        assert!(read_bytes <= TYPE_BYTES);
        let data_start = if BIG_ENDIAN {
            TYPE_BYTES - read_bytes
        } else {
            0
        };
        let data_end = data_start + read_bytes;
        let mut data = [0u8; TYPE_BYTES];
        let mask = (<u32>::MAX >> (TYPE_BITS - data_bits)) << data_lsb;
        let mask = if BIG_ENDIAN {
            mask.to_be_bytes()
        } else {
            mask.to_le_bytes()
        };
        self.read_bits(
            data_addr,
            &mut data[data_start..data_end],
            &mask[data_start..data_end],
        )?;
        let data = if BIG_ENDIAN {
            <u32>::from_be_bytes(data)
        } else {
            <u32>::from_le_bytes(data)
        };
        let value_mask = <u32>::MAX >> (TYPE_BITS - data_bits);
        Ok((data >> data_lsb) & value_mask)
    }
    fn read_u64<const BIG_ENDIAN: bool>(
        &self,
        data_addr: <Self as MemoryRead>::AddressType,
        varnode_lsb: usize,
        data_bits: usize,
    ) -> Result<u64, MemoryReadError<<Self as MemoryRead>::AddressType>> {
        const TYPE_BITS: usize = <u64>::BITS as usize;
        const TYPE_BYTES: usize = TYPE_BITS / 8;
        assert!(data_bits > 0);
        let data_lsb = varnode_lsb % 8;
        let read_bytes = (data_bits + data_lsb + 7) / 8;
        assert!(read_bytes <= TYPE_BYTES);
        let data_start = if BIG_ENDIAN {
            TYPE_BYTES - read_bytes
        } else {
            0
        };
        let data_end = data_start + read_bytes;
        let mut data = [0u8; TYPE_BYTES];
        let mask = (<u64>::MAX >> (TYPE_BITS - data_bits)) << data_lsb;
        let mask = if BIG_ENDIAN {
            mask.to_be_bytes()
        } else {
            mask.to_le_bytes()
        };
        self.read_bits(
            data_addr,
            &mut data[data_start..data_end],
            &mask[data_start..data_end],
        )?;
        let data = if BIG_ENDIAN {
            <u64>::from_be_bytes(data)
        } else {
            <u64>::from_le_bytes(data)
        };
        let value_mask = <u64>::MAX >> (TYPE_BITS - data_bits);
        Ok((data >> data_lsb) & value_mask)
    }
    fn read_u128<const BIG_ENDIAN: bool>(
        &self,
        data_addr: <Self as MemoryRead>::AddressType,
        varnode_lsb: usize,
        data_bits: usize,
    ) -> Result<u128, MemoryReadError<<Self as MemoryRead>::AddressType>> {
        const TYPE_BITS: usize = <u128>::BITS as usize;
        const TYPE_BYTES: usize = TYPE_BITS / 8;
        assert!(data_bits > 0);
        let data_lsb = varnode_lsb % 8;
        let read_bytes = (data_bits + data_lsb + 7) / 8;
        assert!(read_bytes <= TYPE_BYTES);
        let data_start = if BIG_ENDIAN {
            TYPE_BYTES - read_bytes
        } else {
            0
        };
        let data_end = data_start + read_bytes;
        let mut data = [0u8; TYPE_BYTES];
        let mask = (<u128>::MAX >> (TYPE_BITS - data_bits)) << data_lsb;
        let mask = if BIG_ENDIAN {
            mask.to_be_bytes()
        } else {
            mask.to_le_bytes()
        };
        self.read_bits(
            data_addr,
            &mut data[data_start..data_end],
            &mask[data_start..data_end],
        )?;
        let data = if BIG_ENDIAN {
            <u128>::from_be_bytes(data)
        } else {
            <u128>::from_le_bytes(data)
        };
        let value_mask = <u128>::MAX >> (TYPE_BITS - data_bits);
        Ok((data >> data_lsb) & value_mask)
    }
}
impl MemoryWrite for ContextregisterStructDebug {
    fn write(
        &mut self,
        addr: <Self as MemoryRead>::AddressType,
        buf: &[u8],
    ) -> Result<(), MemoryWriteError<<Self as MemoryRead>::AddressType>> {
        let mut inner_buf = vec![0xFF; buf.len()];
        self.write_bits(addr, buf, &inner_buf)
    }
    fn write_u8<const BIG_ENDIAN: bool>(
        &mut self,
        value: u8,
        data_addr: <Self as MemoryRead>::AddressType,
        varnode_lsb: usize,
        data_bits: usize,
    ) -> Result<(), MemoryWriteError<<Self as MemoryRead>::AddressType>> {
        const TYPE_BITS: usize = <u8>::BITS as usize;
        const TYPE_BYTES: usize = TYPE_BITS / 8;
        assert!(data_bits > 0);
        let data_lsb = varnode_lsb % 8;
        let read_bytes = (data_bits + data_lsb + 7) / 8;
        assert!(read_bytes <= TYPE_BYTES);
        let mask = (<u8>::MAX >> (TYPE_BITS - data_bits)) << data_lsb;
        let mask_raw = if BIG_ENDIAN {
            mask.to_be_bytes()
        } else {
            mask.to_le_bytes()
        };
        let data_start = if BIG_ENDIAN {
            TYPE_BYTES - read_bytes
        } else {
            0
        };
        let data_end = data_start + read_bytes;
        let value = (value << data_lsb) & mask;
        let final_mem = if BIG_ENDIAN {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        };
        self.write_bits(
            data_addr,
            &final_mem[data_start..data_end],
            &mask_raw[data_start..data_end],
        )
    }
    fn write_u16<const BIG_ENDIAN: bool>(
        &mut self,
        value: u16,
        data_addr: <Self as MemoryRead>::AddressType,
        varnode_lsb: usize,
        data_bits: usize,
    ) -> Result<(), MemoryWriteError<<Self as MemoryRead>::AddressType>> {
        const TYPE_BITS: usize = <u16>::BITS as usize;
        const TYPE_BYTES: usize = TYPE_BITS / 8;
        assert!(data_bits > 0);
        let data_lsb = varnode_lsb % 8;
        let read_bytes = (data_bits + data_lsb + 7) / 8;
        assert!(read_bytes <= TYPE_BYTES);
        let mask = (<u16>::MAX >> (TYPE_BITS - data_bits)) << data_lsb;
        let mask_raw = if BIG_ENDIAN {
            mask.to_be_bytes()
        } else {
            mask.to_le_bytes()
        };
        let data_start = if BIG_ENDIAN {
            TYPE_BYTES - read_bytes
        } else {
            0
        };
        let data_end = data_start + read_bytes;
        let value = (value << data_lsb) & mask;
        let final_mem = if BIG_ENDIAN {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        };
        self.write_bits(
            data_addr,
            &final_mem[data_start..data_end],
            &mask_raw[data_start..data_end],
        )
    }
    fn write_u32<const BIG_ENDIAN: bool>(
        &mut self,
        value: u32,
        data_addr: <Self as MemoryRead>::AddressType,
        varnode_lsb: usize,
        data_bits: usize,
    ) -> Result<(), MemoryWriteError<<Self as MemoryRead>::AddressType>> {
        const TYPE_BITS: usize = <u32>::BITS as usize;
        const TYPE_BYTES: usize = TYPE_BITS / 8;
        assert!(data_bits > 0);
        let data_lsb = varnode_lsb % 8;
        let read_bytes = (data_bits + data_lsb + 7) / 8;
        assert!(read_bytes <= TYPE_BYTES);
        let mask = (<u32>::MAX >> (TYPE_BITS - data_bits)) << data_lsb;
        let mask_raw = if BIG_ENDIAN {
            mask.to_be_bytes()
        } else {
            mask.to_le_bytes()
        };
        let data_start = if BIG_ENDIAN {
            TYPE_BYTES - read_bytes
        } else {
            0
        };
        let data_end = data_start + read_bytes;
        let value = (value << data_lsb) & mask;
        let final_mem = if BIG_ENDIAN {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        };
        self.write_bits(
            data_addr,
            &final_mem[data_start..data_end],
            &mask_raw[data_start..data_end],
        )
    }
    fn write_u64<const BIG_ENDIAN: bool>(
        &mut self,
        value: u64,
        data_addr: <Self as MemoryRead>::AddressType,
        varnode_lsb: usize,
        data_bits: usize,
    ) -> Result<(), MemoryWriteError<<Self as MemoryRead>::AddressType>> {
        const TYPE_BITS: usize = <u64>::BITS as usize;
        const TYPE_BYTES: usize = TYPE_BITS / 8;
        assert!(data_bits > 0);
        let data_lsb = varnode_lsb % 8;
        let read_bytes = (data_bits + data_lsb + 7) / 8;
        assert!(read_bytes <= TYPE_BYTES);
        let mask = (<u64>::MAX >> (TYPE_BITS - data_bits)) << data_lsb;
        let mask_raw = if BIG_ENDIAN {
            mask.to_be_bytes()
        } else {
            mask.to_le_bytes()
        };
        let data_start = if BIG_ENDIAN {
            TYPE_BYTES - read_bytes
        } else {
            0
        };
        let data_end = data_start + read_bytes;
        let value = (value << data_lsb) & mask;
        let final_mem = if BIG_ENDIAN {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        };
        self.write_bits(
            data_addr,
            &final_mem[data_start..data_end],
            &mask_raw[data_start..data_end],
        )
    }
    fn write_u128<const BIG_ENDIAN: bool>(
        &mut self,
        value: u128,
        data_addr: <Self as MemoryRead>::AddressType,
        varnode_lsb: usize,
        data_bits: usize,
    ) -> Result<(), MemoryWriteError<<Self as MemoryRead>::AddressType>> {
        const TYPE_BITS: usize = <u128>::BITS as usize;
        const TYPE_BYTES: usize = TYPE_BITS / 8;
        assert!(data_bits > 0);
        let data_lsb = varnode_lsb % 8;
        let read_bytes = (data_bits + data_lsb + 7) / 8;
        assert!(read_bytes <= TYPE_BYTES);
        let mask = (<u128>::MAX >> (TYPE_BITS - data_bits)) << data_lsb;
        let mask_raw = if BIG_ENDIAN {
            mask.to_be_bytes()
        } else {
            mask.to_le_bytes()
        };
        let data_start = if BIG_ENDIAN {
            TYPE_BYTES - read_bytes
        } else {
            0
        };
        let data_end = data_start + read_bytes;
        let value = (value << data_lsb) & mask;
        let final_mem = if BIG_ENDIAN {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        };
        self.write_bits(
            data_addr,
            &final_mem[data_start..data_end],
            &mask_raw[data_start..data_end],
        )
    }
}
#[derive(Debug, Clone, Copy, Default)]
pub struct SpacesStruct {
    pub register: ContextregisterStructDebug,
}
impl ContextTrait for SpacesStruct {
    type Typeregister = ContextregisterStructDebug;
    fn register(&self) -> &Self::Typeregister {
        &self.register
    }
    fn register_mut(&mut self) -> &mut Self::Typeregister {
        &mut self.register
    }
}
fn meaning_number<T>(hex: bool, num: T) -> DisplayElement
where
    i128: TryFrom<T>,
    <i128 as TryFrom<T>>::Error: core::fmt::Debug,
{
    DisplayElement::Number(hex, i128::try_from(num).unwrap())
}
fn meaning_0_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_0_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_0_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::R0,
        1 => Register::R1,
        2 => Register::R2,
        3 => Register::R3,
        4 => Register::R4,
        5 => Register::R5,
        6 => Register::R6,
        7 => Register::R7,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_1_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_1_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_1_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::R0,
        1 => Register::R1,
        2 => Register::R2,
        3 => Register::R3,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_2_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_2_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_2_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::R4,
        1 => Register::R5,
        2 => Register::R6,
        3 => Register::R7,
        _ => unreachable!("Invalid Attach Value"),
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_target3_5(u8);
impl TokenField_target3_5 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_reg3_5(u8);
impl TokenField_reg3_5 {
    fn execution(&self) -> Register {
        meaning_0_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_0_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_target0_2(u8);
impl TokenField_target0_2 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_reg0_2(u8);
impl TokenField_reg0_2 {
    fn execution(&self) -> Register {
        meaning_0_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_0_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_reg0_1(u8);
impl TokenField_reg0_1 {
    fn execution(&self) -> Register {
        meaning_1_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_1_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_operation_size(u8);
impl TokenField_operation_size {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_branch_sign(u8);
impl TokenField_branch_sign {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_branch_external(u8);
impl TokenField_branch_external {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_branch_condition(u8);
impl TokenField_branch_condition {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_external_condition(u8);
impl TokenField_external_condition {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opcode6_9(u8);
impl TokenField_opcode6_9 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opcode3_9(u8);
impl TokenField_opcode3_9 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opcode2_9(u8);
impl TokenField_opcode2_9 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opcode1_9(u16);
impl TokenField_opcode1_9 {
    fn execution(&self) -> u16 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opcode0_9(u16);
impl TokenField_opcode0_9 {
    fn execution(&self) -> u16 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_target24_25(u8);
impl TokenField_target24_25 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_reg24_25(u8);
impl TokenField_reg24_25 {
    fn execution(&self) -> Register {
        meaning_2_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_2_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_address_hi(u8);
impl TokenField_address_hi {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_jump_type(u8);
impl TokenField_jump_type {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_address_lo(u16);
impl TokenField_address_lo {
    fn execution(&self) -> u16 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_value_lo(u8);
impl TokenField_value_lo {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_value_hi(u8);
impl TokenField_value_hi {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_addr16(u16);
impl TokenField_addr16 {
    fn execution(&self) -> u16 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
struct TokenParser<const LEN: usize>([u8; LEN]);
impl<const LEN: usize> MemoryRead for TokenParser<LEN> {
    type AddressType = usize;
    fn read(
        &self,
        addr: Self::AddressType,
        buf: &mut [u8],
    ) -> Result<(), MemoryReadError<Self::AddressType>> {
        let end = addr + buf.len();
        self.0
            .get(addr..end)
            .map(|src| buf.copy_from_slice(src))
            .ok_or(MemoryReadError::UnableToReadMemory(addr, end))
    }
}
impl<const LEN: usize> TokenParser<LEN> {
    fn new(data: &[u8]) -> Option<Self> {
        let token_slice: &[u8] = data.get(..LEN)?;
        let token_data = <[u8; LEN]>::try_from(token_slice).unwrap();
        Some(Self(token_data))
    }
    fn TokenFieldtarget3_5(&self) -> TokenField_target3_5 {
        let inner_value = self.read_u8::<true>(1, 3, 3).unwrap();
        TokenField_target3_5(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldreg3_5(&self) -> TokenField_reg3_5 {
        let inner_value = self.read_u8::<true>(1, 3, 3).unwrap();
        TokenField_reg3_5(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldtarget0_2(&self) -> TokenField_target0_2 {
        let inner_value = self.read_u8::<true>(1, 0, 3).unwrap();
        TokenField_target0_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldreg0_2(&self) -> TokenField_reg0_2 {
        let inner_value = self.read_u8::<true>(1, 0, 3).unwrap();
        TokenField_reg0_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldreg0_1(&self) -> TokenField_reg0_1 {
        let inner_value = self.read_u8::<true>(1, 0, 2).unwrap();
        TokenField_reg0_1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldoperation_size(&self) -> TokenField_operation_size {
        let inner_value = self.read_u8::<true>(1, 2, 1).unwrap();
        TokenField_operation_size(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbranch_sign(&self) -> TokenField_branch_sign {
        let inner_value = self.read_u8::<true>(1, 5, 1).unwrap();
        TokenField_branch_sign(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbranch_external(&self) -> TokenField_branch_external {
        let inner_value = self.read_u8::<true>(1, 4, 1).unwrap();
        TokenField_branch_external(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbranch_condition(&self) -> TokenField_branch_condition {
        let inner_value = self.read_u8::<true>(1, 0, 4).unwrap();
        TokenField_branch_condition(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldexternal_condition(&self) -> TokenField_external_condition {
        let inner_value = self.read_u8::<true>(1, 0, 4).unwrap();
        TokenField_external_condition(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode6_9(&self) -> TokenField_opcode6_9 {
        let inner_value = self.read_u16::<true>(0, 6, 4).unwrap();
        TokenField_opcode6_9(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode3_9(&self) -> TokenField_opcode3_9 {
        let inner_value = self.read_u16::<true>(0, 3, 7).unwrap();
        TokenField_opcode3_9(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode2_9(&self) -> TokenField_opcode2_9 {
        let inner_value = self.read_u16::<true>(0, 2, 8).unwrap();
        TokenField_opcode2_9(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode1_9(&self) -> TokenField_opcode1_9 {
        let inner_value = self.read_u16::<true>(0, 1, 9).unwrap();
        TokenField_opcode1_9(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode0_9(&self) -> TokenField_opcode0_9 {
        let inner_value = self.read_u16::<true>(0, 0, 10).unwrap();
        TokenField_opcode0_9(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldtarget24_25(&self) -> TokenField_target24_25 {
        let inner_value = self.read_u8::<true>(0, 0, 2).unwrap();
        TokenField_target24_25(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldreg24_25(&self) -> TokenField_reg24_25 {
        let inner_value = self.read_u8::<true>(0, 0, 2).unwrap();
        TokenField_reg24_25(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldaddress_hi(&self) -> TokenField_address_hi {
        let inner_value = self.read_u8::<true>(1, 2, 6).unwrap();
        TokenField_address_hi(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldjump_type(&self) -> TokenField_jump_type {
        let inner_value = self.read_u8::<true>(1, 0, 2).unwrap();
        TokenField_jump_type(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldaddress_lo(&self) -> TokenField_address_lo {
        let inner_value = self.read_u16::<true>(2, 0, 10).unwrap();
        TokenField_address_lo(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldvalue_lo(&self) -> TokenField_value_lo {
        let inner_value = self.read_u8::<true>(1, 0, 8).unwrap();
        TokenField_value_lo(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldvalue_hi(&self) -> TokenField_value_hi {
        let inner_value = self.read_u8::<true>(3, 0, 8).unwrap();
        TokenField_value_hi(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldimm16(&self) -> TokenField_imm16 {
        let inner_value = self.read_u16::<true>(0, 0, 16).unwrap();
        TokenField_imm16(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldaddr16(&self) -> TokenField_addr16 {
        let inner_value = self.read_u16::<true>(0, 0, 16).unwrap();
        TokenField_addr16(u16::try_from(inner_value).unwrap())
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    I,
    C,
    O,
    Z,
    S,
    contextreg,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::R0 => write!(f, "R0"),
            Self::R1 => write!(f, "R1"),
            Self::R2 => write!(f, "R2"),
            Self::R3 => write!(f, "R3"),
            Self::R4 => write!(f, "R4"),
            Self::R5 => write!(f, "R5"),
            Self::R6 => write!(f, "R6"),
            Self::R7 => write!(f, "R7"),
            Self::I => write!(f, "I"),
            Self::C => write!(f, "C"),
            Self::O => write!(f, "O"),
            Self::Z => write!(f, "Z"),
            Self::S => write!(f, "S"),
            Self::contextreg => write!(f, "contextreg"),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum DisplayElement {
    Literal(&'static str),
    Register(Register),
    Number(bool, i128),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:273:1, end:273:2))"]
#[derive(Clone, Debug)]
struct J_instructionVar0 {
    jmpdest16: Tablejmpdest16,
}
impl J_instructionVar0 {
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
            [DisplayElement::Literal("J"), DisplayElement::Literal(" ")];
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode0_9().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4u64 as u16;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldjump_type().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldtarget24_25().disassembly() != 3i128 {
            return None;
        }
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:277:1, end:277:2))"]
#[derive(Clone, Debug)]
struct JD_instructionVar1 {
    jmpdest16: Tablejmpdest16,
}
impl JD_instructionVar1 {
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
            [DisplayElement::Literal("JD"), DisplayElement::Literal(" ")];
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode0_9().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4u64 as u16;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldjump_type().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldtarget24_25().disassembly() != 3i128 {
            return None;
        }
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:282:1, end:282:2))"]
#[derive(Clone, Debug)]
struct JE_instructionVar2 {
    jmpdest16: Tablejmpdest16,
}
impl JE_instructionVar2 {
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
            [DisplayElement::Literal("JE"), DisplayElement::Literal(" ")];
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode0_9().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4u64 as u16;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldjump_type().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldtarget24_25().disassembly() != 3i128 {
            return None;
        }
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:293:1, end:293:2))"]
#[derive(Clone, Debug)]
struct JSR_instructionVar3 {
    reg24_25: TokenField_reg24_25,
    jmpdest16: Tablejmpdest16,
}
impl JSR_instructionVar3 {
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
            DisplayElement::Literal("JSR"),
            DisplayElement::Literal(" "),
            self.reg24_25.display(),
            DisplayElement::Literal(","),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode0_9().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4u64 as u16;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldjump_type().disassembly() != 0i128 {
            return None;
        }
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
        let reg24_25 = token_parser.TokenFieldreg24_25();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                jmpdest16,
                reg24_25,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:298:1, end:298:2))"]
#[derive(Clone, Debug)]
struct JSRD_instructionVar4 {
    reg24_25: TokenField_reg24_25,
    jmpdest16: Tablejmpdest16,
}
impl JSRD_instructionVar4 {
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
            DisplayElement::Literal("JSRD"),
            DisplayElement::Literal(" "),
            self.reg24_25.display(),
            DisplayElement::Literal(","),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode0_9().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4u64 as u16;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldjump_type().disassembly() != 2i128 {
            return None;
        }
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
        let reg24_25 = token_parser.TokenFieldreg24_25();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                jmpdest16,
                reg24_25,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:304:1, end:304:2))"]
#[derive(Clone, Debug)]
struct JSRE_instructionVar5 {
    reg24_25: TokenField_reg24_25,
    jmpdest16: Tablejmpdest16,
}
impl JSRE_instructionVar5 {
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
            DisplayElement::Literal("JSRE"),
            DisplayElement::Literal(" "),
            self.reg24_25.display(),
            DisplayElement::Literal(","),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode0_9().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4u64 as u16;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldjump_type().disassembly() != 1i128 {
            return None;
        }
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
        let reg24_25 = token_parser.TokenFieldreg24_25();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                jmpdest16,
                reg24_25,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:215:1, end:215:2))"]
#[derive(Clone, Debug)]
struct CLRC_instructionVar6 {}
impl CLRC_instructionVar6 {
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode0_9().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:250:1, end:250:2))"]
#[derive(Clone, Debug)]
struct DIS_instructionVar7 {}
impl DIS_instructionVar7 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("DIS")];
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
        if token_parser.TokenFieldopcode0_9().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:254:1, end:254:2))"]
#[derive(Clone, Debug)]
struct EIS_instructionVar8 {}
impl EIS_instructionVar8 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("EIS")];
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
        if token_parser.TokenFieldopcode0_9().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:263:1, end:263:2))"]
#[derive(Clone, Debug)]
struct HLT_instructionVar9 {}
impl HLT_instructionVar9 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("HLT")];
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
        if token_parser.TokenFieldopcode0_9().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:384:1, end:384:2))"]
#[derive(Clone, Debug)]
struct SDBD_instructionVar10 {}
impl SDBD_instructionVar10 {
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
        global_set.set_doublebyte(
            Some(inst_next),
            context.register().read_doublebyte_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("SDBD")];
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
        if token_parser.TokenFieldopcode0_9().disassembly() != 1i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_doublebyte_disassembly(tmp)
            .unwrap();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:387:1, end:387:2))"]
#[derive(Clone, Debug)]
struct SETC_instructionVar11 {}
impl SETC_instructionVar11 {
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode0_9().disassembly() != 7i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:411:1, end:411:2))"]
#[derive(Clone, Debug)]
struct TCI_instructionVar12 {}
impl TCI_instructionVar12 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("TCI")];
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
        if token_parser.TokenFieldopcode0_9().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:361:1, end:361:2))"]
#[derive(Clone, Debug)]
struct NOP_instructionVar13 {}
impl NOP_instructionVar13 {
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode1_9().disassembly() != 26i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:391:1, end:391:2))"]
#[derive(Clone, Debug)]
struct SIN_instructionVar14 {}
impl SIN_instructionVar14 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("SIN")];
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
        if token_parser.TokenFieldopcode1_9().disassembly() != 27i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:258:1, end:258:2))"]
#[derive(Clone, Debug)]
struct GSWD_instructionVar15 {
    reg0_1: TokenField_reg0_1,
}
impl GSWD_instructionVar15 {
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
            DisplayElement::Literal("GSWD"),
            DisplayElement::Literal(" "),
            self.reg0_1.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2_9().disassembly() != 12i128 {
            return None;
        }
        let reg0_1 = token_parser.TokenFieldreg0_1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:512:1, end:512:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar16 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl RLC_instructionVar16 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("RLC"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("2"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldoperation_size().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 10i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:524:1, end:524:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar17 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl RRC_instructionVar17 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("RRC"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("2"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldoperation_size().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 14i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:536:1, end:536:2))"]
#[derive(Clone, Debug)]
struct SAR_instructionVar18 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SAR_instructionVar18 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SAR"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("2"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldoperation_size().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 13i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:544:1, end:544:2))"]
#[derive(Clone, Debug)]
struct SARC_instructionVar19 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SARC_instructionVar19 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SARC"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("2"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldoperation_size().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 15i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:556:1, end:556:2))"]
#[derive(Clone, Debug)]
struct SLL_instructionVar20 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SLL_instructionVar20 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SLL"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("2"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldoperation_size().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 9i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:564:1, end:564:2))"]
#[derive(Clone, Debug)]
struct SLLC_instructionVar21 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SLLC_instructionVar21 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SLLC"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("2"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldoperation_size().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 11i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:576:1, end:576:2))"]
#[derive(Clone, Debug)]
struct SLR_instructionVar22 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SLR_instructionVar22 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SLR"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("2"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldoperation_size().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 12i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:584:1, end:584:2))"]
#[derive(Clone, Debug)]
struct SWAP_instructionVar23 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SWAP_instructionVar23 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SWAP"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("2"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldoperation_size().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 8i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:166:1, end:166:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar24 {
    addr16: TokenField_addr16,
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl ADD_instructionVar24 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("ADD"),
            DisplayElement::Literal(" "),
            self.addr16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 88i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let addr16 = token_parser.TokenFieldaddr16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
                addr16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:177:1, end:177:2))"]
#[derive(Clone, Debug)]
struct ADCR_instructionVar25 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl ADCR_instructionVar25 {
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
            DisplayElement::Literal("ADCR"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 5i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:188:1, end:188:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar26 {
    addr16: TokenField_addr16,
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl AND_instructionVar26 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("AND"),
            DisplayElement::Literal(" "),
            self.addr16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 112i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let addr16 = token_parser.TokenFieldaddr16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
                addr16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:225:1, end:225:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar27 {
    addr16: TokenField_addr16,
    reg0_2: TokenField_reg0_2,
}
impl CMP_instructionVar27 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("CMP"),
            DisplayElement::Literal(" "),
            self.addr16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 104i128 {
            return None;
        }
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let addr16 = token_parser.TokenFieldaddr16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_2, addr16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:238:1, end:238:2))"]
#[derive(Clone, Debug)]
struct COMR_instructionVar28 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl COMR_instructionVar28 {
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
            DisplayElement::Literal("COMR"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 3i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:244:1, end:244:2))"]
#[derive(Clone, Debug)]
struct DECR_instructionVar29 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl DECR_instructionVar29 {
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
            DisplayElement::Literal("DECR"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 2i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:267:1, end:267:2))"]
#[derive(Clone, Debug)]
struct INCR_instructionVar30 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl INCR_instructionVar30 {
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
            DisplayElement::Literal("INCR"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 1i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:316:1, end:316:2))"]
#[derive(Clone, Debug)]
struct MVI_instructionVar31 {
    addr16: TokenField_addr16,
    reg0_2: TokenField_reg0_2,
    checkbranch: Tablecheckbranch,
}
impl MVI_instructionVar31 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("MVI"),
            DisplayElement::Literal(" "),
            self.addr16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 80i128 {
            return None;
        }
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let addr16 = token_parser.TokenFieldaddr16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                reg0_2,
                addr16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:327:1, end:327:2))"]
#[derive(Clone, Debug)]
struct MVO_instructionVar32 {
    reg0_2: TokenField_reg0_2,
    addr16: TokenField_addr16,
}
impl MVO_instructionVar32 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("MVO"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.addr16.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 72i128 {
            return None;
        }
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let addr16 = token_parser.TokenFieldaddr16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_2, addr16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:345:1, end:345:2))"]
#[derive(Clone, Debug)]
struct MVOI_instructionVar33 {
    reg0_2: TokenField_reg0_2,
}
impl MVOI_instructionVar33 {
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
            DisplayElement::Literal("MVOI"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 79i128 {
            return None;
        }
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:350:1, end:350:2))"]
#[derive(Clone, Debug)]
struct NEGR_instructionVar34 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl NEGR_instructionVar34 {
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
            DisplayElement::Literal("NEGR"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 4i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:367:1, end:367:2))"]
#[derive(Clone, Debug)]
struct PSHR_instructionVar35 {
    reg0_2: TokenField_reg0_2,
}
impl PSHR_instructionVar35 {
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
            DisplayElement::Literal("PSHR"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldreg3_5().disassembly() != 6i128 {
            return None;
        }
        let reg0_2 = token_parser.TokenFieldreg0_2();
        let reg3_5 = token_parser.TokenFieldreg3_5();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:373:1, end:373:2))"]
#[derive(Clone, Debug)]
struct PULR_instructionVar36 {
    reg0_2: TokenField_reg0_2,
    impliedval16: Tableimpliedval16,
}
impl PULR_instructionVar36 {
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
            DisplayElement::Literal("PULR"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldreg3_5().disassembly() != 6i128 {
            return None;
        }
        let impliedval16 = if let Some((len, table)) = Tableimpliedval16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                impliedval16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:377:1, end:377:2))"]
#[derive(Clone, Debug)]
struct RSWD_instructionVar37 {
    reg0_2: TokenField_reg0_2,
}
impl RSWD_instructionVar37 {
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
            DisplayElement::Literal("RSWD"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 7i128 {
            return None;
        }
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:395:1, end:395:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar38 {
    addr16: TokenField_addr16,
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SUB_instructionVar38 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SUB"),
            DisplayElement::Literal(" "),
            self.addr16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 96i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let addr16 = token_parser.TokenFieldaddr16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
                addr16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:419:1, end:419:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar39 {
    addr16: TokenField_addr16,
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl XOR_instructionVar39 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("XOR"),
            DisplayElement::Literal(" "),
            self.addr16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 120i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let addr16 = token_parser.TokenFieldaddr16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
                addr16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:438:1, end:438:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar40 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl RLC_instructionVar40 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("RLC"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("1"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 10i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:448:1, end:448:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar41 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl RRC_instructionVar41 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("RRC"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("1"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 14i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:458:1, end:458:2))"]
#[derive(Clone, Debug)]
struct SAR_instructionVar42 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SAR_instructionVar42 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SAR"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("1"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 13i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:466:1, end:466:2))"]
#[derive(Clone, Debug)]
struct SARC_instructionVar43 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SARC_instructionVar43 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SARC"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("1"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 15i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:476:1, end:476:2))"]
#[derive(Clone, Debug)]
struct SLL_instructionVar44 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SLL_instructionVar44 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SLL"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("1"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 9i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:484:1, end:484:2))"]
#[derive(Clone, Debug)]
struct SLLC_instructionVar45 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SLLC_instructionVar45 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SLLC"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("1"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 11i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:494:1, end:494:2))"]
#[derive(Clone, Debug)]
struct SLR_instructionVar46 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SLR_instructionVar46 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SLR"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("1"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 12i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:502:1, end:502:2))"]
#[derive(Clone, Debug)]
struct SWAP_instructionVar47 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SWAP_instructionVar47 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SWAP"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("1"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 8i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:593:1, end:593:2))"]
#[derive(Clone, Debug)]
struct ADDI_instructionVar48 {
    imm16: TokenField_imm16,
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl ADDI_instructionVar48 {
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("ADDI"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
            self.imm16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 95i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
                imm16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:598:1, end:598:2))"]
#[derive(Clone, Debug)]
struct ANDI_instructionVar49 {
    imm16: TokenField_imm16,
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl ANDI_instructionVar49 {
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("ANDI"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
            self.imm16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 119i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
                imm16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:604:1, end:604:2))"]
#[derive(Clone, Debug)]
struct CMPI_instructionVar50 {
    imm16: TokenField_imm16,
    reg0_2: TokenField_reg0_2,
}
impl CMPI_instructionVar50 {
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("CMPI"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
            self.imm16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 111i128 {
            return None;
        }
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_2, imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:608:1, end:608:2))"]
#[derive(Clone, Debug)]
struct MVII_instructionVar51 {
    imm16: TokenField_imm16,
    reg0_2: TokenField_reg0_2,
    checkbranch: Tablecheckbranch,
}
impl MVII_instructionVar51 {
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("MVII"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
            self.imm16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 87i128 {
            return None;
        }
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                reg0_2,
                imm16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:613:1, end:613:2))"]
#[derive(Clone, Debug)]
struct SUBI_instructionVar52 {
    imm16: TokenField_imm16,
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SUBI_instructionVar52 {
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("SUBI"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
            self.imm16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 103i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
                imm16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:618:1, end:618:2))"]
#[derive(Clone, Debug)]
struct XORI_instructionVar53 {
    imm16: TokenField_imm16,
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl XORI_instructionVar53 {
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("XORI"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
            self.imm16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode3_9().disassembly() != 127i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
                imm16,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:626:1, end:626:2))"]
#[derive(Clone, Debug)]
struct ADDI_instructionVar54 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
    splitimm16: Tablesplitimm16,
}
impl ADDI_instructionVar54 {
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
            DisplayElement::Literal("ADDI"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.splitimm16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_doublebyte_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 95i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let splitimm16 = if let Some((len, table)) = Tablesplitimm16::parse(
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
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                splitimm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:631:1, end:631:2))"]
#[derive(Clone, Debug)]
struct ANDI_instructionVar55 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
    splitimm16: Tablesplitimm16,
}
impl ANDI_instructionVar55 {
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
            DisplayElement::Literal("ANDI"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.splitimm16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_doublebyte_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 119i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let splitimm16 = if let Some((len, table)) = Tablesplitimm16::parse(
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
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                splitimm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:637:1, end:637:2))"]
#[derive(Clone, Debug)]
struct CMPI_instructionVar56 {
    reg0_2: TokenField_reg0_2,
    splitimm16: Tablesplitimm16,
}
impl CMPI_instructionVar56 {
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
            DisplayElement::Literal("CMPI"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.splitimm16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_doublebyte_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 111i128 {
            return None;
        }
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let splitimm16 = if let Some((len, table)) = Tablesplitimm16::parse(
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
        Some((pattern_len, Self { splitimm16, reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:641:1, end:641:2))"]
#[derive(Clone, Debug)]
struct MVII_instructionVar57 {
    reg0_2: TokenField_reg0_2,
    checkbranch: Tablecheckbranch,
    splitimm16: Tablesplitimm16,
}
impl MVII_instructionVar57 {
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
            DisplayElement::Literal("MVII"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.splitimm16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_doublebyte_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 87i128 {
            return None;
        }
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let splitimm16 = if let Some((len, table)) = Tablesplitimm16::parse(
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
        Some((
            pattern_len,
            Self {
                checkbranch,
                splitimm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:646:1, end:646:2))"]
#[derive(Clone, Debug)]
struct SUBI_instructionVar58 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
    splitimm16: Tablesplitimm16,
}
impl SUBI_instructionVar58 {
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
            DisplayElement::Literal("SUBI"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.splitimm16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_doublebyte_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 103i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let splitimm16 = if let Some((len, table)) = Tablesplitimm16::parse(
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
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                splitimm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:651:1, end:651:2))"]
#[derive(Clone, Debug)]
struct XORI_instructionVar59 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
    splitimm16: Tablesplitimm16,
}
impl XORI_instructionVar59 {
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
            DisplayElement::Literal("XORI"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.splitimm16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_doublebyte_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldopcode3_9().disassembly() != 127i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let splitimm16 = if let Some((len, table)) = Tablesplitimm16::parse(
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
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                splitimm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:332:1, end:332:2))"]
#[derive(Clone, Debug)]
struct MVO_instructionVar60 {
    reg0_2: TokenField_reg0_2,
    reg3_5: TokenField_reg3_5,
    checkbranch: Tablecheckbranch,
}
impl MVO_instructionVar60 {
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("MVO"),
            DisplayElement::Literal("@"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg3_5.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 9i128 {
            return None;
        }
        let mut sub_pattern_c62 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldreg3_5().disassembly() == 4i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldreg3_5().disassembly() == 5i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c62(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        let reg3_5 = token_parser.TokenFieldreg3_5();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                reg0_2,
                reg3_5,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:364:1, end:364:2))"]
#[derive(Clone, Debug)]
struct NOPP_instructionVar61 {}
impl NOPP_instructionVar61 {
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
            [DisplayElement::Literal("NOPP")];
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
        if token_parser.TokenFieldopcode6_9().disassembly() != 8i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 8i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:211:1, end:211:2))"]
#[derive(Clone, Debug)]
struct BEXT_instructionVar62 {
    external_condition: TokenField_external_condition,
    branchdest16: Tablebranchdest16,
}
impl BEXT_instructionVar62 {
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
            DisplayElement::Literal("BEXT"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.branchdest16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.external_condition.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let mut sub_pattern_c43 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldopcode6_9().disassembly() != 8i128 {
                return None;
            }
            if token_parser.TokenFieldbranch_external().disassembly() != 1i128 {
                return None;
            }
            let external_condition =
                token_parser.TokenFieldexternal_condition();
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (external_condition), pattern_len))
        };
        let ((), (external_condition), sub_len) =
            sub_pattern_c43(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let branchdest16 = if let Some((len, table)) = Tablebranchdest16::parse(
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
        Some((
            pattern_len,
            Self {
                branchdest16,
                external_condition,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:219:1, end:219:2))"]
#[derive(Clone, Debug)]
struct CLRR_instructionVar63 {
    reg0_2: TokenField_reg0_2,
    checkbranch: Tablecheckbranch,
}
impl CLRR_instructionVar63 {
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
            DisplayElement::Literal("CLRR"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 7i128 {
            return None;
        }
        let mut sub_pattern_c45 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldtarget0_2().disassembly()
                != token_parser.TokenFieldtarget3_5().disassembly()
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c45(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:287:1, end:287:2))"]
#[derive(Clone, Debug)]
struct JR_instructionVar64 {
    reg3_5: TokenField_reg3_5,
}
impl JR_instructionVar64 {
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
            DisplayElement::Literal("JR"),
            DisplayElement::Literal(" "),
            self.reg3_5.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldreg0_2().disassembly() != 7i128 {
            return None;
        }
        let reg3_5 = token_parser.TokenFieldreg3_5();
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:415:1, end:415:2))"]
#[derive(Clone, Debug)]
struct TSTR_instructionVar65 {
    reg0_2: TokenField_reg0_2,
}
impl TSTR_instructionVar65 {
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
            DisplayElement::Literal("TSTR"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 2i128 {
            return None;
        }
        let mut sub_pattern_c45 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldtarget0_2().disassembly()
                != token_parser.TokenFieldtarget3_5().disassembly()
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c45(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:172:1, end:172:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar66 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
    impliedval16: Tableimpliedval16,
}
impl ADD_instructionVar66 {
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
            DisplayElement::Literal("ADD"),
            DisplayElement::Literal("@"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.impliedval16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 11i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let impliedval16 = if let Some((len, table)) = Tableimpliedval16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                impliedval16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:183:1, end:183:2))"]
#[derive(Clone, Debug)]
struct ADDR_instructionVar67 {
    reg3_5: TokenField_reg3_5,
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl ADDR_instructionVar67 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("ADDR"),
            DisplayElement::Literal(" "),
            self.reg3_5.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 3i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg3_5 = token_parser.TokenFieldreg3_5();
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg3_5,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:195:1, end:195:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar68 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
    impliedval16: Tableimpliedval16,
}
impl AND_instructionVar68 {
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
            DisplayElement::Literal("AND"),
            DisplayElement::Literal("@"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.impliedval16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 14i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let impliedval16 = if let Some((len, table)) = Tableimpliedval16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                impliedval16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:201:1, end:201:2))"]
#[derive(Clone, Debug)]
struct ANDR_instructionVar69 {
    reg3_5: TokenField_reg3_5,
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl ANDR_instructionVar69 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("ANDR"),
            DisplayElement::Literal(" "),
            self.reg3_5.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 6i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg3_5 = token_parser.TokenFieldreg3_5();
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg3_5,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:207:1, end:207:2))"]
#[derive(Clone, Debug)]
struct B_instructionVar70 {
    cc: Tablecc,
    branchdest16: Tablebranchdest16,
}
impl B_instructionVar70 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("B")];
        display.extend_from_slice(&extend);
        self.cc.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.branchdest16.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldopcode6_9().disassembly() != 8i128 {
                return None;
            }
            let cc = if let Some((len, table)) =
                Tablecc::parse(tokens, &mut context_instance, inst_start)
            {
                block_0_len = block_0_len.max(len as u16);
                table
            } else {
                return None;
            };
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((cc), (), pattern_len))
        };
        let ((mut cc), (), sub_len) =
            sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let branchdest16 = if let Some((len, table)) = Tablebranchdest16::parse(
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
        Some((pattern_len, Self { cc, branchdest16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:230:1, end:230:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar71 {
    reg0_2: TokenField_reg0_2,
    impliedval16: Tableimpliedval16,
}
impl CMP_instructionVar71 {
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
            DisplayElement::Literal("CMP"),
            DisplayElement::Literal("@"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.impliedval16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 13i128 {
            return None;
        }
        let impliedval16 = if let Some((len, table)) = Tableimpliedval16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                impliedval16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:234:1, end:234:2))"]
#[derive(Clone, Debug)]
struct CMPR_instructionVar72 {
    reg3_5: TokenField_reg3_5,
    reg0_2: TokenField_reg0_2,
}
impl CMPR_instructionVar72 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("CMPR"),
            DisplayElement::Literal(" "),
            self.reg3_5.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 5i128 {
            return None;
        }
        let reg3_5 = token_parser.TokenFieldreg3_5();
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5, reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:310:1, end:310:2))"]
#[derive(Clone, Debug)]
struct MOVR_instructionVar73 {
    reg3_5: TokenField_reg3_5,
    reg0_2: TokenField_reg0_2,
    checkbranch: Tablecheckbranch,
}
impl MOVR_instructionVar73 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("MOVR"),
            DisplayElement::Literal(" "),
            self.reg3_5.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 2i128 {
            return None;
        }
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        let reg3_5 = token_parser.TokenFieldreg3_5();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                reg0_2,
                reg3_5,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:322:1, end:322:2))"]
#[derive(Clone, Debug)]
struct MVI_instructionVar74 {
    reg0_2: TokenField_reg0_2,
    impliedval16: Tableimpliedval16,
    checkbranch: Tablecheckbranch,
}
impl MVI_instructionVar74 {
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
            DisplayElement::Literal("MVI"),
            DisplayElement::Literal("@"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.impliedval16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 10i128 {
            return None;
        }
        let impliedval16 = if let Some((len, table)) = Tableimpliedval16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                impliedval16,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:339:1, end:339:2))"]
#[derive(Clone, Debug)]
struct MVO_instructionVar75 {
    reg0_2: TokenField_reg0_2,
    reg3_5: TokenField_reg3_5,
    checkbranch: Tablecheckbranch,
}
impl MVO_instructionVar75 {
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("MVO"),
            DisplayElement::Literal("@"),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg3_5.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 9i128 {
            return None;
        }
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        let reg3_5 = token_parser.TokenFieldreg3_5();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                reg0_2,
                reg3_5,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:401:1, end:401:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar76 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
    impliedval16: Tableimpliedval16,
}
impl SUB_instructionVar76 {
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
            DisplayElement::Literal("SUB"),
            DisplayElement::Literal("@"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.impliedval16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 12i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let impliedval16 = if let Some((len, table)) = Tableimpliedval16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                impliedval16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:406:1, end:406:2))"]
#[derive(Clone, Debug)]
struct SUBR_instructionVar77 {
    reg3_5: TokenField_reg3_5,
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SUBR_instructionVar77 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("SUBR"),
            DisplayElement::Literal(" "),
            self.reg3_5.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 4i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg3_5 = token_parser.TokenFieldreg3_5();
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg3_5,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:426:1, end:426:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar78 {
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
    impliedval16: Tableimpliedval16,
}
impl XOR_instructionVar78 {
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
            DisplayElement::Literal("XOR"),
            DisplayElement::Literal("@"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.impliedval16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 15i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let impliedval16 = if let Some((len, table)) = Tableimpliedval16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                impliedval16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:432:1, end:432:2))"]
#[derive(Clone, Debug)]
struct XORR_instructionVar79 {
    reg3_5: TokenField_reg3_5,
    reg0_2: TokenField_reg0_2,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl XORR_instructionVar79 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("XORR"),
            DisplayElement::Literal(" "),
            self.reg3_5.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_2.display(),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode6_9().disassembly() != 7i128 {
            return None;
        }
        let regval0_2 = if let Some((len, table)) = Tableregval0_2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) = Tablecheckbranch::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let reg3_5 = token_parser.TokenFieldreg3_5();
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg3_5,
                reg0_2,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(J_instructionVar0),
    Var1(JD_instructionVar1),
    Var2(JE_instructionVar2),
    Var3(JSR_instructionVar3),
    Var4(JSRD_instructionVar4),
    Var5(JSRE_instructionVar5),
    Var6(CLRC_instructionVar6),
    Var7(DIS_instructionVar7),
    Var8(EIS_instructionVar8),
    Var9(HLT_instructionVar9),
    Var10(SDBD_instructionVar10),
    Var11(SETC_instructionVar11),
    Var12(TCI_instructionVar12),
    Var13(NOP_instructionVar13),
    Var14(SIN_instructionVar14),
    Var15(GSWD_instructionVar15),
    Var16(RLC_instructionVar16),
    Var17(RRC_instructionVar17),
    Var18(SAR_instructionVar18),
    Var19(SARC_instructionVar19),
    Var20(SLL_instructionVar20),
    Var21(SLLC_instructionVar21),
    Var22(SLR_instructionVar22),
    Var23(SWAP_instructionVar23),
    Var24(ADD_instructionVar24),
    Var25(ADCR_instructionVar25),
    Var26(AND_instructionVar26),
    Var27(CMP_instructionVar27),
    Var28(COMR_instructionVar28),
    Var29(DECR_instructionVar29),
    Var30(INCR_instructionVar30),
    Var31(MVI_instructionVar31),
    Var32(MVO_instructionVar32),
    Var33(MVOI_instructionVar33),
    Var34(NEGR_instructionVar34),
    Var35(PSHR_instructionVar35),
    Var36(PULR_instructionVar36),
    Var37(RSWD_instructionVar37),
    Var38(SUB_instructionVar38),
    Var39(XOR_instructionVar39),
    Var40(RLC_instructionVar40),
    Var41(RRC_instructionVar41),
    Var42(SAR_instructionVar42),
    Var43(SARC_instructionVar43),
    Var44(SLL_instructionVar44),
    Var45(SLLC_instructionVar45),
    Var46(SLR_instructionVar46),
    Var47(SWAP_instructionVar47),
    Var48(ADDI_instructionVar48),
    Var49(ANDI_instructionVar49),
    Var50(CMPI_instructionVar50),
    Var51(MVII_instructionVar51),
    Var52(SUBI_instructionVar52),
    Var53(XORI_instructionVar53),
    Var54(ADDI_instructionVar54),
    Var55(ANDI_instructionVar55),
    Var56(CMPI_instructionVar56),
    Var57(MVII_instructionVar57),
    Var58(SUBI_instructionVar58),
    Var59(XORI_instructionVar59),
    Var60(MVO_instructionVar60),
    Var61(NOPP_instructionVar61),
    Var62(BEXT_instructionVar62),
    Var63(CLRR_instructionVar63),
    Var64(JR_instructionVar64),
    Var65(TSTR_instructionVar65),
    Var66(ADD_instructionVar66),
    Var67(ADDR_instructionVar67),
    Var68(AND_instructionVar68),
    Var69(ANDR_instructionVar69),
    Var70(B_instructionVar70),
    Var71(CMP_instructionVar71),
    Var72(CMPR_instructionVar72),
    Var73(MOVR_instructionVar73),
    Var74(MVI_instructionVar74),
    Var75(MVO_instructionVar75),
    Var76(SUB_instructionVar76),
    Var77(SUBR_instructionVar77),
    Var78(XOR_instructionVar78),
    Var79(XORR_instructionVar79),
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
        if let Some((inst_len, parsed)) = J_instructionVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = JD_instructionVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = JE_instructionVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = JSR_instructionVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = JSRD_instructionVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) = JSRE_instructionVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) = CLRC_instructionVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) = DIS_instructionVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) = EIS_instructionVar8::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) = HLT_instructionVar9::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) = SDBD_instructionVar10::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) = SETC_instructionVar11::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) = TCI_instructionVar12::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) = NOP_instructionVar13::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) = SIN_instructionVar14::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) = GSWD_instructionVar15::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        if let Some((inst_len, parsed)) = RLC_instructionVar16::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var16(parsed)));
        }
        if let Some((inst_len, parsed)) = RRC_instructionVar17::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var17(parsed)));
        }
        if let Some((inst_len, parsed)) = SAR_instructionVar18::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var18(parsed)));
        }
        if let Some((inst_len, parsed)) = SARC_instructionVar19::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var19(parsed)));
        }
        if let Some((inst_len, parsed)) = SLL_instructionVar20::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var20(parsed)));
        }
        if let Some((inst_len, parsed)) = SLLC_instructionVar21::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var21(parsed)));
        }
        if let Some((inst_len, parsed)) = SLR_instructionVar22::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var22(parsed)));
        }
        if let Some((inst_len, parsed)) = SWAP_instructionVar23::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var23(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar24::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var24(parsed)));
        }
        if let Some((inst_len, parsed)) = ADCR_instructionVar25::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var25(parsed)));
        }
        if let Some((inst_len, parsed)) = AND_instructionVar26::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var26(parsed)));
        }
        if let Some((inst_len, parsed)) = CMP_instructionVar27::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var27(parsed)));
        }
        if let Some((inst_len, parsed)) = COMR_instructionVar28::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var28(parsed)));
        }
        if let Some((inst_len, parsed)) = DECR_instructionVar29::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var29(parsed)));
        }
        if let Some((inst_len, parsed)) = INCR_instructionVar30::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var30(parsed)));
        }
        if let Some((inst_len, parsed)) = MVI_instructionVar31::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var31(parsed)));
        }
        if let Some((inst_len, parsed)) = MVO_instructionVar32::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var32(parsed)));
        }
        if let Some((inst_len, parsed)) = MVOI_instructionVar33::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var33(parsed)));
        }
        if let Some((inst_len, parsed)) = NEGR_instructionVar34::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var34(parsed)));
        }
        if let Some((inst_len, parsed)) = PSHR_instructionVar35::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var35(parsed)));
        }
        if let Some((inst_len, parsed)) = PULR_instructionVar36::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var36(parsed)));
        }
        if let Some((inst_len, parsed)) = RSWD_instructionVar37::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var37(parsed)));
        }
        if let Some((inst_len, parsed)) = SUB_instructionVar38::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var38(parsed)));
        }
        if let Some((inst_len, parsed)) = XOR_instructionVar39::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var39(parsed)));
        }
        if let Some((inst_len, parsed)) = RLC_instructionVar40::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var40(parsed)));
        }
        if let Some((inst_len, parsed)) = RRC_instructionVar41::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var41(parsed)));
        }
        if let Some((inst_len, parsed)) = SAR_instructionVar42::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var42(parsed)));
        }
        if let Some((inst_len, parsed)) = SARC_instructionVar43::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var43(parsed)));
        }
        if let Some((inst_len, parsed)) = SLL_instructionVar44::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var44(parsed)));
        }
        if let Some((inst_len, parsed)) = SLLC_instructionVar45::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var45(parsed)));
        }
        if let Some((inst_len, parsed)) = SLR_instructionVar46::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var46(parsed)));
        }
        if let Some((inst_len, parsed)) = SWAP_instructionVar47::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var47(parsed)));
        }
        if let Some((inst_len, parsed)) = ADDI_instructionVar48::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var48(parsed)));
        }
        if let Some((inst_len, parsed)) = ANDI_instructionVar49::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var49(parsed)));
        }
        if let Some((inst_len, parsed)) = CMPI_instructionVar50::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var50(parsed)));
        }
        if let Some((inst_len, parsed)) = MVII_instructionVar51::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var51(parsed)));
        }
        if let Some((inst_len, parsed)) = SUBI_instructionVar52::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var52(parsed)));
        }
        if let Some((inst_len, parsed)) = XORI_instructionVar53::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var53(parsed)));
        }
        if let Some((inst_len, parsed)) = ADDI_instructionVar54::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var54(parsed)));
        }
        if let Some((inst_len, parsed)) = ANDI_instructionVar55::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var55(parsed)));
        }
        if let Some((inst_len, parsed)) = CMPI_instructionVar56::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var56(parsed)));
        }
        if let Some((inst_len, parsed)) = MVII_instructionVar57::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var57(parsed)));
        }
        if let Some((inst_len, parsed)) = SUBI_instructionVar58::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var58(parsed)));
        }
        if let Some((inst_len, parsed)) = XORI_instructionVar59::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var59(parsed)));
        }
        if let Some((inst_len, parsed)) = MVO_instructionVar60::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var60(parsed)));
        }
        if let Some((inst_len, parsed)) = NOPP_instructionVar61::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var61(parsed)));
        }
        if let Some((inst_len, parsed)) = BEXT_instructionVar62::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var62(parsed)));
        }
        if let Some((inst_len, parsed)) = CLRR_instructionVar63::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var63(parsed)));
        }
        if let Some((inst_len, parsed)) = JR_instructionVar64::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var64(parsed)));
        }
        if let Some((inst_len, parsed)) = TSTR_instructionVar65::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var65(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar66::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var66(parsed)));
        }
        if let Some((inst_len, parsed)) = ADDR_instructionVar67::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var67(parsed)));
        }
        if let Some((inst_len, parsed)) = AND_instructionVar68::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var68(parsed)));
        }
        if let Some((inst_len, parsed)) = ANDR_instructionVar69::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var69(parsed)));
        }
        if let Some((inst_len, parsed)) = B_instructionVar70::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var70(parsed)));
        }
        if let Some((inst_len, parsed)) = CMP_instructionVar71::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var71(parsed)));
        }
        if let Some((inst_len, parsed)) = CMPR_instructionVar72::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var72(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVR_instructionVar73::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var73(parsed)));
        }
        if let Some((inst_len, parsed)) = MVI_instructionVar74::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var74(parsed)));
        }
        if let Some((inst_len, parsed)) = MVO_instructionVar75::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var75(parsed)));
        }
        if let Some((inst_len, parsed)) = SUB_instructionVar76::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var76(parsed)));
        }
        if let Some((inst_len, parsed)) = SUBR_instructionVar77::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var77(parsed)));
        }
        if let Some((inst_len, parsed)) = XOR_instructionVar78::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var78(parsed)));
        }
        if let Some((inst_len, parsed)) = XORR_instructionVar79::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var79(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:58:1, end:58:10))"]
#[derive(Clone, Debug)]
struct jmpdest16Var0 {
    address_hi: TokenField_address_hi,
    address_lo: TokenField_address_lo,
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
        let mut calc_reloc: i128 = 0;
        calc_reloc = u32::try_from(10i128)
            .ok()
            .map(|shl| self.address_hi.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            .wrapping_add(self.address_lo.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_reloc)];
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
        let mut calc_reloc: i128 = 0;
        let mut block_0_len = 4u64 as u16;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        calc_reloc = u32::try_from(10i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldaddress_hi()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            .wrapping_add(token_parser.TokenFieldaddress_lo().disassembly());
        let address_hi = token_parser.TokenFieldaddress_hi();
        let address_lo = token_parser.TokenFieldaddress_lo();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                address_hi,
                address_lo,
            },
        ))
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:59:1, end:59:13))"]
#[derive(Clone, Debug)]
struct branchdest16Var0 {
    imm16: TokenField_imm16,
}
impl branchdest16Var0 {
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
        let mut calc_reloc: i128 = 0;
        calc_reloc = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(2i128)
            .wrapping_add(self.imm16.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_reloc)];
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
        let mut calc_reloc: i128 = 0;
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbranch_sign().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_reloc = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(2i128)
            .wrapping_add(token_parser.TokenFieldimm16().disassembly());
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:60:1, end:60:13))"]
#[derive(Clone, Debug)]
struct branchdest16Var1 {
    imm16: TokenField_imm16,
}
impl branchdest16Var1 {
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
        let mut calc_reloc: i128 = 0;
        calc_reloc = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(2i128)
            .wrapping_add((self.imm16.disassembly() ^ 65535i128));
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_reloc)];
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
        let mut calc_reloc: i128 = 0;
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbranch_sign().disassembly() != 1i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_reloc = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(2i128)
            .wrapping_add(
                (token_parser.TokenFieldimm16().disassembly() ^ 65535i128),
            );
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[derive(Clone, Debug)]
enum Tablebranchdest16 {
    Var0(branchdest16Var0),
    Var1(branchdest16Var1),
}
impl Tablebranchdest16 {
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
        if let Some((inst_len, parsed)) = branchdest16Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = branchdest16Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:61:1, end:61:11))"]
#[derive(Clone, Debug)]
struct splitimm16Var0 {
    value_hi: TokenField_value_hi,
    value_lo: TokenField_value_lo,
}
impl splitimm16Var0 {
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
        let mut calc_split: i128 = 0;
        calc_split = u32::try_from(8i128)
            .ok()
            .map(|shl| self.value_hi.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            .wrapping_add(self.value_lo.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_split)];
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
        let mut calc_split: i128 = 0;
        let mut block_0_len = 4u64 as u16;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        calc_split = u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldvalue_hi()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            .wrapping_add(token_parser.TokenFieldvalue_lo().disassembly());
        let value_hi = token_parser.TokenFieldvalue_hi();
        let value_lo = token_parser.TokenFieldvalue_lo();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { value_hi, value_lo }))
    }
}
#[derive(Clone, Debug)]
enum Tablesplitimm16 {
    Var0(splitimm16Var0),
}
impl Tablesplitimm16 {
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
        if let Some((inst_len, parsed)) = splitimm16Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:74:1, end:74:13))"]
#[derive(Clone, Debug)]
struct impliedval16Var0 {
    reg3_5: TokenField_reg3_5,
}
impl impliedval16Var0 {
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
        let extend: [DisplayElement; 1usize] = [self.reg3_5.display()];
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
        if token_parser.TokenFieldtarget3_5().disassembly() != 6i128 {
            return None;
        }
        if context_instance
            .register()
            .read_doublebyte_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        let reg3_5 = token_parser.TokenFieldreg3_5();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:89:1, end:89:13))"]
#[derive(Clone, Debug)]
struct impliedval16Var1 {
    reg3_5: TokenField_reg3_5,
}
impl impliedval16Var1 {
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
        let extend: [DisplayElement; 1usize] = [self.reg3_5.display()];
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
        if token_parser.TokenFieldtarget3_5().disassembly() != 6i128 {
            return None;
        }
        if context_instance
            .register()
            .read_doublebyte_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        let reg3_5 = token_parser.TokenFieldreg3_5();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:68:1, end:68:13))"]
#[derive(Clone, Debug)]
struct impliedval16Var2 {
    reg3_5: TokenField_reg3_5,
}
impl impliedval16Var2 {
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
        let extend: [DisplayElement; 1usize] = [self.reg3_5.display()];
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
        if context_instance
            .register()
            .read_doublebyte_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        let mut sub_pattern_c34 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 4i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 5i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c34(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let reg3_5 = token_parser.TokenFieldreg3_5();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:80:1, end:80:13))"]
#[derive(Clone, Debug)]
struct impliedval16Var3 {
    reg3_5: TokenField_reg3_5,
}
impl impliedval16Var3 {
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
        let extend: [DisplayElement; 1usize] = [self.reg3_5.display()];
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
        if context_instance
            .register()
            .read_doublebyte_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        let mut sub_pattern_c34 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 4i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 5i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c34(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let reg3_5 = token_parser.TokenFieldreg3_5();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:63:1, end:63:13))"]
#[derive(Clone, Debug)]
struct impliedval16Var4 {
    reg3_5: TokenField_reg3_5,
}
impl impliedval16Var4 {
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
        let extend: [DisplayElement; 1usize] = [self.reg3_5.display()];
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
        if context_instance
            .register()
            .read_doublebyte_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        let mut sub_pattern_c34 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 0i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 1i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 2i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 3i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 7i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c34(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let reg3_5 = token_parser.TokenFieldreg3_5();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:98:1, end:98:13))"]
#[derive(Clone, Debug)]
struct impliedval16Var5 {
    reg3_5: TokenField_reg3_5,
}
impl impliedval16Var5 {
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
        let extend: [DisplayElement; 1usize] = [self.reg3_5.display()];
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
        if context_instance
            .register()
            .read_doublebyte_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        let mut sub_pattern_c34 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 0i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 1i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 2i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 3i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                let token_parser = <TokenParser<2usize>>::new(tokens_param)?;
                if token_parser.TokenFieldtarget3_5().disassembly() == 7i128 {
                    return Some(((), (), u16::try_from(2u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c34(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let reg3_5 = token_parser.TokenFieldreg3_5();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[derive(Clone, Debug)]
enum Tableimpliedval16 {
    Var0(impliedval16Var0),
    Var1(impliedval16Var1),
    Var2(impliedval16Var2),
    Var3(impliedval16Var3),
    Var4(impliedval16Var4),
    Var5(impliedval16Var5),
}
impl Tableimpliedval16 {
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
        if let Some((inst_len, parsed)) = impliedval16Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = impliedval16Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = impliedval16Var2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = impliedval16Var3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = impliedval16Var4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) = impliedval16Var5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:104:1, end:104:12))"]
#[derive(Clone, Debug)]
struct checkbranchVar0 {}
impl checkbranchVar0 {
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
        if token_parser.TokenFieldreg0_2().disassembly() != 7i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:105:1, end:105:12))"]
#[derive(Clone, Debug)]
struct checkbranchVar1 {}
impl checkbranchVar1 {
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
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablecheckbranch {
    Var0(checkbranchVar0),
    Var1(checkbranchVar1),
}
impl Tablecheckbranch {
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
        if let Some((inst_len, parsed)) = checkbranchVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = checkbranchVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:106:1, end:106:10))"]
#[derive(Clone, Debug)]
struct regval0_2Var0 {}
impl regval0_2Var0 {
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
        if token_parser.TokenFieldreg0_2().disassembly() != 7i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:110:1, end:110:10))"]
#[derive(Clone, Debug)]
struct regval0_2Var1 {}
impl regval0_2Var1 {
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
        let reg0_2 = token_parser.TokenFieldreg0_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tableregval0_2 {
    Var0(regval0_2Var0),
    Var1(regval0_2Var1),
}
impl Tableregval0_2 {
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
            regval0_2Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            regval0_2Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:112:1, end:112:3))"]
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
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:113:1, end:113:3))"]
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 1i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:114:1, end:114:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("OV")];
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
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:115:1, end:115:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("PL")];
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
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:116:1, end:116:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("EQ")];
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
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:117:1, end:117:3))"]
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:118:1, end:118:3))"]
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:119:1, end:119:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("USC")];
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
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 7i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:120:1, end:120:3))"]
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 9i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:121:1, end:121:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NOV")];
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
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 10i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:122:1, end:122:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("MI")];
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
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 11i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:123:1, end:123:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NEQ")];
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
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 12i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:124:1, end:124:3))"]
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 13i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:125:1, end:125:3))"]
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 14i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:126:1, end:126:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ESC")];
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
        if token_parser.TokenFieldbranch_external().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbranch_condition().disassembly() != 15i128 {
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
