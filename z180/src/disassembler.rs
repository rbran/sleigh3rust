use sleigh4rust::*;
pub type AddrType = u16;
pub trait GlobalSetTrait {
    fn set_assume8bitIOSpace(&mut self, address: Option<u16>, value: i128);
}
#[derive(Default)]
pub struct GlobalSetDefault<C: ContextTrait>(
    pub std::collections::HashMap<AddrType, C>,
);
impl<C: ContextTrait> GlobalSetTrait for GlobalSetDefault<C> {
    fn set_assume8bitIOSpace(
        &mut self,
        inst_start: Option<AddrType>,
        value: i128,
    ) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_assume8bitIOSpace_disassembly(value)
                .unwrap();
            context
        });
    }
}
pub trait ContextregisterTrait:
    MemoryRead<AddressType = u8> + MemoryWrite
{
    fn read_assume8bitIOSpace_raw(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u8::<false>(0, 0, 1)?;
        Ok(u8::try_from(work_value).unwrap())
    }
    fn write_assume8bitIOSpace_raw(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u8::<false>(u8::from(param), 0, 0, 1)
    }
    fn read_assume8bitIOSpace_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_assume8bitIOSpace_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_assume8bitIOSpace_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_assume8bitIOSpace_raw(param as u8)
    }
    fn read_assume8bitIOSpace_execution(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        self.read_assume8bitIOSpace_raw()
    }
    fn write_assume8bitIOSpace_execution(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_assume8bitIOSpace_raw(param)
    }
    fn assume8bitIOSpace_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_assume8bitIOSpace_raw()?))
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
    type AddressType = u8;
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
        0 => Register::BC,
        1 => Register::DE,
        2 => Register::HL,
        3 => Register::SP,
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
        0 => Register::BC,
        1 => Register::DE,
        2 => Register::IX,
        3 => Register::SP,
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
        0 => Register::BC,
        1 => Register::DE,
        2 => Register::HL,
        3 => Register::AF,
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
        0 => Register::BC,
        1 => Register::DE,
        2 => Register::IY,
        3 => Register::SP,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_4_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_4_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_4_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::B,
        1 => Register::C,
        2 => Register::D,
        3 => Register::E,
        4 => Register::H,
        5 => Register::L,
        7 => Register::A,
        _ => unreachable!("Invalid Attach Value"),
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op0_8(u8);
impl TokenField_op0_8 {
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
struct TokenField_op6_2(u8);
impl TokenField_op6_2 {
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
struct TokenField_dRegPair4_2(u8);
impl TokenField_dRegPair4_2 {
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
struct TokenField_pRegPair4_2(u8);
impl TokenField_pRegPair4_2 {
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
struct TokenField_sRegPair4_2(u8);
impl TokenField_sRegPair4_2 {
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
struct TokenField_qRegPair4_2(u8);
impl TokenField_qRegPair4_2 {
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
struct TokenField_rRegPair4_2(u8);
impl TokenField_rRegPair4_2 {
    fn execution(&self) -> Register {
        meaning_3_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_3_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_reg3_3(u8);
impl TokenField_reg3_3 {
    fn execution(&self) -> Register {
        meaning_4_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_4_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_bits3_3(u8);
impl TokenField_bits3_3 {
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
struct TokenField_bits0_4(u8);
impl TokenField_bits0_4 {
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
struct TokenField_reg0_3(u8);
impl TokenField_reg0_3 {
    fn execution(&self) -> Register {
        meaning_4_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_4_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_bits0_3(u8);
impl TokenField_bits0_3 {
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
struct TokenField_imm8(u8);
impl TokenField_imm8 {
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
struct TokenField_sign8(u8);
impl TokenField_sign8 {
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
struct TokenField_simm8(i8);
impl TokenField_simm8 {
    fn execution(&self) -> i8 {
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
struct TokenField_timm4(u8);
impl TokenField_timm4 {
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
struct TokenField_sign16(u8);
impl TokenField_sign16 {
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
struct TokenField_simm16(i16);
impl TokenField_simm16 {
    fn execution(&self) -> i16 {
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
    fn TokenFieldop0_8(&self) -> TokenField_op0_8 {
        let inner_value = self.read_u8::<false>(0, 0, 8).unwrap();
        TokenField_op0_8(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop6_2(&self) -> TokenField_op6_2 {
        let inner_value = self.read_u8::<false>(0, 6, 2).unwrap();
        TokenField_op6_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFielddRegPair4_2(&self) -> TokenField_dRegPair4_2 {
        let inner_value = self.read_u8::<false>(0, 4, 2).unwrap();
        TokenField_dRegPair4_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldpRegPair4_2(&self) -> TokenField_pRegPair4_2 {
        let inner_value = self.read_u8::<false>(0, 4, 2).unwrap();
        TokenField_pRegPair4_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsRegPair4_2(&self) -> TokenField_sRegPair4_2 {
        let inner_value = self.read_u8::<false>(0, 4, 2).unwrap();
        TokenField_sRegPair4_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldqRegPair4_2(&self) -> TokenField_qRegPair4_2 {
        let inner_value = self.read_u8::<false>(0, 4, 2).unwrap();
        TokenField_qRegPair4_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrRegPair4_2(&self) -> TokenField_rRegPair4_2 {
        let inner_value = self.read_u8::<false>(0, 4, 2).unwrap();
        TokenField_rRegPair4_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldreg3_3(&self) -> TokenField_reg3_3 {
        let inner_value = self.read_u8::<false>(0, 3, 3).unwrap();
        TokenField_reg3_3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbits3_3(&self) -> TokenField_bits3_3 {
        let inner_value = self.read_u8::<false>(0, 3, 3).unwrap();
        TokenField_bits3_3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbits0_4(&self) -> TokenField_bits0_4 {
        let inner_value = self.read_u8::<false>(0, 0, 4).unwrap();
        TokenField_bits0_4(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldreg0_3(&self) -> TokenField_reg0_3 {
        let inner_value = self.read_u8::<false>(0, 0, 3).unwrap();
        TokenField_reg0_3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbits0_3(&self) -> TokenField_bits0_3 {
        let inner_value = self.read_u8::<false>(0, 0, 3).unwrap();
        TokenField_bits0_3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldimm8(&self) -> TokenField_imm8 {
        let inner_value = self.read_u8::<false>(0, 0, 8).unwrap();
        TokenField_imm8(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsign8(&self) -> TokenField_sign8 {
        let inner_value = self.read_u8::<false>(0, 7, 1).unwrap();
        TokenField_sign8(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsimm8(&self) -> TokenField_simm8 {
        let inner_value = self.read_i8::<false>(0, 0, 8).unwrap();
        TokenField_simm8(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldtimm4(&self) -> TokenField_timm4 {
        let inner_value = self.read_u8::<false>(1, 4, 4).unwrap();
        TokenField_timm4(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldimm16(&self) -> TokenField_imm16 {
        let inner_value = self.read_u16::<false>(0, 0, 16).unwrap();
        TokenField_imm16(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldsign16(&self) -> TokenField_sign16 {
        let inner_value = self.read_u8::<false>(1, 7, 1).unwrap();
        TokenField_sign16(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsimm16(&self) -> TokenField_simm16 {
        let inner_value = self.read_i16::<false>(0, 0, 16).unwrap();
        TokenField_simm16(i16::try_from(inner_value).unwrap())
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    F,
    A,
    C,
    B,
    E,
    D,
    L,
    H,
    I,
    R,
    AF,
    BC,
    DE,
    HL,
    F_,
    A_,
    C_,
    B_,
    E_,
    D_,
    L_,
    H_,
    AF_,
    BC_,
    DE_,
    HL_,
    PC,
    SP,
    IX,
    IY,
    rCBAR,
    rCBR,
    rBBR,
    DECOMPILE_MODE,
    contextreg,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::F => write!(f, "F"),
            Self::A => write!(f, "A"),
            Self::C => write!(f, "C"),
            Self::B => write!(f, "B"),
            Self::E => write!(f, "E"),
            Self::D => write!(f, "D"),
            Self::L => write!(f, "L"),
            Self::H => write!(f, "H"),
            Self::I => write!(f, "I"),
            Self::R => write!(f, "R"),
            Self::AF => write!(f, "AF"),
            Self::BC => write!(f, "BC"),
            Self::DE => write!(f, "DE"),
            Self::HL => write!(f, "HL"),
            Self::F_ => write!(f, "F_"),
            Self::A_ => write!(f, "A_"),
            Self::C_ => write!(f, "C_"),
            Self::B_ => write!(f, "B_"),
            Self::E_ => write!(f, "E_"),
            Self::D_ => write!(f, "D_"),
            Self::L_ => write!(f, "L_"),
            Self::H_ => write!(f, "H_"),
            Self::AF_ => write!(f, "AF_"),
            Self::BC_ => write!(f, "BC_"),
            Self::DE_ => write!(f, "DE_"),
            Self::HL_ => write!(f, "HL_"),
            Self::PC => write!(f, "PC"),
            Self::SP => write!(f, "SP"),
            Self::IX => write!(f, "IX"),
            Self::IY => write!(f, "IY"),
            Self::rCBAR => write!(f, "rCBAR"),
            Self::rCBR => write!(f, "rCBR"),
            Self::rBBR => write!(f, "rBBR"),
            Self::DECOMPILE_MODE => write!(f, "DECOMPILE_MODE"),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1360:1, end:1360:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar0 {
    ixMem8: TableixMem8,
}
impl RLC_instructionVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RLC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1370:1, end:1370:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar1 {
    iyMem8: TableiyMem8,
}
impl RLC_instructionVar1 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RLC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1404:1, end:1404:2))"]
#[derive(Clone, Debug)]
struct RL_instructionVar2 {
    ixMem8: TableixMem8,
}
impl RL_instructionVar2 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 22i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1416:1, end:1416:2))"]
#[derive(Clone, Debug)]
struct RL_instructionVar3 {
    iyMem8: TableiyMem8,
}
impl RL_instructionVar3 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 22i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1449:1, end:1449:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar4 {
    ixMem8: TableixMem8,
}
impl RRC_instructionVar4 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RRC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 14i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1460:1, end:1460:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar5 {
    iyMem8: TableiyMem8,
}
impl RRC_instructionVar5 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RRC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 14i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1494:1, end:1494:2))"]
#[derive(Clone, Debug)]
struct RR_instructionVar6 {
    ixMem8: TableixMem8,
}
impl RR_instructionVar6 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 30i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1506:1, end:1506:2))"]
#[derive(Clone, Debug)]
struct RR_instructionVar7 {
    iyMem8: TableiyMem8,
}
impl RR_instructionVar7 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 30i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1539:1, end:1539:2))"]
#[derive(Clone, Debug)]
struct SLA_instructionVar8 {
    ixMem8: TableixMem8,
}
impl SLA_instructionVar8 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("SLA"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 38i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1550:1, end:1550:2))"]
#[derive(Clone, Debug)]
struct SLA_instructionVar9 {
    iyMem8: TableiyMem8,
}
impl SLA_instructionVar9 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("SLA"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 38i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1582:1, end:1582:2))"]
#[derive(Clone, Debug)]
struct SRA_instructionVar10 {
    ixMem8: TableixMem8,
}
impl SRA_instructionVar10 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("SRA"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 46i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1593:1, end:1593:2))"]
#[derive(Clone, Debug)]
struct SRA_instructionVar11 {
    iyMem8: TableiyMem8,
}
impl SRA_instructionVar11 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("SRA"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 46i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1625:1, end:1625:2))"]
#[derive(Clone, Debug)]
struct SRL_instructionVar12 {
    ixMem8: TableixMem8,
}
impl SRL_instructionVar12 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("SRL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 62i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1636:1, end:1636:2))"]
#[derive(Clone, Debug)]
struct SRL_instructionVar13 {
    iyMem8: TableiyMem8,
}
impl SRL_instructionVar13 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("SRL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 62i128 {
            return None;
        }
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1699:1, end:1699:2))"]
#[derive(Clone, Debug)]
struct BIT_instructionVar14 {
    bits3_3: TokenField_bits3_3,
    ixMem8: TableixMem8,
}
impl BIT_instructionVar14 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BIT"),
            DisplayElement::Literal(" "),
            self.bits3_3.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        let bits3_3 = token_parser.TokenFieldbits3_3();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8, bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1707:1, end:1707:2))"]
#[derive(Clone, Debug)]
struct BIT_instructionVar15 {
    bits3_3: TokenField_bits3_3,
    iyMem8: TableiyMem8,
}
impl BIT_instructionVar15 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BIT"),
            DisplayElement::Literal(" "),
            self.bits3_3.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        let bits3_3 = token_parser.TokenFieldbits3_3();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8, bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1728:1, end:1728:2))"]
#[derive(Clone, Debug)]
struct SET_instructionVar16 {
    bits3_3: TokenField_bits3_3,
    ixMem8: TableixMem8,
}
impl SET_instructionVar16 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SET"),
            DisplayElement::Literal(" "),
            self.bits3_3.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        if token_parser.TokenFieldop6_2().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        let bits3_3 = token_parser.TokenFieldbits3_3();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8, bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1734:1, end:1734:2))"]
#[derive(Clone, Debug)]
struct SET_instructionVar17 {
    bits3_3: TokenField_bits3_3,
    iyMem8: TableiyMem8,
}
impl SET_instructionVar17 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SET"),
            DisplayElement::Literal(" "),
            self.bits3_3.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        if token_parser.TokenFieldop6_2().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        let bits3_3 = token_parser.TokenFieldbits3_3();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8, bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1753:1, end:1753:2))"]
#[derive(Clone, Debug)]
struct RES_instructionVar18 {
    bits3_3: TokenField_bits3_3,
    ixMem8: TableixMem8,
}
impl RES_instructionVar18 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RES"),
            DisplayElement::Literal(" "),
            self.bits3_3.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        if token_parser.TokenFieldop6_2().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        let bits3_3 = token_parser.TokenFieldbits3_3();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8, bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1759:1, end:1759:2))"]
#[derive(Clone, Debug)]
struct RES_instructionVar19 {
    bits3_3: TokenField_bits3_3,
    iyMem8: TableiyMem8,
}
impl RES_instructionVar19 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RES"),
            DisplayElement::Literal(" "),
            self.bits3_3.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        if token_parser.TokenFieldop6_2().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        let bits3_3 = token_parser.TokenFieldbits3_3();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8, bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:388:1, end:388:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar20 {
    reg3_3: TokenField_reg3_3,
    ixMem8: TableixMem8,
}
impl LD_instructionVar20 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            self.reg3_3.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() == 6i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        let reg3_3 = token_parser.TokenFieldreg3_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        Some((pattern_len, Self { ixMem8, reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:392:1, end:392:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar21 {
    reg3_3: TokenField_reg3_3,
    iyMem8: TableiyMem8,
}
impl LD_instructionVar21 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            self.reg3_3.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() == 6i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        let reg3_3 = token_parser.TokenFieldreg3_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        Some((pattern_len, Self { iyMem8, reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:400:1, end:400:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar22 {
    reg0_3: TokenField_reg0_3,
    ixMem8: TableixMem8,
}
impl LD_instructionVar22 {
    fn display_extend<T>(
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
        self.ixMem8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.reg0_3.display()];
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() == 6i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        Some((pattern_len, Self { ixMem8, reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:404:1, end:404:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar23 {
    reg0_3: TokenField_reg0_3,
    iyMem8: TableiyMem8,
}
impl LD_instructionVar23 {
    fn display_extend<T>(
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
        self.iyMem8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.reg0_3.display()];
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() == 6i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        Some((pattern_len, Self { iyMem8, reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:413:1, end:413:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar24 {
    imm8: TokenField_imm8,
    ixMem8: TableixMem8,
}
impl LD_instructionVar24 {
    fn display_extend<T>(
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
        self.ixMem8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.imm8.display()];
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8, imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:417:1, end:417:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar25 {
    imm8: TokenField_imm8,
    iyMem8: TableiyMem8,
}
impl LD_instructionVar25 {
    fn display_extend<T>(
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
        self.iyMem8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.imm8.display()];
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_3_len;
        tokens_current =
            &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8, imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:445:1, end:445:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar26 {}
impl LD_instructionVar26 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::I),
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 87i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:453:1, end:453:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar27 {}
impl LD_instructionVar27 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::R),
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 95i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:461:1, end:461:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar28 {}
impl LD_instructionVar28 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::I),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::A),
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 71i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:465:1, end:465:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar29 {}
impl LD_instructionVar29 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::R),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::A),
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 79i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:473:1, end:473:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar30 {
    imm16: TokenField_imm16,
}
impl LD_instructionVar30 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IX),
            DisplayElement::Literal(","),
            self.imm16.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 33i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:477:1, end:477:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar31 {
    imm16: TokenField_imm16,
}
impl LD_instructionVar31 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IY),
            DisplayElement::Literal(","),
            self.imm16.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 33i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:489:1, end:489:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar32 {
    Mem16: TableMem16,
}
impl LD_instructionVar32 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IX),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.Mem16.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 42i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { Mem16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:493:1, end:493:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar33 {
    Mem16: TableMem16,
}
impl LD_instructionVar33 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IY),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.Mem16.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 42i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { Mem16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:505:1, end:505:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar34 {
    Mem16: TableMem16,
}
impl LD_instructionVar34 {
    fn display_extend<T>(
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
        self.Mem16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::IX),
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 34i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { Mem16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:509:1, end:509:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar35 {
    Mem16: TableMem16,
}
impl LD_instructionVar35 {
    fn display_extend<T>(
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
        self.Mem16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::IY),
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 34i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { Mem16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:517:1, end:517:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar36 {}
impl LD_instructionVar36 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::SP),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::IX),
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 249i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:521:1, end:521:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar37 {}
impl LD_instructionVar37 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::SP),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::IY),
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 249i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:529:1, end:529:2))"]
#[derive(Clone, Debug)]
struct PUSH_instructionVar38 {}
impl PUSH_instructionVar38 {
    fn display_extend<T>(
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
            DisplayElement::Literal("PUSH"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IX),
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 229i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:533:1, end:533:2))"]
#[derive(Clone, Debug)]
struct PUSH_instructionVar39 {}
impl PUSH_instructionVar39 {
    fn display_extend<T>(
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
            DisplayElement::Literal("PUSH"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IY),
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 229i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:541:1, end:541:2))"]
#[derive(Clone, Debug)]
struct POP_instructionVar40 {}
impl POP_instructionVar40 {
    fn display_extend<T>(
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
            DisplayElement::Literal("POP"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IX),
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 225i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:546:1, end:546:2))"]
#[derive(Clone, Debug)]
struct POP_instructionVar41 {}
impl POP_instructionVar41 {
    fn display_extend<T>(
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
            DisplayElement::Literal("POP"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IY),
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 225i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:578:1, end:578:2))"]
#[derive(Clone, Debug)]
struct EX_instructionVar42 {}
impl EX_instructionVar42 {
    fn display_extend<T>(
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
            DisplayElement::Literal("EX"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::SP),
            DisplayElement::Literal("),"),
            DisplayElement::Register(Register::IX),
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 227i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:582:1, end:582:2))"]
#[derive(Clone, Debug)]
struct EX_instructionVar43 {}
impl EX_instructionVar43 {
    fn display_extend<T>(
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
            DisplayElement::Literal("EX"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::SP),
            DisplayElement::Literal("),"),
            DisplayElement::Register(Register::IY),
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 227i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:587:1, end:587:2))"]
#[derive(Clone, Debug)]
struct LDI_instructionVar44 {}
impl LDI_instructionVar44 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("LDI")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 160i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:599:1, end:599:2))"]
#[derive(Clone, Debug)]
struct LDIR_instructionVar45 {}
impl LDIR_instructionVar45 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("LDIR")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 176i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:612:1, end:612:2))"]
#[derive(Clone, Debug)]
struct LDD_instructionVar46 {}
impl LDD_instructionVar46 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("LDD")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 168i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:621:1, end:621:2))"]
#[derive(Clone, Debug)]
struct LDDR_instructionVar47 {}
impl LDDR_instructionVar47 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("LDDR")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 184i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:634:1, end:634:2))"]
#[derive(Clone, Debug)]
struct CPI_instructionVar48 {}
impl CPI_instructionVar48 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("CPI")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 161i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:648:1, end:648:2))"]
#[derive(Clone, Debug)]
struct CPIR_instructionVar49 {}
impl CPIR_instructionVar49 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("CPIR")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 177i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:664:1, end:664:2))"]
#[derive(Clone, Debug)]
struct CPD_instructionVar50 {}
impl CPD_instructionVar50 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("CPD")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 169i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:678:1, end:678:2))"]
#[derive(Clone, Debug)]
struct CPDR_instructionVar51 {}
impl CPDR_instructionVar51 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("CPDR")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 185i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:721:1, end:721:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar52 {
    ixMem8: TableixMem8,
}
impl ADD_instructionVar52 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 134i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:730:1, end:730:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar53 {
    iyMem8: TableiyMem8,
}
impl ADD_instructionVar53 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 134i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:756:1, end:756:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar54 {
    ixMem8: TableixMem8,
}
impl ADC_instructionVar54 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 142i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:762:1, end:762:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar55 {
    iyMem8: TableiyMem8,
}
impl ADC_instructionVar55 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 142i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:794:1, end:794:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar56 {
    ixMem8: TableixMem8,
}
impl SUB_instructionVar56 {
    fn display_extend<T>(
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
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 150i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:803:1, end:803:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar57 {
    iyMem8: TableiyMem8,
}
impl SUB_instructionVar57 {
    fn display_extend<T>(
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
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 150i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:829:1, end:829:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar58 {
    ixMem8: TableixMem8,
}
impl SBC_instructionVar58 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SBC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 158i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:835:1, end:835:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar59 {
    iyMem8: TableiyMem8,
}
impl SBC_instructionVar59 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SBC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 158i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:870:1, end:870:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar60 {
    ixMem8: TableixMem8,
}
impl AND_instructionVar60 {
    fn display_extend<T>(
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
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 166i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:879:1, end:879:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar61 {
    iyMem8: TableiyMem8,
}
impl AND_instructionVar61 {
    fn display_extend<T>(
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
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 166i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:917:1, end:917:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar62 {
    ixMem8: TableixMem8,
}
impl OR_instructionVar62 {
    fn display_extend<T>(
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
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 182i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:926:1, end:926:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar63 {
    iyMem8: TableiyMem8,
}
impl OR_instructionVar63 {
    fn display_extend<T>(
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
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 182i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:964:1, end:964:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar64 {
    ixMem8: TableixMem8,
}
impl XOR_instructionVar64 {
    fn display_extend<T>(
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
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 174i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:973:1, end:973:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar65 {
    iyMem8: TableiyMem8,
}
impl XOR_instructionVar65 {
    fn display_extend<T>(
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
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 174i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1003:1, end:1003:2))"]
#[derive(Clone, Debug)]
struct CP_instructionVar66 {
    ixMem8: TableixMem8,
}
impl CP_instructionVar66 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("CP"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 190i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1011:1, end:1011:2))"]
#[derive(Clone, Debug)]
struct CP_instructionVar67 {
    iyMem8: TableiyMem8,
}
impl CP_instructionVar67 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("CP"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 190i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1038:1, end:1038:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar68 {
    ixMem8: TableixMem8,
}
impl INC_instructionVar68 {
    fn display_extend<T>(
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
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 52i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1048:1, end:1048:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar69 {
    iyMem8: TableiyMem8,
}
impl INC_instructionVar69 {
    fn display_extend<T>(
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
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 52i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1077:1, end:1077:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar70 {
    ixMem8: TableixMem8,
}
impl DEC_instructionVar70 {
    fn display_extend<T>(
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
        self.ixMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 53i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let ixMem8 = if let Some((len, table)) = TableixMem8::parse(
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
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1087:1, end:1087:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar71 {
    iyMem8: TableiyMem8,
}
impl DEC_instructionVar71 {
    fn display_extend<T>(
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
        self.iyMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 53i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let iyMem8 = if let Some((len, table)) = TableiyMem8::parse(
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
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1197:1, end:1197:2))"]
#[derive(Clone, Debug)]
struct NEG_instructionVar72 {}
impl NEG_instructionVar72 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NEG")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 68i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1235:1, end:1235:2))"]
#[derive(Clone, Debug)]
struct IM_instructionVar73 {}
impl IM_instructionVar73 {
    fn display_extend<T>(
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
            DisplayElement::Literal("IM"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("0"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 70i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1239:1, end:1239:2))"]
#[derive(Clone, Debug)]
struct IM_instructionVar74 {}
impl IM_instructionVar74 {
    fn display_extend<T>(
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
            DisplayElement::Literal("IM"),
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
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 86i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1243:1, end:1243:2))"]
#[derive(Clone, Debug)]
struct IM_instructionVar75 {}
impl IM_instructionVar75 {
    fn display_extend<T>(
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
            DisplayElement::Literal("IM"),
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
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 94i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1289:1, end:1289:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar76 {}
impl INC_instructionVar76 {
    fn display_extend<T>(
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
            DisplayElement::Literal("INC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IX),
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 35i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1293:1, end:1293:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar77 {}
impl INC_instructionVar77 {
    fn display_extend<T>(
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
            DisplayElement::Literal("INC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IY),
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 35i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1301:1, end:1301:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar78 {}
impl DEC_instructionVar78 {
    fn display_extend<T>(
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
            DisplayElement::Literal("DEC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IX),
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 43i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1305:1, end:1305:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar79 {}
impl DEC_instructionVar79 {
    fn display_extend<T>(
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
            DisplayElement::Literal("DEC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IY),
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 43i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1348:1, end:1348:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar80 {}
impl RLC_instructionVar80 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RLC"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1391:1, end:1391:2))"]
#[derive(Clone, Debug)]
struct RL_instructionVar81 {}
impl RL_instructionVar81 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RL"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 22i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1437:1, end:1437:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar82 {}
impl RRC_instructionVar82 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RRC"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 14i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1481:1, end:1481:2))"]
#[derive(Clone, Debug)]
struct RR_instructionVar83 {}
impl RR_instructionVar83 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RR"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 30i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1527:1, end:1527:2))"]
#[derive(Clone, Debug)]
struct SLA_instructionVar84 {}
impl SLA_instructionVar84 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SLA"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 38i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1570:1, end:1570:2))"]
#[derive(Clone, Debug)]
struct SRA_instructionVar85 {}
impl SRA_instructionVar85 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SRA"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 46i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1613:1, end:1613:2))"]
#[derive(Clone, Debug)]
struct SRL_instructionVar86 {}
impl SRL_instructionVar86 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SRL"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 62i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1647:1, end:1647:2))"]
#[derive(Clone, Debug)]
struct RLD_instructionVar87 {}
impl RLD_instructionVar87 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("RLD")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 111i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1665:1, end:1665:2))"]
#[derive(Clone, Debug)]
struct RRD_instructionVar88 {}
impl RRD_instructionVar88 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("RRD")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 103i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1788:1, end:1788:2))"]
#[derive(Clone, Debug)]
struct JP_instructionVar89 {}
impl JP_instructionVar89 {
    fn display_extend<T>(
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
            DisplayElement::Literal("JP"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::IX),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 233i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1793:1, end:1793:2))"]
#[derive(Clone, Debug)]
struct JP_instructionVar90 {}
impl JP_instructionVar90 {
    fn display_extend<T>(
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
            DisplayElement::Literal("JP"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::IY),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 233i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1827:1, end:1827:2))"]
#[derive(Clone, Debug)]
struct RETI_instructionVar91 {}
impl RETI_instructionVar91 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RETI")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 77i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1833:1, end:1833:2))"]
#[derive(Clone, Debug)]
struct RETN_instructionVar92 {}
impl RETN_instructionVar92 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RETN")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 69i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1857:1, end:1857:2))"]
#[derive(Clone, Debug)]
struct INI_instructionVar93 {
    IOAddrC: TableIOAddrC,
}
impl INI_instructionVar93 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("INI")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 162i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1867:1, end:1867:2))"]
#[derive(Clone, Debug)]
struct INIR_instructionVar94 {
    IOAddrC: TableIOAddrC,
}
impl INIR_instructionVar94 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("INIR")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 178i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1878:1, end:1878:2))"]
#[derive(Clone, Debug)]
struct IND_instructionVar95 {
    IOAddrC: TableIOAddrC,
}
impl IND_instructionVar95 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("IND")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 170i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1888:1, end:1888:2))"]
#[derive(Clone, Debug)]
struct INDR_instructionVar96 {
    IOAddrC: TableIOAddrC,
}
impl INDR_instructionVar96 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("INDR")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 186i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1907:1, end:1907:2))"]
#[derive(Clone, Debug)]
struct OUTI_instructionVar97 {
    IOAddrC: TableIOAddrC,
}
impl OUTI_instructionVar97 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("OUTI")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 163i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1917:1, end:1917:2))"]
#[derive(Clone, Debug)]
struct OTIR_instructionVar98 {
    IOAddrC: TableIOAddrC,
}
impl OTIR_instructionVar98 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("OTIR")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 179i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1928:1, end:1928:2))"]
#[derive(Clone, Debug)]
struct OUTD_instructionVar99 {
    IOAddrC: TableIOAddrC,
}
impl OUTD_instructionVar99 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("OUTD")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 171i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1938:1, end:1938:2))"]
#[derive(Clone, Debug)]
struct OTDR_instructionVar100 {
    IOAddrC: TableIOAddrC,
}
impl OTDR_instructionVar100 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("OTDR")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        if token_parser.TokenFieldop0_8().disassembly() != 187i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1965:1, end:1965:2))"]
#[derive(Clone, Debug)]
struct TST_instructionVar101 {
    hlMem8: TablehlMem8,
}
impl TST_instructionVar101 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("TST"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.hlMem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 52i128 {
            return None;
        }
        let hlMem8 = if let Some((len, table)) = TablehlMem8::parse(
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
        Some((pattern_len, Self { hlMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1976:1, end:1976:2))"]
#[derive(Clone, Debug)]
struct TST_instructionVar102 {
    imm8: TokenField_imm8,
}
impl TST_instructionVar102 {
    fn display_extend<T>(
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
            DisplayElement::Literal("TST"),
            DisplayElement::Literal(" "),
            self.imm8.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 100i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1985:1, end:1985:2))"]
#[derive(Clone, Debug)]
struct IN0_instructionVar103 {
    Flag: TableFlag,
    IOAddr8: TableIOAddr8,
}
impl IN0_instructionVar103 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("IN0"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Flag.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.IOAddr8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 0i128 {
            return None;
        }
        let Flag = if let Some((len, table)) =
            TableFlag::parse(tokens_current, &mut context_instance, inst_start)
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
        let IOAddr8 = if let Some((len, table)) = TableIOAddr8::parse(
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
        Some((pattern_len, Self { Flag, IOAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2006:1, end:2006:2))"]
#[derive(Clone, Debug)]
struct OTDM_instructionVar104 {
    hlMem8: TablehlMem8,
    IOAddrC: TableIOAddrC,
}
impl OTDM_instructionVar104 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("OTDM")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 139i128 {
            return None;
        }
        let hlMem8 = if let Some((len, table)) = TablehlMem8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        Some((pattern_len, Self { hlMem8, IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2018:1, end:2018:2))"]
#[derive(Clone, Debug)]
struct OTDMR_instructionVar105 {
    hlMem8: TablehlMem8,
    IOAddrC: TableIOAddrC,
}
impl OTDMR_instructionVar105 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("OTDMR")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 155i128 {
            return None;
        }
        let hlMem8 = if let Some((len, table)) = TablehlMem8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        Some((pattern_len, Self { hlMem8, IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2033:1, end:2033:2))"]
#[derive(Clone, Debug)]
struct TSTIO_instructionVar106 {
    IOAddr8: TableIOAddr8,
}
impl TSTIO_instructionVar106 {
    fn display_extend<T>(
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
            DisplayElement::Literal("TSTIO"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.IOAddr8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 116i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let IOAddr8 = if let Some((len, table)) = TableIOAddr8::parse(
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
        Some((pattern_len, Self { IOAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2044:1, end:2044:2))"]
#[derive(Clone, Debug)]
struct OTIM_instructionVar107 {
    hlMem8: TablehlMem8,
    IOAddrC: TableIOAddrC,
}
impl OTIM_instructionVar107 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("OTIM")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 131i128 {
            return None;
        }
        let hlMem8 = if let Some((len, table)) = TablehlMem8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        Some((pattern_len, Self { hlMem8, IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2059:1, end:2059:2))"]
#[derive(Clone, Debug)]
struct OTIMR_instructionVar108 {
    hlMem8: TablehlMem8,
    IOAddrC: TableIOAddrC,
}
impl OTIMR_instructionVar108 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("OTIMR")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 147i128 {
            return None;
        }
        let hlMem8 = if let Some((len, table)) = TablehlMem8::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        Some((pattern_len, Self { hlMem8, IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2074:1, end:2074:2))"]
#[derive(Clone, Debug)]
struct SLP_instructionVar109 {}
impl SLP_instructionVar109 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("SLP")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0_8().disassembly() != 118i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:485:1, end:485:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar110 {
    dRegPair4_2: TokenField_dRegPair4_2,
    Mem16: TableMem16,
}
impl LD_instructionVar110 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            self.dRegPair4_2.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.Mem16.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_4().disassembly() != 11i128 {
            return None;
        }
        let dRegPair4_2 = token_parser.TokenFielddRegPair4_2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { Mem16, dRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:501:1, end:501:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar111 {
    dRegPair4_2: TokenField_dRegPair4_2,
    Mem16: TableMem16,
}
impl LD_instructionVar111 {
    fn display_extend<T>(
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
        self.Mem16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.dRegPair4_2.display()];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_4().disassembly() != 3i128 {
            return None;
        }
        let dRegPair4_2 = token_parser.TokenFielddRegPair4_2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { Mem16, dRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1255:1, end:1255:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar112 {
    sRegPair4_2: TokenField_sRegPair4_2,
}
impl ADC_instructionVar112 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(","),
            self.sRegPair4_2.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_4().disassembly() != 10i128 {
            return None;
        }
        let sRegPair4_2 = token_parser.TokenFieldsRegPair4_2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1264:1, end:1264:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar113 {
    sRegPair4_2: TokenField_sRegPair4_2,
}
impl SBC_instructionVar113 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SBC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(","),
            self.sRegPair4_2.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_4().disassembly() != 2i128 {
            return None;
        }
        let sRegPair4_2 = token_parser.TokenFieldsRegPair4_2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1269:1, end:1269:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar114 {
    pRegPair4_2: TokenField_pRegPair4_2,
}
impl ADD_instructionVar114 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IX),
            DisplayElement::Literal(","),
            self.pRegPair4_2.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 221i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_4().disassembly() != 9i128 {
            return None;
        }
        let pRegPair4_2 = token_parser.TokenFieldpRegPair4_2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { pRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1277:1, end:1277:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar115 {
    pRegPair4_2: TokenField_pRegPair4_2,
}
impl ADD_instructionVar115 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::IY),
            DisplayElement::Literal(","),
            self.pRegPair4_2.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 253i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_4().disassembly() != 9i128 {
            return None;
        }
        let pRegPair4_2 = token_parser.TokenFieldpRegPair4_2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { pRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1339:1, end:1339:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar116 {
    reg0_3: TokenField_reg0_3,
}
impl RLC_instructionVar116 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RLC"),
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 0i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1381:1, end:1381:2))"]
#[derive(Clone, Debug)]
struct RL_instructionVar117 {
    reg0_3: TokenField_reg0_3,
}
impl RL_instructionVar117 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RL"),
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 2i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1428:1, end:1428:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar118 {
    reg0_3: TokenField_reg0_3,
}
impl RRC_instructionVar118 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RRC"),
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 1i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1471:1, end:1471:2))"]
#[derive(Clone, Debug)]
struct RR_instructionVar119 {
    reg0_3: TokenField_reg0_3,
}
impl RR_instructionVar119 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RR"),
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 3i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1518:1, end:1518:2))"]
#[derive(Clone, Debug)]
struct SLA_instructionVar120 {
    reg0_3: TokenField_reg0_3,
}
impl SLA_instructionVar120 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SLA"),
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 4i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1561:1, end:1561:2))"]
#[derive(Clone, Debug)]
struct SRA_instructionVar121 {
    reg0_3: TokenField_reg0_3,
}
impl SRA_instructionVar121 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SRA"),
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 5i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1604:1, end:1604:2))"]
#[derive(Clone, Debug)]
struct SRL_instructionVar122 {
    reg0_3: TokenField_reg0_3,
}
impl SRL_instructionVar122 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SRL"),
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 7i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1950:1, end:1950:2))"]
#[derive(Clone, Debug)]
struct MLT_instructionVar123 {
    qRegPair4_2: TokenField_qRegPair4_2,
}
impl MLT_instructionVar123 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MLT"),
            DisplayElement::Literal(" "),
            self.qRegPair4_2.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_4().disassembly() != 12i128 {
            return None;
        }
        let qRegPair4_2 = token_parser.TokenFieldqRegPair4_2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { qRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1690:1, end:1690:2))"]
#[derive(Clone, Debug)]
struct BIT_instructionVar124 {
    bits3_3: TokenField_bits3_3,
}
impl BIT_instructionVar124 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BIT"),
            DisplayElement::Literal(" "),
            self.bits3_3.display(),
            DisplayElement::Literal(",("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        let bits3_3 = token_parser.TokenFieldbits3_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1720:1, end:1720:2))"]
#[derive(Clone, Debug)]
struct SET_instructionVar125 {
    bits3_3: TokenField_bits3_3,
}
impl SET_instructionVar125 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SET"),
            DisplayElement::Literal(" "),
            self.bits3_3.display(),
            DisplayElement::Literal(",("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        let bits3_3 = token_parser.TokenFieldbits3_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1745:1, end:1745:2))"]
#[derive(Clone, Debug)]
struct RES_instructionVar126 {
    bits3_3: TokenField_bits3_3,
}
impl RES_instructionVar126 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RES"),
            DisplayElement::Literal(" "),
            self.bits3_3.display(),
            DisplayElement::Literal(",("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        let bits3_3 = token_parser.TokenFieldbits3_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1849:1, end:1849:2))"]
#[derive(Clone, Debug)]
struct IN_instructionVar127 {
    reg3_3: TokenField_reg3_3,
    IOAddrC: TableIOAddrC,
}
impl IN_instructionVar127 {
    fn display_extend<T>(
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
            DisplayElement::Literal("IN"),
            DisplayElement::Literal(" "),
            self.reg3_3.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.IOAddrC.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 0i128 {
            return None;
        }
        let reg3_3 = token_parser.TokenFieldreg3_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC, reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1903:1, end:1903:2))"]
#[derive(Clone, Debug)]
struct OUT_instructionVar128 {
    reg3_3: TokenField_reg3_3,
    IOAddrC: TableIOAddrC,
}
impl OUT_instructionVar128 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("OUT"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.IOAddrC.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.reg3_3.display()];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        let IOAddrC = if let Some((len, table)) = TableIOAddrC::parse(
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
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 1i128 {
            return None;
        }
        let reg3_3 = token_parser.TokenFieldreg3_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC, reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1956:1, end:1956:2))"]
#[derive(Clone, Debug)]
struct TST_instructionVar129 {
    reg3_3: TokenField_reg3_3,
}
impl TST_instructionVar129 {
    fn display_extend<T>(
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
            DisplayElement::Literal("TST"),
            DisplayElement::Literal(" "),
            self.reg3_3.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 4i128 {
            return None;
        }
        let reg3_3 = token_parser.TokenFieldreg3_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1994:1, end:1994:2))"]
#[derive(Clone, Debug)]
struct IN0_instructionVar130 {
    reg3_3: TokenField_reg3_3,
    IOAddr8: TableIOAddr8,
}
impl IN0_instructionVar130 {
    fn display_extend<T>(
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
            DisplayElement::Literal("IN0"),
            DisplayElement::Literal(" "),
            self.reg3_3.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.IOAddr8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 0i128 {
            return None;
        }
        let reg3_3 = token_parser.TokenFieldreg3_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let IOAddr8 = if let Some((len, table)) = TableIOAddr8::parse(
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
        Some((pattern_len, Self { IOAddr8, reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2002:1, end:2002:2))"]
#[derive(Clone, Debug)]
struct OUT0_instructionVar131 {
    reg3_3: TokenField_reg3_3,
    IOAddr8: TableIOAddr8,
}
impl OUT0_instructionVar131 {
    fn display_extend<T>(
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
            DisplayElement::Literal("OUT0"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.IOAddr8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.reg3_3.display()];
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
        if token_parser.TokenFieldop0_8().disassembly() != 237i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 1i128 {
            return None;
        }
        let reg3_3 = token_parser.TokenFieldreg3_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let IOAddr8 = if let Some((len, table)) = TableIOAddr8::parse(
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
        Some((pattern_len, Self { IOAddr8, reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1683:1, end:1683:2))"]
#[derive(Clone, Debug)]
struct BIT_instructionVar132 {
    bits3_3: TokenField_bits3_3,
    reg0_3: TokenField_reg0_3,
}
impl BIT_instructionVar132 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BIT"),
            DisplayElement::Literal(" "),
            self.bits3_3.display(),
            DisplayElement::Literal(","),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        let bits3_3 = token_parser.TokenFieldbits3_3();
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3, reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1715:1, end:1715:2))"]
#[derive(Clone, Debug)]
struct SET_instructionVar133 {
    bits3_3: TokenField_bits3_3,
    reg0_3: TokenField_reg0_3,
}
impl SET_instructionVar133 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SET"),
            DisplayElement::Literal(" "),
            self.bits3_3.display(),
            DisplayElement::Literal(","),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 3i128 {
            return None;
        }
        let bits3_3 = token_parser.TokenFieldbits3_3();
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3, reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1740:1, end:1740:2))"]
#[derive(Clone, Debug)]
struct RES_instructionVar134 {
    bits3_3: TokenField_bits3_3,
    reg0_3: TokenField_reg0_3,
}
impl RES_instructionVar134 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RES"),
            DisplayElement::Literal(" "),
            self.bits3_3.display(),
            DisplayElement::Literal(","),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 203i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop6_2().disassembly() != 2i128 {
            return None;
        }
        let bits3_3 = token_parser.TokenFieldbits3_3();
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3, reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:408:1, end:408:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar135 {
    imm8: TokenField_imm8,
}
impl LD_instructionVar135 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal("),"),
            self.imm8.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 54i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:421:1, end:421:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar136 {}
impl LD_instructionVar136 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(",("),
            DisplayElement::Register(Register::BC),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 10i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:425:1, end:425:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar137 {}
impl LD_instructionVar137 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(",("),
            DisplayElement::Register(Register::DE),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 26i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:429:1, end:429:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar138 {
    Mem8: TableMem8,
}
impl LD_instructionVar138 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.Mem8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 58i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Mem8 = if let Some((len, table)) =
            TableMem8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { Mem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:433:1, end:433:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar139 {}
impl LD_instructionVar139 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::BC),
            DisplayElement::Literal("),"),
            DisplayElement::Register(Register::A),
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
        if token_parser.TokenFieldop0_8().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:437:1, end:437:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar140 {}
impl LD_instructionVar140 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::DE),
            DisplayElement::Literal("),"),
            DisplayElement::Register(Register::A),
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
        if token_parser.TokenFieldop0_8().disassembly() != 18i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:441:1, end:441:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar141 {
    Mem8: TableMem8,
}
impl LD_instructionVar141 {
    fn display_extend<T>(
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
        self.Mem8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::A),
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
        if token_parser.TokenFieldop0_8().disassembly() != 50i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Mem8 = if let Some((len, table)) =
            TableMem8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { Mem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:481:1, end:481:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar142 {
    Mem16: TableMem16,
}
impl LD_instructionVar142 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.Mem16.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 42i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { Mem16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:497:1, end:497:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar143 {
    Mem16: TableMem16,
}
impl LD_instructionVar143 {
    fn display_extend<T>(
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
        self.Mem16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::HL),
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
        if token_parser.TokenFieldop0_8().disassembly() != 34i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { Mem16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:513:1, end:513:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar144 {}
impl LD_instructionVar144 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::SP),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::HL),
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
        if token_parser.TokenFieldop0_8().disassembly() != 249i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:550:1, end:550:2))"]
#[derive(Clone, Debug)]
struct EX_instructionVar145 {}
impl EX_instructionVar145 {
    fn display_extend<T>(
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
            DisplayElement::Literal("EX"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::DE),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::HL),
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
        if token_parser.TokenFieldop0_8().disassembly() != 235i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:556:1, end:556:2))"]
#[derive(Clone, Debug)]
struct EX_instructionVar146 {}
impl EX_instructionVar146 {
    fn display_extend<T>(
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
            DisplayElement::Literal("EX"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::AF),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::AF_),
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
        if token_parser.TokenFieldop0_8().disassembly() != 8i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:562:1, end:562:2))"]
#[derive(Clone, Debug)]
struct EXX_instructionVar147 {}
impl EXX_instructionVar147 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("EXX")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 217i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:574:1, end:574:2))"]
#[derive(Clone, Debug)]
struct EX_instructionVar148 {}
impl EX_instructionVar148 {
    fn display_extend<T>(
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
            DisplayElement::Literal("EX"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::SP),
            DisplayElement::Literal("),"),
            DisplayElement::Register(Register::HL),
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
        if token_parser.TokenFieldop0_8().disassembly() != 227i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:703:1, end:703:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar149 {
    imm8: TokenField_imm8,
}
impl ADD_instructionVar149 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.imm8.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 198i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:711:1, end:711:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar150 {}
impl ADD_instructionVar150 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("ADD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 134i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:744:1, end:744:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar151 {
    imm8: TokenField_imm8,
}
impl ADC_instructionVar151 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.imm8.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 206i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:749:1, end:749:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar152 {}
impl ADC_instructionVar152 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("ADC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 142i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:776:1, end:776:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar153 {
    imm8: TokenField_imm8,
}
impl SUB_instructionVar153 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
            self.imm8.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 214i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:784:1, end:784:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar154 {}
impl SUB_instructionVar154 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SUB"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 150i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:817:1, end:817:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar155 {
    imm8: TokenField_imm8,
}
impl SBC_instructionVar155 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SBC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.imm8.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 222i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:822:1, end:822:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar156 {}
impl SBC_instructionVar156 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("SBC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 158i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:850:1, end:850:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar157 {
    imm8: TokenField_imm8,
}
impl AND_instructionVar157 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
            self.imm8.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 230i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:859:1, end:859:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar158 {}
impl AND_instructionVar158 {
    fn display_extend<T>(
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
            DisplayElement::Literal("AND"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 166i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:897:1, end:897:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar159 {
    imm8: TokenField_imm8,
}
impl OR_instructionVar159 {
    fn display_extend<T>(
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
            DisplayElement::Literal("OR"),
            DisplayElement::Literal(" "),
            self.imm8.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 246i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:906:1, end:906:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar160 {}
impl OR_instructionVar160 {
    fn display_extend<T>(
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
            DisplayElement::Literal("OR"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 182i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:944:1, end:944:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar161 {
    imm8: TokenField_imm8,
}
impl XOR_instructionVar161 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
            self.imm8.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 238i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:953:1, end:953:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar162 {}
impl XOR_instructionVar162 {
    fn display_extend<T>(
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
            DisplayElement::Literal("XOR"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 174i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:988:1, end:988:2))"]
#[derive(Clone, Debug)]
struct CP_instructionVar163 {
    imm8: TokenField_imm8,
}
impl CP_instructionVar163 {
    fn display_extend<T>(
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
            DisplayElement::Literal("CP"),
            DisplayElement::Literal(" "),
            self.imm8.display(),
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
        if token_parser.TokenFieldop0_8().disassembly() != 254i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:994:1, end:994:2))"]
#[derive(Clone, Debug)]
struct CP_instructionVar164 {}
impl CP_instructionVar164 {
    fn display_extend<T>(
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
            DisplayElement::Literal("CP"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 190i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1027:1, end:1027:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar165 {}
impl INC_instructionVar165 {
    fn display_extend<T>(
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
            DisplayElement::Literal("INC"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 52i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1066:1, end:1066:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar166 {}
impl DEC_instructionVar166 {
    fn display_extend<T>(
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
            DisplayElement::Literal("DEC"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 53i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1097:1, end:1097:2))"]
#[derive(Clone, Debug)]
struct DAA_instructionVar167 {}
impl DAA_instructionVar167 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("DAA")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 39i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1191:1, end:1191:2))"]
#[derive(Clone, Debug)]
struct CPL_instructionVar168 {}
impl CPL_instructionVar168 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("CPL")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 47i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1205:1, end:1205:2))"]
#[derive(Clone, Debug)]
struct CCF_instructionVar169 {}
impl CCF_instructionVar169 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("CCF")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 63i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1210:1, end:1210:2))"]
#[derive(Clone, Debug)]
struct SCF_instructionVar170 {}
impl SCF_instructionVar170 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("SCF")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 55i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1216:1, end:1216:2))"]
#[derive(Clone, Debug)]
struct NOP_instructionVar171 {}
impl NOP_instructionVar171 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0_8().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1219:1, end:1219:2))"]
#[derive(Clone, Debug)]
struct HALT_instructionVar172 {}
impl HALT_instructionVar172 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("HALT")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 118i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1223:1, end:1223:2))"]
#[derive(Clone, Debug)]
struct DI_instructionVar173 {}
impl DI_instructionVar173 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0_8().disassembly() != 243i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1229:1, end:1229:2))"]
#[derive(Clone, Debug)]
struct EI_instructionVar174 {}
impl EI_instructionVar174 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0_8().disassembly() != 251i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1309:1, end:1309:2))"]
#[derive(Clone, Debug)]
struct RLCA_instructionVar175 {}
impl RLCA_instructionVar175 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RLCA")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 7i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1316:1, end:1316:2))"]
#[derive(Clone, Debug)]
struct RLA_instructionVar176 {}
impl RLA_instructionVar176 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("RLA")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 23i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1324:1, end:1324:2))"]
#[derive(Clone, Debug)]
struct RRCA_instructionVar177 {}
impl RRCA_instructionVar177 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RRCA")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 15i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1331:1, end:1331:2))"]
#[derive(Clone, Debug)]
struct RRA_instructionVar178 {}
impl RRA_instructionVar178 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("RRA")];
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
        if token_parser.TokenFieldop0_8().disassembly() != 31i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1765:1, end:1765:2))"]
#[derive(Clone, Debug)]
struct JP_instructionVar179 {
    Addr16: TableAddr16,
}
impl JP_instructionVar179 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("JP"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr16.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 195i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Addr16 = if let Some((len, table)) = TableAddr16::parse(
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
        Some((pattern_len, Self { Addr16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1773:1, end:1773:2))"]
#[derive(Clone, Debug)]
struct JR_instructionVar180 {
    RelAddr8: TableRelAddr8,
}
impl JR_instructionVar180 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("JR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RelAddr8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 24i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let RelAddr8 = if let Some((len, table)) = TableRelAddr8::parse(
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
        Some((pattern_len, Self { RelAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1781:1, end:1781:2))"]
#[derive(Clone, Debug)]
struct JP_instructionVar181 {}
impl JP_instructionVar181 {
    fn display_extend<T>(
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
            DisplayElement::Literal("JP"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop0_8().disassembly() != 233i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1798:1, end:1798:2))"]
#[derive(Clone, Debug)]
struct DJNZ_instructionVar182 {
    RelAddr8: TableRelAddr8,
}
impl DJNZ_instructionVar182 {
    fn display_extend<T>(
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
        self.RelAddr8.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 16i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let RelAddr8 = if let Some((len, table)) = TableRelAddr8::parse(
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
        Some((pattern_len, Self { RelAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1803:1, end:1803:2))"]
#[derive(Clone, Debug)]
struct CALL_instructionVar183 {
    Addr16: TableAddr16,
}
impl CALL_instructionVar183 {
    fn display_extend<T>(
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
            DisplayElement::Literal("CALL"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr16.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 205i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Addr16 = if let Some((len, table)) = TableAddr16::parse(
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
        Some((pattern_len, Self { Addr16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1814:1, end:1814:2))"]
#[derive(Clone, Debug)]
struct RET_instructionVar184 {}
impl RET_instructionVar184 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0_8().disassembly() != 201i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1845:1, end:1845:2))"]
#[derive(Clone, Debug)]
struct IN_instructionVar185 {
    IOAddr8a: TableIOAddr8a,
}
impl IN_instructionVar185 {
    fn display_extend<T>(
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
            DisplayElement::Literal("IN"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.IOAddr8a.display_extend(
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
        if token_parser.TokenFieldop0_8().disassembly() != 219i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let IOAddr8a = if let Some((len, table)) = TableIOAddr8a::parse(
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
        Some((pattern_len, Self { IOAddr8a }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1899:1, end:1899:2))"]
#[derive(Clone, Debug)]
struct OUT_instructionVar186 {
    IOAddr8a: TableIOAddr8a,
}
impl OUT_instructionVar186 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("OUT"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.IOAddr8a.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::A),
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
        if token_parser.TokenFieldop0_8().disassembly() != 211i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let IOAddr8a = if let Some((len, table)) = TableIOAddr8a::parse(
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
        Some((pattern_len, Self { IOAddr8a }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:469:1, end:469:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar187 {
    dRegPair4_2: TokenField_dRegPair4_2,
    imm16: TokenField_imm16,
}
impl LD_instructionVar187 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            self.dRegPair4_2.display(),
            DisplayElement::Literal(","),
            self.imm16.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_4().disassembly() != 1i128 {
            return None;
        }
        let dRegPair4_2 = token_parser.TokenFielddRegPair4_2();
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
        Some((pattern_len, Self { dRegPair4_2, imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:525:1, end:525:2))"]
#[derive(Clone, Debug)]
struct PUSH_instructionVar188 {
    qRegPair4_2: TokenField_qRegPair4_2,
}
impl PUSH_instructionVar188 {
    fn display_extend<T>(
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
            DisplayElement::Literal("PUSH"),
            DisplayElement::Literal(" "),
            self.qRegPair4_2.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_4().disassembly() != 5i128 {
            return None;
        }
        let qRegPair4_2 = token_parser.TokenFieldqRegPair4_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { qRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:537:1, end:537:2))"]
#[derive(Clone, Debug)]
struct POP_instructionVar189 {
    qRegPair4_2: TokenField_qRegPair4_2,
}
impl POP_instructionVar189 {
    fn display_extend<T>(
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
            DisplayElement::Literal("POP"),
            DisplayElement::Literal(" "),
            self.qRegPair4_2.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_4().disassembly() != 1i128 {
            return None;
        }
        let qRegPair4_2 = token_parser.TokenFieldqRegPair4_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { qRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1247:1, end:1247:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar190 {
    sRegPair4_2: TokenField_sRegPair4_2,
}
impl ADD_instructionVar190 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADD"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(","),
            self.sRegPair4_2.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_4().disassembly() != 9i128 {
            return None;
        }
        let sRegPair4_2 = token_parser.TokenFieldsRegPair4_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1285:1, end:1285:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar191 {
    sRegPair4_2: TokenField_sRegPair4_2,
}
impl INC_instructionVar191 {
    fn display_extend<T>(
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
            DisplayElement::Literal("INC"),
            DisplayElement::Literal(" "),
            self.sRegPair4_2.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_4().disassembly() != 3i128 {
            return None;
        }
        let sRegPair4_2 = token_parser.TokenFieldsRegPair4_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1297:1, end:1297:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar192 {
    sRegPair4_2: TokenField_sRegPair4_2,
}
impl DEC_instructionVar192 {
    fn display_extend<T>(
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
            DisplayElement::Literal("DEC"),
            DisplayElement::Literal(" "),
            self.sRegPair4_2.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_4().disassembly() != 11i128 {
            return None;
        }
        let sRegPair4_2 = token_parser.TokenFieldsRegPair4_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:380:1, end:380:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar193 {
    reg3_3: TokenField_reg3_3,
    imm8: TokenField_imm8,
}
impl LD_instructionVar193 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            self.reg3_3.display(),
            DisplayElement::Literal(","),
            self.imm8.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        let reg3_3 = token_parser.TokenFieldreg3_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_3, imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:384:1, end:384:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar194 {
    reg3_3: TokenField_reg3_3,
}
impl LD_instructionVar194 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            self.reg3_3.display(),
            DisplayElement::Literal(",("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 6i128 {
            return None;
        }
        let reg3_3 = token_parser.TokenFieldreg3_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:396:1, end:396:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar195 {
    reg0_3: TokenField_reg0_3,
}
impl LD_instructionVar195 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal("),"),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 6i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:694:1, end:694:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar196 {
    reg0_3: TokenField_reg0_3,
}
impl ADD_instructionVar196 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 0i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:739:1, end:739:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar197 {
    reg0_3: TokenField_reg0_3,
}
impl ADC_instructionVar197 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 1i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:768:1, end:768:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar198 {
    reg0_3: TokenField_reg0_3,
}
impl SUB_instructionVar198 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 2i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:812:1, end:812:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar199 {
    reg0_3: TokenField_reg0_3,
}
impl SBC_instructionVar199 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SBC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::A),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 3i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:841:1, end:841:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar200 {
    reg0_3: TokenField_reg0_3,
}
impl AND_instructionVar200 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 4i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:888:1, end:888:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar201 {
    reg0_3: TokenField_reg0_3,
}
impl OR_instructionVar201 {
    fn display_extend<T>(
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
            DisplayElement::Literal("OR"),
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 6i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:935:1, end:935:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar202 {
    reg0_3: TokenField_reg0_3,
}
impl XOR_instructionVar202 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 5i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:982:1, end:982:2))"]
#[derive(Clone, Debug)]
struct CP_instructionVar203 {
    reg0_3: TokenField_reg0_3,
}
impl CP_instructionVar203 {
    fn display_extend<T>(
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
            DisplayElement::Literal("CP"),
            DisplayElement::Literal(" "),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldbits3_3().disassembly() != 7i128 {
            return None;
        }
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1019:1, end:1019:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar204 {
    reg3_3: TokenField_reg3_3,
}
impl INC_instructionVar204 {
    fn display_extend<T>(
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
            DisplayElement::Literal("INC"),
            DisplayElement::Literal(" "),
            self.reg3_3.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 4i128 {
            return None;
        }
        let reg3_3 = token_parser.TokenFieldreg3_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1058:1, end:1058:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar205 {
    reg3_3: TokenField_reg3_3,
}
impl DEC_instructionVar205 {
    fn display_extend<T>(
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
            DisplayElement::Literal("DEC"),
            DisplayElement::Literal(" "),
            self.reg3_3.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 5i128 {
            return None;
        }
        let reg3_3 = token_parser.TokenFieldreg3_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1769:1, end:1769:2))"]
#[derive(Clone, Debug)]
struct JP_instructionVar206 {
    cc: Tablecc,
    Addr16: TableAddr16,
}
impl JP_instructionVar206 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("JP"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.cc.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Addr16.display_extend(
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
        if token_parser.TokenFieldop6_2().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 2i128 {
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
        let Addr16 = if let Some((len, table)) = TableAddr16::parse(
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
        Some((pattern_len, Self { cc, Addr16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1777:1, end:1777:2))"]
#[derive(Clone, Debug)]
struct JR_instructionVar207 {
    cc2: Tablecc2,
    RelAddr8: TableRelAddr8,
}
impl JR_instructionVar207 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("JR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.cc2.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.RelAddr8.display_extend(
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
        if token_parser.TokenFieldop6_2().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 0i128 {
            return None;
        }
        let cc2 = if let Some((len, table)) =
            Tablecc2::parse(tokens_current, &mut context_instance, inst_start)
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
        let RelAddr8 = if let Some((len, table)) = TableRelAddr8::parse(
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
        Some((pattern_len, Self { cc2, RelAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1808:1, end:1808:2))"]
#[derive(Clone, Debug)]
struct CALL_instructionVar208 {
    cc: Tablecc,
    Addr16: TableAddr16,
}
impl CALL_instructionVar208 {
    fn display_extend<T>(
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
            DisplayElement::Literal("CALL"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.cc.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Addr16.display_extend(
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
        if token_parser.TokenFieldop6_2().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 4i128 {
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
        let Addr16 = if let Some((len, table)) = TableAddr16::parse(
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
        Some((pattern_len, Self { cc, Addr16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1820:1, end:1820:2))"]
#[derive(Clone, Debug)]
struct RET_instructionVar209 {
    cc: Tablecc,
}
impl RET_instructionVar209 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RET"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.cc.display_extend(
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
        if token_parser.TokenFieldop6_2().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 0i128 {
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
        *context = context_instance;
        Some((pattern_len, Self { cc }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1840:1, end:1840:2))"]
#[derive(Clone, Debug)]
struct RST_instructionVar210 {
    RstAddr: TableRstAddr,
}
impl RST_instructionVar210 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RST"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RstAddr.display_extend(
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
        if token_parser.TokenFieldop6_2().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldbits0_3().disassembly() != 7i128 {
            return None;
        }
        let RstAddr = if let Some((len, table)) = TableRstAddr::parse(
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
        Some((pattern_len, Self { RstAddr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:376:1, end:376:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar211 {
    reg3_3: TokenField_reg3_3,
    reg0_3: TokenField_reg0_3,
}
impl LD_instructionVar211 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LD"),
            DisplayElement::Literal(" "),
            self.reg3_3.display(),
            DisplayElement::Literal(","),
            self.reg0_3.display(),
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
        if token_parser.TokenFieldop6_2().disassembly() != 1i128 {
            return None;
        }
        let reg3_3 = token_parser.TokenFieldreg3_3();
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_3, reg0_3 }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(RLC_instructionVar0),
    Var1(RLC_instructionVar1),
    Var2(RL_instructionVar2),
    Var3(RL_instructionVar3),
    Var4(RRC_instructionVar4),
    Var5(RRC_instructionVar5),
    Var6(RR_instructionVar6),
    Var7(RR_instructionVar7),
    Var8(SLA_instructionVar8),
    Var9(SLA_instructionVar9),
    Var10(SRA_instructionVar10),
    Var11(SRA_instructionVar11),
    Var12(SRL_instructionVar12),
    Var13(SRL_instructionVar13),
    Var14(BIT_instructionVar14),
    Var15(BIT_instructionVar15),
    Var16(SET_instructionVar16),
    Var17(SET_instructionVar17),
    Var18(RES_instructionVar18),
    Var19(RES_instructionVar19),
    Var20(LD_instructionVar20),
    Var21(LD_instructionVar21),
    Var22(LD_instructionVar22),
    Var23(LD_instructionVar23),
    Var24(LD_instructionVar24),
    Var25(LD_instructionVar25),
    Var26(LD_instructionVar26),
    Var27(LD_instructionVar27),
    Var28(LD_instructionVar28),
    Var29(LD_instructionVar29),
    Var30(LD_instructionVar30),
    Var31(LD_instructionVar31),
    Var32(LD_instructionVar32),
    Var33(LD_instructionVar33),
    Var34(LD_instructionVar34),
    Var35(LD_instructionVar35),
    Var36(LD_instructionVar36),
    Var37(LD_instructionVar37),
    Var38(PUSH_instructionVar38),
    Var39(PUSH_instructionVar39),
    Var40(POP_instructionVar40),
    Var41(POP_instructionVar41),
    Var42(EX_instructionVar42),
    Var43(EX_instructionVar43),
    Var44(LDI_instructionVar44),
    Var45(LDIR_instructionVar45),
    Var46(LDD_instructionVar46),
    Var47(LDDR_instructionVar47),
    Var48(CPI_instructionVar48),
    Var49(CPIR_instructionVar49),
    Var50(CPD_instructionVar50),
    Var51(CPDR_instructionVar51),
    Var52(ADD_instructionVar52),
    Var53(ADD_instructionVar53),
    Var54(ADC_instructionVar54),
    Var55(ADC_instructionVar55),
    Var56(SUB_instructionVar56),
    Var57(SUB_instructionVar57),
    Var58(SBC_instructionVar58),
    Var59(SBC_instructionVar59),
    Var60(AND_instructionVar60),
    Var61(AND_instructionVar61),
    Var62(OR_instructionVar62),
    Var63(OR_instructionVar63),
    Var64(XOR_instructionVar64),
    Var65(XOR_instructionVar65),
    Var66(CP_instructionVar66),
    Var67(CP_instructionVar67),
    Var68(INC_instructionVar68),
    Var69(INC_instructionVar69),
    Var70(DEC_instructionVar70),
    Var71(DEC_instructionVar71),
    Var72(NEG_instructionVar72),
    Var73(IM_instructionVar73),
    Var74(IM_instructionVar74),
    Var75(IM_instructionVar75),
    Var76(INC_instructionVar76),
    Var77(INC_instructionVar77),
    Var78(DEC_instructionVar78),
    Var79(DEC_instructionVar79),
    Var80(RLC_instructionVar80),
    Var81(RL_instructionVar81),
    Var82(RRC_instructionVar82),
    Var83(RR_instructionVar83),
    Var84(SLA_instructionVar84),
    Var85(SRA_instructionVar85),
    Var86(SRL_instructionVar86),
    Var87(RLD_instructionVar87),
    Var88(RRD_instructionVar88),
    Var89(JP_instructionVar89),
    Var90(JP_instructionVar90),
    Var91(RETI_instructionVar91),
    Var92(RETN_instructionVar92),
    Var93(INI_instructionVar93),
    Var94(INIR_instructionVar94),
    Var95(IND_instructionVar95),
    Var96(INDR_instructionVar96),
    Var97(OUTI_instructionVar97),
    Var98(OTIR_instructionVar98),
    Var99(OUTD_instructionVar99),
    Var100(OTDR_instructionVar100),
    Var101(TST_instructionVar101),
    Var102(TST_instructionVar102),
    Var103(IN0_instructionVar103),
    Var104(OTDM_instructionVar104),
    Var105(OTDMR_instructionVar105),
    Var106(TSTIO_instructionVar106),
    Var107(OTIM_instructionVar107),
    Var108(OTIMR_instructionVar108),
    Var109(SLP_instructionVar109),
    Var110(LD_instructionVar110),
    Var111(LD_instructionVar111),
    Var112(ADC_instructionVar112),
    Var113(SBC_instructionVar113),
    Var114(ADD_instructionVar114),
    Var115(ADD_instructionVar115),
    Var116(RLC_instructionVar116),
    Var117(RL_instructionVar117),
    Var118(RRC_instructionVar118),
    Var119(RR_instructionVar119),
    Var120(SLA_instructionVar120),
    Var121(SRA_instructionVar121),
    Var122(SRL_instructionVar122),
    Var123(MLT_instructionVar123),
    Var124(BIT_instructionVar124),
    Var125(SET_instructionVar125),
    Var126(RES_instructionVar126),
    Var127(IN_instructionVar127),
    Var128(OUT_instructionVar128),
    Var129(TST_instructionVar129),
    Var130(IN0_instructionVar130),
    Var131(OUT0_instructionVar131),
    Var132(BIT_instructionVar132),
    Var133(SET_instructionVar133),
    Var134(RES_instructionVar134),
    Var135(LD_instructionVar135),
    Var136(LD_instructionVar136),
    Var137(LD_instructionVar137),
    Var138(LD_instructionVar138),
    Var139(LD_instructionVar139),
    Var140(LD_instructionVar140),
    Var141(LD_instructionVar141),
    Var142(LD_instructionVar142),
    Var143(LD_instructionVar143),
    Var144(LD_instructionVar144),
    Var145(EX_instructionVar145),
    Var146(EX_instructionVar146),
    Var147(EXX_instructionVar147),
    Var148(EX_instructionVar148),
    Var149(ADD_instructionVar149),
    Var150(ADD_instructionVar150),
    Var151(ADC_instructionVar151),
    Var152(ADC_instructionVar152),
    Var153(SUB_instructionVar153),
    Var154(SUB_instructionVar154),
    Var155(SBC_instructionVar155),
    Var156(SBC_instructionVar156),
    Var157(AND_instructionVar157),
    Var158(AND_instructionVar158),
    Var159(OR_instructionVar159),
    Var160(OR_instructionVar160),
    Var161(XOR_instructionVar161),
    Var162(XOR_instructionVar162),
    Var163(CP_instructionVar163),
    Var164(CP_instructionVar164),
    Var165(INC_instructionVar165),
    Var166(DEC_instructionVar166),
    Var167(DAA_instructionVar167),
    Var168(CPL_instructionVar168),
    Var169(CCF_instructionVar169),
    Var170(SCF_instructionVar170),
    Var171(NOP_instructionVar171),
    Var172(HALT_instructionVar172),
    Var173(DI_instructionVar173),
    Var174(EI_instructionVar174),
    Var175(RLCA_instructionVar175),
    Var176(RLA_instructionVar176),
    Var177(RRCA_instructionVar177),
    Var178(RRA_instructionVar178),
    Var179(JP_instructionVar179),
    Var180(JR_instructionVar180),
    Var181(JP_instructionVar181),
    Var182(DJNZ_instructionVar182),
    Var183(CALL_instructionVar183),
    Var184(RET_instructionVar184),
    Var185(IN_instructionVar185),
    Var186(OUT_instructionVar186),
    Var187(LD_instructionVar187),
    Var188(PUSH_instructionVar188),
    Var189(POP_instructionVar189),
    Var190(ADD_instructionVar190),
    Var191(INC_instructionVar191),
    Var192(DEC_instructionVar192),
    Var193(LD_instructionVar193),
    Var194(LD_instructionVar194),
    Var195(LD_instructionVar195),
    Var196(ADD_instructionVar196),
    Var197(ADC_instructionVar197),
    Var198(SUB_instructionVar198),
    Var199(SBC_instructionVar199),
    Var200(AND_instructionVar200),
    Var201(OR_instructionVar201),
    Var202(XOR_instructionVar202),
    Var203(CP_instructionVar203),
    Var204(INC_instructionVar204),
    Var205(DEC_instructionVar205),
    Var206(JP_instructionVar206),
    Var207(JR_instructionVar207),
    Var208(CALL_instructionVar208),
    Var209(RET_instructionVar209),
    Var210(RST_instructionVar210),
    Var211(LD_instructionVar211),
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
        if let Some((inst_len, parsed)) = RLC_instructionVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = RLC_instructionVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = RL_instructionVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = RL_instructionVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = RRC_instructionVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) = RRC_instructionVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) = RR_instructionVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) = RR_instructionVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) = SLA_instructionVar8::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) = SLA_instructionVar9::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) = SRA_instructionVar10::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) = SRA_instructionVar11::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) = SRL_instructionVar12::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) = SRL_instructionVar13::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) = BIT_instructionVar14::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) = BIT_instructionVar15::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        if let Some((inst_len, parsed)) = SET_instructionVar16::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var16(parsed)));
        }
        if let Some((inst_len, parsed)) = SET_instructionVar17::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var17(parsed)));
        }
        if let Some((inst_len, parsed)) = RES_instructionVar18::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var18(parsed)));
        }
        if let Some((inst_len, parsed)) = RES_instructionVar19::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var19(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar20::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var20(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar21::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var21(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar22::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var22(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar23::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var23(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar24::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var24(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar25::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var25(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar26::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var26(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar27::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var27(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar28::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var28(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar29::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var29(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar30::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var30(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar31::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var31(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar32::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var32(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar33::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var33(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar34::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var34(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar35::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var35(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar36::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var36(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar37::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var37(parsed)));
        }
        if let Some((inst_len, parsed)) = PUSH_instructionVar38::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var38(parsed)));
        }
        if let Some((inst_len, parsed)) = PUSH_instructionVar39::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var39(parsed)));
        }
        if let Some((inst_len, parsed)) = POP_instructionVar40::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var40(parsed)));
        }
        if let Some((inst_len, parsed)) = POP_instructionVar41::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var41(parsed)));
        }
        if let Some((inst_len, parsed)) = EX_instructionVar42::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var42(parsed)));
        }
        if let Some((inst_len, parsed)) = EX_instructionVar43::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var43(parsed)));
        }
        if let Some((inst_len, parsed)) = LDI_instructionVar44::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var44(parsed)));
        }
        if let Some((inst_len, parsed)) = LDIR_instructionVar45::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var45(parsed)));
        }
        if let Some((inst_len, parsed)) = LDD_instructionVar46::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var46(parsed)));
        }
        if let Some((inst_len, parsed)) = LDDR_instructionVar47::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var47(parsed)));
        }
        if let Some((inst_len, parsed)) = CPI_instructionVar48::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var48(parsed)));
        }
        if let Some((inst_len, parsed)) = CPIR_instructionVar49::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var49(parsed)));
        }
        if let Some((inst_len, parsed)) = CPD_instructionVar50::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var50(parsed)));
        }
        if let Some((inst_len, parsed)) = CPDR_instructionVar51::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var51(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar52::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var52(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar53::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var53(parsed)));
        }
        if let Some((inst_len, parsed)) = ADC_instructionVar54::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var54(parsed)));
        }
        if let Some((inst_len, parsed)) = ADC_instructionVar55::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var55(parsed)));
        }
        if let Some((inst_len, parsed)) = SUB_instructionVar56::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var56(parsed)));
        }
        if let Some((inst_len, parsed)) = SUB_instructionVar57::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var57(parsed)));
        }
        if let Some((inst_len, parsed)) = SBC_instructionVar58::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var58(parsed)));
        }
        if let Some((inst_len, parsed)) = SBC_instructionVar59::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var59(parsed)));
        }
        if let Some((inst_len, parsed)) = AND_instructionVar60::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var60(parsed)));
        }
        if let Some((inst_len, parsed)) = AND_instructionVar61::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var61(parsed)));
        }
        if let Some((inst_len, parsed)) = OR_instructionVar62::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var62(parsed)));
        }
        if let Some((inst_len, parsed)) = OR_instructionVar63::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var63(parsed)));
        }
        if let Some((inst_len, parsed)) = XOR_instructionVar64::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var64(parsed)));
        }
        if let Some((inst_len, parsed)) = XOR_instructionVar65::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var65(parsed)));
        }
        if let Some((inst_len, parsed)) = CP_instructionVar66::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var66(parsed)));
        }
        if let Some((inst_len, parsed)) = CP_instructionVar67::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var67(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar68::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var68(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar69::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var69(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar70::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var70(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar71::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var71(parsed)));
        }
        if let Some((inst_len, parsed)) = NEG_instructionVar72::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var72(parsed)));
        }
        if let Some((inst_len, parsed)) = IM_instructionVar73::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var73(parsed)));
        }
        if let Some((inst_len, parsed)) = IM_instructionVar74::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var74(parsed)));
        }
        if let Some((inst_len, parsed)) = IM_instructionVar75::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var75(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar76::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var76(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar77::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var77(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar78::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var78(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar79::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var79(parsed)));
        }
        if let Some((inst_len, parsed)) = RLC_instructionVar80::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var80(parsed)));
        }
        if let Some((inst_len, parsed)) = RL_instructionVar81::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var81(parsed)));
        }
        if let Some((inst_len, parsed)) = RRC_instructionVar82::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var82(parsed)));
        }
        if let Some((inst_len, parsed)) = RR_instructionVar83::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var83(parsed)));
        }
        if let Some((inst_len, parsed)) = SLA_instructionVar84::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var84(parsed)));
        }
        if let Some((inst_len, parsed)) = SRA_instructionVar85::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var85(parsed)));
        }
        if let Some((inst_len, parsed)) = SRL_instructionVar86::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var86(parsed)));
        }
        if let Some((inst_len, parsed)) = RLD_instructionVar87::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var87(parsed)));
        }
        if let Some((inst_len, parsed)) = RRD_instructionVar88::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var88(parsed)));
        }
        if let Some((inst_len, parsed)) = JP_instructionVar89::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var89(parsed)));
        }
        if let Some((inst_len, parsed)) = JP_instructionVar90::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var90(parsed)));
        }
        if let Some((inst_len, parsed)) = RETI_instructionVar91::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var91(parsed)));
        }
        if let Some((inst_len, parsed)) = RETN_instructionVar92::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var92(parsed)));
        }
        if let Some((inst_len, parsed)) = INI_instructionVar93::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var93(parsed)));
        }
        if let Some((inst_len, parsed)) = INIR_instructionVar94::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var94(parsed)));
        }
        if let Some((inst_len, parsed)) = IND_instructionVar95::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var95(parsed)));
        }
        if let Some((inst_len, parsed)) = INDR_instructionVar96::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var96(parsed)));
        }
        if let Some((inst_len, parsed)) = OUTI_instructionVar97::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var97(parsed)));
        }
        if let Some((inst_len, parsed)) = OTIR_instructionVar98::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var98(parsed)));
        }
        if let Some((inst_len, parsed)) = OUTD_instructionVar99::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var99(parsed)));
        }
        if let Some((inst_len, parsed)) = OTDR_instructionVar100::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var100(parsed)));
        }
        if let Some((inst_len, parsed)) = TST_instructionVar101::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var101(parsed)));
        }
        if let Some((inst_len, parsed)) = TST_instructionVar102::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var102(parsed)));
        }
        if let Some((inst_len, parsed)) = IN0_instructionVar103::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var103(parsed)));
        }
        if let Some((inst_len, parsed)) = OTDM_instructionVar104::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var104(parsed)));
        }
        if let Some((inst_len, parsed)) = OTDMR_instructionVar105::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var105(parsed)));
        }
        if let Some((inst_len, parsed)) = TSTIO_instructionVar106::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var106(parsed)));
        }
        if let Some((inst_len, parsed)) = OTIM_instructionVar107::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var107(parsed)));
        }
        if let Some((inst_len, parsed)) = OTIMR_instructionVar108::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var108(parsed)));
        }
        if let Some((inst_len, parsed)) = SLP_instructionVar109::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var109(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar110::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var110(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar111::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var111(parsed)));
        }
        if let Some((inst_len, parsed)) = ADC_instructionVar112::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var112(parsed)));
        }
        if let Some((inst_len, parsed)) = SBC_instructionVar113::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var113(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar114::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var114(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar115::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var115(parsed)));
        }
        if let Some((inst_len, parsed)) = RLC_instructionVar116::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var116(parsed)));
        }
        if let Some((inst_len, parsed)) = RL_instructionVar117::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var117(parsed)));
        }
        if let Some((inst_len, parsed)) = RRC_instructionVar118::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var118(parsed)));
        }
        if let Some((inst_len, parsed)) = RR_instructionVar119::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var119(parsed)));
        }
        if let Some((inst_len, parsed)) = SLA_instructionVar120::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var120(parsed)));
        }
        if let Some((inst_len, parsed)) = SRA_instructionVar121::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var121(parsed)));
        }
        if let Some((inst_len, parsed)) = SRL_instructionVar122::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var122(parsed)));
        }
        if let Some((inst_len, parsed)) = MLT_instructionVar123::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var123(parsed)));
        }
        if let Some((inst_len, parsed)) = BIT_instructionVar124::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var124(parsed)));
        }
        if let Some((inst_len, parsed)) = SET_instructionVar125::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var125(parsed)));
        }
        if let Some((inst_len, parsed)) = RES_instructionVar126::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var126(parsed)));
        }
        if let Some((inst_len, parsed)) = IN_instructionVar127::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var127(parsed)));
        }
        if let Some((inst_len, parsed)) = OUT_instructionVar128::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var128(parsed)));
        }
        if let Some((inst_len, parsed)) = TST_instructionVar129::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var129(parsed)));
        }
        if let Some((inst_len, parsed)) = IN0_instructionVar130::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var130(parsed)));
        }
        if let Some((inst_len, parsed)) = OUT0_instructionVar131::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var131(parsed)));
        }
        if let Some((inst_len, parsed)) = BIT_instructionVar132::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var132(parsed)));
        }
        if let Some((inst_len, parsed)) = SET_instructionVar133::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var133(parsed)));
        }
        if let Some((inst_len, parsed)) = RES_instructionVar134::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var134(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar135::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var135(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar136::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var136(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar137::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var137(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar138::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var138(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar139::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var139(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar140::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var140(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar141::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var141(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar142::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var142(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar143::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var143(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar144::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var144(parsed)));
        }
        if let Some((inst_len, parsed)) = EX_instructionVar145::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var145(parsed)));
        }
        if let Some((inst_len, parsed)) = EX_instructionVar146::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var146(parsed)));
        }
        if let Some((inst_len, parsed)) = EXX_instructionVar147::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var147(parsed)));
        }
        if let Some((inst_len, parsed)) = EX_instructionVar148::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var148(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar149::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var149(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar150::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var150(parsed)));
        }
        if let Some((inst_len, parsed)) = ADC_instructionVar151::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var151(parsed)));
        }
        if let Some((inst_len, parsed)) = ADC_instructionVar152::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var152(parsed)));
        }
        if let Some((inst_len, parsed)) = SUB_instructionVar153::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var153(parsed)));
        }
        if let Some((inst_len, parsed)) = SUB_instructionVar154::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var154(parsed)));
        }
        if let Some((inst_len, parsed)) = SBC_instructionVar155::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var155(parsed)));
        }
        if let Some((inst_len, parsed)) = SBC_instructionVar156::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var156(parsed)));
        }
        if let Some((inst_len, parsed)) = AND_instructionVar157::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var157(parsed)));
        }
        if let Some((inst_len, parsed)) = AND_instructionVar158::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var158(parsed)));
        }
        if let Some((inst_len, parsed)) = OR_instructionVar159::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var159(parsed)));
        }
        if let Some((inst_len, parsed)) = OR_instructionVar160::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var160(parsed)));
        }
        if let Some((inst_len, parsed)) = XOR_instructionVar161::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var161(parsed)));
        }
        if let Some((inst_len, parsed)) = XOR_instructionVar162::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var162(parsed)));
        }
        if let Some((inst_len, parsed)) = CP_instructionVar163::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var163(parsed)));
        }
        if let Some((inst_len, parsed)) = CP_instructionVar164::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var164(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar165::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var165(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar166::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var166(parsed)));
        }
        if let Some((inst_len, parsed)) = DAA_instructionVar167::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var167(parsed)));
        }
        if let Some((inst_len, parsed)) = CPL_instructionVar168::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var168(parsed)));
        }
        if let Some((inst_len, parsed)) = CCF_instructionVar169::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var169(parsed)));
        }
        if let Some((inst_len, parsed)) = SCF_instructionVar170::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var170(parsed)));
        }
        if let Some((inst_len, parsed)) = NOP_instructionVar171::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var171(parsed)));
        }
        if let Some((inst_len, parsed)) = HALT_instructionVar172::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var172(parsed)));
        }
        if let Some((inst_len, parsed)) = DI_instructionVar173::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var173(parsed)));
        }
        if let Some((inst_len, parsed)) = EI_instructionVar174::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var174(parsed)));
        }
        if let Some((inst_len, parsed)) = RLCA_instructionVar175::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var175(parsed)));
        }
        if let Some((inst_len, parsed)) = RLA_instructionVar176::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var176(parsed)));
        }
        if let Some((inst_len, parsed)) = RRCA_instructionVar177::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var177(parsed)));
        }
        if let Some((inst_len, parsed)) = RRA_instructionVar178::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var178(parsed)));
        }
        if let Some((inst_len, parsed)) = JP_instructionVar179::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var179(parsed)));
        }
        if let Some((inst_len, parsed)) = JR_instructionVar180::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var180(parsed)));
        }
        if let Some((inst_len, parsed)) = JP_instructionVar181::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var181(parsed)));
        }
        if let Some((inst_len, parsed)) = DJNZ_instructionVar182::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var182(parsed)));
        }
        if let Some((inst_len, parsed)) = CALL_instructionVar183::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var183(parsed)));
        }
        if let Some((inst_len, parsed)) = RET_instructionVar184::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var184(parsed)));
        }
        if let Some((inst_len, parsed)) = IN_instructionVar185::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var185(parsed)));
        }
        if let Some((inst_len, parsed)) = OUT_instructionVar186::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var186(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar187::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var187(parsed)));
        }
        if let Some((inst_len, parsed)) = PUSH_instructionVar188::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var188(parsed)));
        }
        if let Some((inst_len, parsed)) = POP_instructionVar189::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var189(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar190::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var190(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar191::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var191(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar192::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var192(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar193::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var193(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar194::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var194(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar195::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var195(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar196::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var196(parsed)));
        }
        if let Some((inst_len, parsed)) = ADC_instructionVar197::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var197(parsed)));
        }
        if let Some((inst_len, parsed)) = SUB_instructionVar198::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var198(parsed)));
        }
        if let Some((inst_len, parsed)) = SBC_instructionVar199::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var199(parsed)));
        }
        if let Some((inst_len, parsed)) = AND_instructionVar200::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var200(parsed)));
        }
        if let Some((inst_len, parsed)) = OR_instructionVar201::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var201(parsed)));
        }
        if let Some((inst_len, parsed)) = XOR_instructionVar202::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var202(parsed)));
        }
        if let Some((inst_len, parsed)) = CP_instructionVar203::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var203(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar204::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var204(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar205::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var205(parsed)));
        }
        if let Some((inst_len, parsed)) = JP_instructionVar206::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var206(parsed)));
        }
        if let Some((inst_len, parsed)) = JR_instructionVar207::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var207(parsed)));
        }
        if let Some((inst_len, parsed)) = CALL_instructionVar208::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var208(parsed)));
        }
        if let Some((inst_len, parsed)) = RET_instructionVar209::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var209(parsed)));
        }
        if let Some((inst_len, parsed)) = RST_instructionVar210::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var210(parsed)));
        }
        if let Some((inst_len, parsed)) = LD_instructionVar211::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var211(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:284:1, end:284:5))"]
#[derive(Clone, Debug)]
struct FlagVar0 {}
impl FlagVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("Flag")];
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
        let reg0_3 = token_parser.TokenFieldreg0_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableFlag {
    Var0(FlagVar0),
}
impl TableFlag {
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
            FlagVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:314:1, end:314:7))"]
#[derive(Clone, Debug)]
struct hlMem8Var0 {}
impl hlMem8Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::HL),
            DisplayElement::Literal(")"),
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
        let mut block_0_len = 0u64 as u16;
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TablehlMem8 {
    Var0(hlMem8Var0),
}
impl TablehlMem8 {
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
            hlMem8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:318:1, end:318:7))"]
#[derive(Clone, Debug)]
struct ixMem8Var0 {
    simm8: TokenField_simm8,
}
impl ixMem8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_val: i128 = 0;
        calc_val = (-self.simm8.disassembly());
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::IX),
            DisplayElement::Literal("-"),
            DisplayElement::Number(true, calc_val),
            DisplayElement::Literal(")"),
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
        let mut calc_val: i128 = 0;
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldsign8().disassembly() != 1i128 {
            return None;
        }
        calc_val = (-token_parser.TokenFieldsimm8().disassembly());
        let simm8 = token_parser.TokenFieldsimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:317:1, end:317:7))"]
#[derive(Clone, Debug)]
struct ixMem8Var1 {
    simm8: TokenField_simm8,
}
impl ixMem8Var1 {
    fn display_extend<T>(
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
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::IX),
            DisplayElement::Literal("+"),
            self.simm8.display(),
            DisplayElement::Literal(")"),
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
        let simm8 = token_parser.TokenFieldsimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[derive(Clone, Debug)]
enum TableixMem8 {
    Var0(ixMem8Var0),
    Var1(ixMem8Var1),
}
impl TableixMem8 {
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
            ixMem8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ixMem8Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:321:1, end:321:7))"]
#[derive(Clone, Debug)]
struct iyMem8Var0 {
    simm8: TokenField_simm8,
}
impl iyMem8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_val: i128 = 0;
        calc_val = (-self.simm8.disassembly());
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::IY),
            DisplayElement::Literal("-"),
            DisplayElement::Number(true, calc_val),
            DisplayElement::Literal(")"),
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
        let mut calc_val: i128 = 0;
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldsign8().disassembly() != 1i128 {
            return None;
        }
        calc_val = (-token_parser.TokenFieldsimm8().disassembly());
        let simm8 = token_parser.TokenFieldsimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:320:1, end:320:7))"]
#[derive(Clone, Debug)]
struct iyMem8Var1 {
    simm8: TokenField_simm8,
}
impl iyMem8Var1 {
    fn display_extend<T>(
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
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::IY),
            DisplayElement::Literal("+"),
            self.simm8.display(),
            DisplayElement::Literal(")"),
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
        let simm8 = token_parser.TokenFieldsimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[derive(Clone, Debug)]
enum TableiyMem8 {
    Var0(iyMem8Var0),
    Var1(iyMem8Var1),
}
impl TableiyMem8 {
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
            iyMem8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            iyMem8Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:326:1, end:326:7))"]
#[derive(Clone, Debug)]
struct Addr16Var0 {
    imm16: TokenField_imm16,
}
impl Addr16Var0 {
    fn display_extend<T>(
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
enum TableAddr16 {
    Var0(Addr16Var0),
}
impl TableAddr16 {
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
            Addr16Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:328:1, end:328:5))"]
#[derive(Clone, Debug)]
struct Mem8Var0 {
    imm16: TokenField_imm16,
}
impl Mem8Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("("),
            self.imm16.display(),
            DisplayElement::Literal(")"),
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
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[derive(Clone, Debug)]
enum TableMem8 {
    Var0(Mem8Var0),
}
impl TableMem8 {
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
            Mem8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:330:1, end:330:6))"]
#[derive(Clone, Debug)]
struct Mem16Var0 {
    imm16: TokenField_imm16,
}
impl Mem16Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("("),
            self.imm16.display(),
            DisplayElement::Literal(")"),
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
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[derive(Clone, Debug)]
enum TableMem16 {
    Var0(Mem16Var0),
}
impl TableMem16 {
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
            Mem16Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:342:1, end:342:9))"]
#[derive(Clone, Debug)]
struct RelAddr8Var0 {
    simm8: TokenField_simm8,
}
impl RelAddr8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_loc: i128 = 0;
        calc_loc = i128::try_from(inst_next)
            .unwrap()
            .wrapping_add(self.simm8.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_loc)];
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
        let mut calc_loc: i128 = 0;
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
enum TableRelAddr8 {
    Var0(RelAddr8Var0),
}
impl TableRelAddr8 {
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
            RelAddr8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:344:1, end:344:8))"]
#[derive(Clone, Debug)]
struct RstAddrVar0 {
    bits3_3: TokenField_bits3_3,
}
impl RstAddrVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_loc: i128 = 0;
        calc_loc = u32::try_from(3i128)
            .ok()
            .map(|shl| self.bits3_3.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0);
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_loc)];
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
        let mut calc_loc: i128 = 0;
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_loc = u32::try_from(3i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldbits3_3()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0);
        let bits3_3 = token_parser.TokenFieldbits3_3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3 }))
    }
}
#[derive(Clone, Debug)]
enum TableRstAddr {
    Var0(RstAddrVar0),
}
impl TableRstAddr {
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
            RstAddrVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:347:1, end:347:8))"]
#[derive(Clone, Debug)]
struct IOAddr8Var0 {
    imm8: TokenField_imm8,
}
impl IOAddr8Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("("),
            self.imm8.display(),
            DisplayElement::Literal(")"),
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
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[derive(Clone, Debug)]
enum TableIOAddr8 {
    Var0(IOAddr8Var0),
}
impl TableIOAddr8 {
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
            IOAddr8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:349:1, end:349:9))"]
#[derive(Clone, Debug)]
struct IOAddr8aVar0 {
    imm8: TokenField_imm8,
}
impl IOAddr8aVar0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("("),
            self.imm8.display(),
            DisplayElement::Literal(")"),
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
        if context_instance
            .register()
            .read_assume8bitIOSpace_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:350:1, end:350:9))"]
#[derive(Clone, Debug)]
struct IOAddr8aVar1 {
    imm8: TokenField_imm8,
}
impl IOAddr8aVar1 {
    fn display_extend<T>(
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
            DisplayElement::Literal("("),
            self.imm8.display(),
            DisplayElement::Literal(")"),
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
        if context_instance
            .register()
            .read_assume8bitIOSpace_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[derive(Clone, Debug)]
enum TableIOAddr8a {
    Var0(IOAddr8aVar0),
    Var1(IOAddr8aVar1),
}
impl TableIOAddr8a {
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
            IOAddr8aVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            IOAddr8aVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:352:1, end:352:8))"]
#[derive(Clone, Debug)]
struct IOAddrCVar0 {}
impl IOAddrCVar0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::C),
            DisplayElement::Literal(")"),
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
        let mut block_0_len = 0u64 as u16;
        if context_instance
            .register()
            .read_assume8bitIOSpace_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:353:1, end:353:8))"]
#[derive(Clone, Debug)]
struct IOAddrCVar1 {}
impl IOAddrCVar1 {
    fn display_extend<T>(
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
            DisplayElement::Literal("("),
            DisplayElement::Register(Register::C),
            DisplayElement::Literal(")"),
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
        let mut block_0_len = 0u64 as u16;
        if context_instance
            .register()
            .read_assume8bitIOSpace_disassembly()
            .unwrap()
            != 1i128
        {
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
enum TableIOAddrC {
    Var0(IOAddrCVar0),
    Var1(IOAddrCVar1),
}
impl TableIOAddrC {
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
            IOAddrCVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            IOAddrCVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:361:1, end:361:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NZ")];
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
        if token_parser.TokenFieldbits3_3().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:362:1, end:362:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("Z")];
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
        if token_parser.TokenFieldbits3_3().disassembly() != 1i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:363:1, end:363:3))"]
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
        if token_parser.TokenFieldbits3_3().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:364:1, end:364:3))"]
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
        if token_parser.TokenFieldbits3_3().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:365:1, end:365:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("PO")];
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
        if token_parser.TokenFieldbits3_3().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:366:1, end:366:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("PE")];
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
        if token_parser.TokenFieldbits3_3().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:367:1, end:367:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("P")];
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
        if token_parser.TokenFieldbits3_3().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:368:1, end:368:3))"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("M")];
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
        if token_parser.TokenFieldbits3_3().disassembly() != 7i128 {
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
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:370:1, end:370:4))"]
#[derive(Clone, Debug)]
struct cc2Var0 {}
impl cc2Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NZ")];
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
        if token_parser.TokenFieldbits3_3().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:371:1, end:371:4))"]
#[derive(Clone, Debug)]
struct cc2Var1 {}
impl cc2Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("Z")];
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
        if token_parser.TokenFieldbits3_3().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:372:1, end:372:4))"]
#[derive(Clone, Debug)]
struct cc2Var2 {}
impl cc2Var2 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldbits3_3().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:373:1, end:373:4))"]
#[derive(Clone, Debug)]
struct cc2Var3 {}
impl cc2Var3 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldbits3_3().disassembly() != 7i128 {
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
enum Tablecc2 {
    Var0(cc2Var0),
    Var1(cc2Var1),
    Var2(cc2Var2),
    Var3(cc2Var3),
}
impl Tablecc2 {
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
            cc2Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            cc2Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            cc2Var2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            cc2Var3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
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
