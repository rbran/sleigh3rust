use sleigh4rust::*;
pub type AddrType = u16;
pub trait GlobalSetTrait {
    fn set_doPseudo(&mut self, address: Option<u16>, value: i128);
    fn set_possibleSkip(&mut self, address: Option<u16>, value: i128);
}
#[derive(Default)]
pub struct GlobalSetDefault<C: ContextTrait>(
    pub std::collections::HashMap<AddrType, C>,
);
impl<C: ContextTrait> GlobalSetTrait for GlobalSetDefault<C> {
    fn set_doPseudo(&mut self, inst_start: Option<AddrType>, value: i128) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_doPseudo_disassembly(value)
                .unwrap();
            context
        });
    }
    fn set_possibleSkip(&mut self, inst_start: Option<AddrType>, value: i128) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_possibleSkip_disassembly(value)
                .unwrap();
            context
        });
    }
}
pub trait ContextregisterTrait:
    MemoryRead<AddressType = u16> + MemoryWrite
{
    fn read_doPseudo_raw(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u8::<false>(0, 0, 1)?;
        Ok(u8::try_from(work_value).unwrap())
    }
    fn write_doPseudo_raw(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u8::<false>(u8::from(param), 0, 0, 1)
    }
    fn read_doPseudo_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_doPseudo_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_doPseudo_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_doPseudo_raw(param as u8)
    }
    fn read_doPseudo_execution(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        self.read_doPseudo_raw()
    }
    fn write_doPseudo_execution(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_doPseudo_raw(param)
    }
    fn doPseudo_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_doPseudo_raw()?))
    }
    fn read_possibleSkip_raw(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u8::<false>(0, 1, 1)?;
        Ok(u8::try_from(work_value).unwrap())
    }
    fn write_possibleSkip_raw(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u8::<false>(u8::from(param), 0, 1, 1)
    }
    fn read_possibleSkip_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_possibleSkip_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_possibleSkip_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_possibleSkip_raw(param as u8)
    }
    fn read_possibleSkip_execution(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        self.read_possibleSkip_raw()
    }
    fn write_possibleSkip_execution(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_possibleSkip_raw(param)
    }
    fn possibleSkip_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_possibleSkip_raw()?))
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
        0 => Register::FSR0,
        1 => Register::FSR1,
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
        0 => Register::INDF0,
        1 => Register::INDF1,
        2 => Register::PCL,
        3 => Register::STATUS,
        4 => Register::FSR0L,
        5 => Register::FSR0H,
        6 => Register::FSR1L,
        7 => Register::FSR1H,
        8 => Register::BSR,
        9 => Register::W,
        10 => Register::PCLATH,
        11 => Register::INTCON,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_0_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => DisplayElement::Literal("IOCIF"),
        1 => DisplayElement::Literal("INTF"),
        2 => DisplayElement::Literal("TMR0IF"),
        3 => DisplayElement::Literal("IOCIE"),
        4 => DisplayElement::Literal("INTE"),
        5 => DisplayElement::Literal("TMR0IE"),
        6 => DisplayElement::Literal("PEIE"),
        7 => DisplayElement::Literal("GIE"),
        _ => unreachable!("Invalid Attach Value"),
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op14(u16);
impl TokenField_op14 {
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
struct TokenField_op12(u16);
impl TokenField_op12 {
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
struct TokenField_op11(u16);
impl TokenField_op11 {
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
struct TokenField_op9(u16);
impl TokenField_op9 {
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
struct TokenField_op7(u8);
impl TokenField_op7 {
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
struct TokenField_op6(u8);
impl TokenField_op6 {
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
struct TokenField_op5(u8);
impl TokenField_op5 {
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
struct TokenField_op4(u8);
impl TokenField_op4 {
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
struct TokenField_op3(u8);
impl TokenField_op3 {
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
struct TokenField_d(u8);
impl TokenField_d {
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
struct TokenField_b3(u8);
impl TokenField_b3 {
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
struct TokenField_IntConBits(u8);
impl TokenField_IntConBits {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_0_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_StatusBit(u8);
impl TokenField_StatusBit {
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
struct TokenField_f7(u8);
impl TokenField_f7 {
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
struct TokenField_lf7(u8);
impl TokenField_lf7 {
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
struct TokenField_uf7(u8);
impl TokenField_uf7 {
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
struct TokenField_fsr(u8);
impl TokenField_fsr {
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
struct TokenField_fsrk(u8);
impl TokenField_fsrk {
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
struct TokenField_fregCore(u8);
impl TokenField_fregCore {
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
struct TokenField_k5(u8);
impl TokenField_k5 {
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
struct TokenField_k7(u8);
impl TokenField_k7 {
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
struct TokenField_k8(u8);
impl TokenField_k8 {
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
struct TokenField_sk9(i16);
impl TokenField_sk9 {
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
#[derive(Clone, Copy, Debug)]
struct TokenField_k11(u16);
impl TokenField_k11 {
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
struct TokenField_sk6(i8);
impl TokenField_sk6 {
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
struct TokenField_l5(u8);
impl TokenField_l5 {
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
struct TokenField_mm(u8);
impl TokenField_mm {
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
    fn TokenFieldop14(&self) -> TokenField_op14 {
        let inner_value = self.read_u16::<false>(0, 0, 14).unwrap();
        TokenField_op14(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldop12(&self) -> TokenField_op12 {
        let inner_value = self.read_u16::<false>(0, 2, 12).unwrap();
        TokenField_op12(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldop11(&self) -> TokenField_op11 {
        let inner_value = self.read_u16::<false>(0, 3, 11).unwrap();
        TokenField_op11(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldop9(&self) -> TokenField_op9 {
        let inner_value = self.read_u16::<false>(0, 5, 9).unwrap();
        TokenField_op9(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldop7(&self) -> TokenField_op7 {
        let inner_value = self.read_u16::<false>(0, 7, 7).unwrap();
        TokenField_op7(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop6(&self) -> TokenField_op6 {
        let inner_value = self.read_u8::<false>(1, 0, 6).unwrap();
        TokenField_op6(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop5(&self) -> TokenField_op5 {
        let inner_value = self.read_u8::<false>(1, 1, 5).unwrap();
        TokenField_op5(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop4(&self) -> TokenField_op4 {
        let inner_value = self.read_u8::<false>(1, 2, 4).unwrap();
        TokenField_op4(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop3(&self) -> TokenField_op3 {
        let inner_value = self.read_u8::<false>(1, 3, 3).unwrap();
        TokenField_op3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldd(&self) -> TokenField_d {
        let inner_value = self.read_u8::<false>(0, 7, 1).unwrap();
        TokenField_d(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldb3(&self) -> TokenField_b3 {
        let inner_value = self.read_u16::<false>(0, 7, 3).unwrap();
        TokenField_b3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldIntConBits(&self) -> TokenField_IntConBits {
        let inner_value = self.read_u16::<false>(0, 7, 3).unwrap();
        TokenField_IntConBits(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldStatusBit(&self) -> TokenField_StatusBit {
        let inner_value = self.read_u16::<false>(0, 7, 3).unwrap();
        TokenField_StatusBit(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldf7(&self) -> TokenField_f7 {
        let inner_value = self.read_u8::<false>(0, 0, 7).unwrap();
        TokenField_f7(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlf7(&self) -> TokenField_lf7 {
        let inner_value = self.read_u8::<false>(0, 0, 4).unwrap();
        TokenField_lf7(u8::try_from(inner_value).unwrap())
    }
    fn TokenFielduf7(&self) -> TokenField_uf7 {
        let inner_value = self.read_u8::<false>(0, 4, 3).unwrap();
        TokenField_uf7(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfsr(&self) -> TokenField_fsr {
        let inner_value = self.read_u8::<false>(0, 2, 1).unwrap();
        TokenField_fsr(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfsrk(&self) -> TokenField_fsrk {
        let inner_value = self.read_u8::<false>(0, 6, 1).unwrap();
        TokenField_fsrk(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfregCore(&self) -> TokenField_fregCore {
        let inner_value = self.read_u8::<false>(0, 0, 4).unwrap();
        TokenField_fregCore(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldk5(&self) -> TokenField_k5 {
        let inner_value = self.read_u8::<false>(0, 0, 5).unwrap();
        TokenField_k5(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldk7(&self) -> TokenField_k7 {
        let inner_value = self.read_u8::<false>(0, 0, 7).unwrap();
        TokenField_k7(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldk8(&self) -> TokenField_k8 {
        let inner_value = self.read_u8::<false>(0, 0, 8).unwrap();
        TokenField_k8(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsk9(&self) -> TokenField_sk9 {
        let inner_value = self.read_i16::<false>(0, 0, 9).unwrap();
        TokenField_sk9(i16::try_from(inner_value).unwrap())
    }
    fn TokenFieldk11(&self) -> TokenField_k11 {
        let inner_value = self.read_u16::<false>(0, 0, 11).unwrap();
        TokenField_k11(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldsk6(&self) -> TokenField_sk6 {
        let inner_value = self.read_i8::<false>(0, 0, 6).unwrap();
        TokenField_sk6(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldl5(&self) -> TokenField_l5 {
        let inner_value = self.read_u8::<false>(0, 0, 5).unwrap();
        TokenField_l5(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldmm(&self) -> TokenField_mm {
        let inner_value = self.read_u8::<false>(0, 0, 2).unwrap();
        TokenField_mm(u8::try_from(inner_value).unwrap())
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    PC,
    STKPTR,
    W,
    SkipNext,
    IRP,
    RP,
    INDF0,
    INDF1,
    PCL,
    STATUS,
    FSR0L,
    FSR0H,
    FSR1L,
    FSR1H,
    BSR,
    WREG,
    PCLATH,
    INTCON,
    FSR0,
    FSR1,
    contextreg,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PC => write!(f, "PC"),
            Self::STKPTR => write!(f, "STKPTR"),
            Self::W => write!(f, "W"),
            Self::SkipNext => write!(f, "SkipNext"),
            Self::IRP => write!(f, "IRP"),
            Self::RP => write!(f, "RP"),
            Self::INDF0 => write!(f, "INDF0"),
            Self::INDF1 => write!(f, "INDF1"),
            Self::PCL => write!(f, "PCL"),
            Self::STATUS => write!(f, "STATUS"),
            Self::FSR0L => write!(f, "FSR0L"),
            Self::FSR0H => write!(f, "FSR0H"),
            Self::FSR1L => write!(f, "FSR1L"),
            Self::FSR1H => write!(f, "FSR1H"),
            Self::BSR => write!(f, "BSR"),
            Self::WREG => write!(f, "WREG"),
            Self::PCLATH => write!(f, "PCLATH"),
            Self::INTCON => write!(f, "INTCON"),
            Self::FSR0 => write!(f, "FSR0"),
            Self::FSR1 => write!(f, "FSR1"),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:520:1, end:520:2))"]
#[derive(Clone, Debug)]
struct BC_instructionVar0 {
    bit: Tablebit,
    absAddr11: TableabsAddr11,
}
impl BC_instructionVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.absAddr11.display_extend(
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
        if context_instance
            .register()
            .read_doPseudo_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop4().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop3().disassembly() != 5i128 {
            return None;
        }
        let absAddr11 = if let Some((len, table)) = TableabsAddr11::parse(
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
        Some((pattern_len, Self { bit, absAddr11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:567:1, end:567:2))"]
#[derive(Clone, Debug)]
struct BZ_instructionVar1 {
    bit: Tablebit,
    absAddr11: TableabsAddr11,
}
impl BZ_instructionVar1 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BZ"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.absAddr11.display_extend(
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
        if context_instance
            .register()
            .read_doPseudo_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop4().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop3().disassembly() != 5i128 {
            return None;
        }
        let absAddr11 = if let Some((len, table)) = TableabsAddr11::parse(
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
        Some((pattern_len, Self { bit, absAddr11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:587:1, end:587:2))"]
#[derive(Clone, Debug)]
struct BNC_instructionVar2 {
    bit: Tablebit,
    absAddr11: TableabsAddr11,
}
impl BNC_instructionVar2 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BNC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.absAddr11.display_extend(
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
        if context_instance
            .register()
            .read_doPseudo_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop4().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop3().disassembly() != 5i128 {
            return None;
        }
        let absAddr11 = if let Some((len, table)) = TableabsAddr11::parse(
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
        Some((pattern_len, Self { bit, absAddr11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:604:1, end:604:2))"]
#[derive(Clone, Debug)]
struct BNZ_instructionVar3 {
    bit: Tablebit,
    absAddr11: TableabsAddr11,
}
impl BNZ_instructionVar3 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BNZ"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.absAddr11.display_extend(
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
        if context_instance
            .register()
            .read_doPseudo_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop4().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop3().disassembly() != 5i128 {
            return None;
        }
        let absAddr11 = if let Some((len, table)) = TableabsAddr11::parse(
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
        Some((pattern_len, Self { bit, absAddr11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:316:1, end:316:2))"]
#[derive(Clone, Debug)]
struct ADDWF_instructionVar4 {
    D: TableD,
}
impl ADDWF_instructionVar4 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADDWF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::PC),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldd().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 2i128 {
            return None;
        }
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { D }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:346:1, end:346:2))"]
#[derive(Clone, Debug)]
struct ADDWFC_instructionVar5 {
    D: TableD,
}
impl ADDWFC_instructionVar5 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADDWFC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::PC),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 61i128 {
            return None;
        }
        if token_parser.TokenFieldd().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 2i128 {
            return None;
        }
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { D }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:409:1, end:409:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar6 {}
impl BCF_instructionVar6 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BCF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("C"),
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
        if token_parser.TokenFieldop4().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:415:1, end:415:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar7 {
    bit: Tablebit,
}
impl BCF_instructionVar7 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BCF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("DC"),
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
        if token_parser.TokenFieldop4().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:422:1, end:422:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar8 {
    bit: Tablebit,
}
impl BCF_instructionVar8 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BCF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("Z"),
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
        if token_parser.TokenFieldop4().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:429:1, end:429:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar9 {
    bit: Tablebit,
}
impl BCF_instructionVar9 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BCF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("RP0"),
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
        if token_parser.TokenFieldop4().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:438:1, end:438:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar10 {
    bit: Tablebit,
}
impl BCF_instructionVar10 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BCF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("RP1"),
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
        if token_parser.TokenFieldop4().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:447:1, end:447:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar11 {
    bit: Tablebit,
}
impl BCF_instructionVar11 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BCF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("IRP"),
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
        if token_parser.TokenFieldop4().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:468:1, end:468:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar12 {}
impl BSF_instructionVar12 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BSF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("C"),
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
        if token_parser.TokenFieldop4().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:474:1, end:474:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar13 {}
impl BSF_instructionVar13 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BSF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("DC"),
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
        if token_parser.TokenFieldop4().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:480:1, end:480:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar14 {
    bit: Tablebit,
}
impl BSF_instructionVar14 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BSF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("Z"),
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
        if token_parser.TokenFieldop4().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:486:1, end:486:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar15 {
    bit: Tablebit,
}
impl BSF_instructionVar15 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BSF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("RP0"),
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
        if token_parser.TokenFieldop4().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:494:1, end:494:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar16 {
    bit: Tablebit,
}
impl BSF_instructionVar16 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BSF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("RP1"),
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
        if token_parser.TokenFieldop4().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:503:1, end:503:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar17 {
    bit: Tablebit,
}
impl BSF_instructionVar17 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BSF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("IRP"),
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
        if token_parser.TokenFieldop4().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:531:1, end:531:2))"]
#[derive(Clone, Debug)]
struct BRW_instructionVar18 {}
impl BRW_instructionVar18 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("BRW")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop14().disassembly() != 11i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:539:1, end:539:2))"]
#[derive(Clone, Debug)]
struct BTFSC_instructionVar19 {
    bit: Tablebit,
}
impl BTFSC_instructionVar19 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("BTFSC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
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
        if token_parser.TokenFieldop4().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:545:1, end:545:2))"]
#[derive(Clone, Debug)]
struct SKPC_instructionVar20 {
    bit: Tablebit,
}
impl SKPC_instructionVar20 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("SKPC")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
            .read_doPseudo_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop4().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:549:1, end:549:2))"]
#[derive(Clone, Debug)]
struct SKPNC_instructionVar21 {
    bit: Tablebit,
}
impl SKPNC_instructionVar21 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("SKPNC")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
            .read_doPseudo_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop4().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:553:1, end:553:2))"]
#[derive(Clone, Debug)]
struct SKPZ_instructionVar22 {
    bit: Tablebit,
}
impl SKPZ_instructionVar22 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("SKPZ")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
            .read_doPseudo_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop4().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:557:1, end:557:2))"]
#[derive(Clone, Debug)]
struct SKPNZ_instructionVar23 {
    bit: Tablebit,
}
impl SKPNZ_instructionVar23 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("SKPNZ")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
            .read_doPseudo_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop4().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:561:1, end:561:2))"]
#[derive(Clone, Debug)]
struct BTFSC_instructionVar24 {
    bit: Tablebit,
}
impl BTFSC_instructionVar24 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("BTFSC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
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
        if token_parser.TokenFieldop4().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:572:1, end:572:2))"]
#[derive(Clone, Debug)]
struct BTFSC_instructionVar25 {
    bit: Tablebit,
}
impl BTFSC_instructionVar25 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("BTFSC"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
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
        if token_parser.TokenFieldop4().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:592:1, end:592:2))"]
#[derive(Clone, Debug)]
struct BTFSS_instructionVar26 {
    bit: Tablebit,
}
impl BTFSS_instructionVar26 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("BTFSS"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
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
        if token_parser.TokenFieldop4().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:598:1, end:598:2))"]
#[derive(Clone, Debug)]
struct BTFSS_instructionVar27 {
    bit: Tablebit,
}
impl BTFSS_instructionVar27 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("BTFSS"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
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
        if token_parser.TokenFieldop4().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:609:1, end:609:2))"]
#[derive(Clone, Debug)]
struct BTFSS_instructionVar28 {
    bit: Tablebit,
}
impl BTFSS_instructionVar28 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("BTFSS"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
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
        if token_parser.TokenFieldop4().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:623:1, end:623:2))"]
#[derive(Clone, Debug)]
struct CALLW_instructionVar29 {}
impl CALLW_instructionVar29 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("CALLW")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop14().disassembly() != 10i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:637:1, end:637:2))"]
#[derive(Clone, Debug)]
struct CLRF_instructionVar30 {}
impl CLRF_instructionVar30 {
    fn display_extend<T>(
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
            DisplayElement::Literal("CLRF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::STATUS),
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
        if token_parser.TokenFieldop6().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldd().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:655:1, end:655:2))"]
#[derive(Clone, Debug)]
struct CLRWDT_instructionVar31 {}
impl CLRWDT_instructionVar31 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("CLRWDT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop14().disassembly() != 100i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:819:1, end:819:2))"]
#[derive(Clone, Debug)]
struct MOVWF_instructionVar32 {}
impl MOVWF_instructionVar32 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVWF"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::PC),
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
        if token_parser.TokenFieldop6().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldd().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:831:1, end:831:2))"]
#[derive(Clone, Debug)]
struct OPTION_instructionVar33 {}
impl OPTION_instructionVar33 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("OPTION")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop14().disassembly() != 98i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:836:1, end:836:2))"]
#[derive(Clone, Debug)]
struct RESET_instructionVar34 {}
impl RESET_instructionVar34 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RESET")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop14().disassembly() != 1i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:841:1, end:841:2))"]
#[derive(Clone, Debug)]
struct RETFIE_instructionVar35 {}
impl RETFIE_instructionVar35 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RETFIE")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop14().disassembly() != 9i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:858:1, end:858:2))"]
#[derive(Clone, Debug)]
struct RETURN_instructionVar36 {}
impl RETURN_instructionVar36 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RETURN")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop14().disassembly() != 8i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:893:1, end:893:2))"]
#[derive(Clone, Debug)]
struct SLEEP_instructionVar37 {}
impl SLEEP_instructionVar37 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("SLEEP")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop14().disassembly() != 99i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:648:1, end:648:2))"]
#[derive(Clone, Debug)]
struct CLRW_instructionVar38 {}
impl CLRW_instructionVar38 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("CLRW")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop12().disassembly() != 64i128 {
            return None;
        }
        let mm = token_parser.TokenFieldmm();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:768:1, end:768:2))"]
#[derive(Clone, Debug)]
struct MOVIW_instructionVar39 {
    srcFSR: TablesrcFSR,
}
impl MOVIW_instructionVar39 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVIW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcFSR.display_extend(
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
        if token_parser.TokenFieldop11().disassembly() != 2i128 {
            return None;
        }
        let srcFSR = if let Some((len, table)) = TablesrcFSR::parse(
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
        Some((pattern_len, Self { srcFSR }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:778:1, end:778:2))"]
#[derive(Clone, Debug)]
struct MOVWI_instructionVar40 {
    srcFSR: TablesrcFSR,
}
impl MOVWI_instructionVar40 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVWI"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcFSR.display_extend(
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
        if token_parser.TokenFieldop11().disassembly() != 3i128 {
            return None;
        }
        let srcFSR = if let Some((len, table)) = TablesrcFSR::parse(
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
        Some((pattern_len, Self { srcFSR }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:786:1, end:786:2))"]
#[derive(Clone, Debug)]
struct MOVLB_instructionVar41 {
    imm5: Tableimm5,
}
impl MOVLB_instructionVar41 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVLB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm5.display_extend(
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
        if token_parser.TokenFieldop9().disassembly() != 1i128 {
            return None;
        }
        let imm5 = if let Some((len, table)) =
            Tableimm5::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:827:1, end:827:2))"]
#[derive(Clone, Debug)]
struct NOP_instructionVar42 {}
impl NOP_instructionVar42 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldd().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldl5().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:947:1, end:947:2))"]
#[derive(Clone, Debug)]
struct TRIS_instructionVar43 {
    trisREG: TabletrisREG,
}
impl TRIS_instructionVar43 {
    fn display_extend<T>(
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
            DisplayElement::Literal("TRIS"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.trisREG.display_extend(
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
        if token_parser.TokenFieldop9().disassembly() != 3i128 {
            return None;
        }
        let trisREG = if let Some((len, table)) = TabletrisREG::parse(
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
        Some((pattern_len, Self { trisREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:290:1, end:290:2))"]
#[derive(Clone, Debug)]
struct ADDFSR_instructionVar44 {
    fsrk: TokenField_fsrk,
    sk6: TokenField_sk6,
}
impl ADDFSR_instructionVar44 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADDFSR"),
            DisplayElement::Literal(" "),
            self.fsrk.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.sk6.display(),
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
        if token_parser.TokenFieldop7().disassembly() != 98i128 {
            return None;
        }
        let fsrk = token_parser.TokenFieldfsrk();
        let sk6 = token_parser.TokenFieldsk6();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fsrk, sk6 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:629:1, end:629:2))"]
#[derive(Clone, Debug)]
struct CLRF_instructionVar45 {
    srcREG: TablesrcREG,
}
impl CLRF_instructionVar45 {
    fn display_extend<T>(
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
            DisplayElement::Literal("CLRF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldd().disassembly() != 1i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
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
        Some((pattern_len, Self { srcREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:773:1, end:773:2))"]
#[derive(Clone, Debug)]
struct MOVIW_instructionVar46 {
    srcFSRk: TablesrcFSRk,
}
impl MOVIW_instructionVar46 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVIW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcFSRk.display_extend(
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
        if token_parser.TokenFieldop7().disassembly() != 126i128 {
            return None;
        }
        let srcFSRk = if let Some((len, table)) = TablesrcFSRk::parse(
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
        Some((pattern_len, Self { srcFSRk }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:782:1, end:782:2))"]
#[derive(Clone, Debug)]
struct MOVWI_instructionVar47 {
    srcFSRk: TablesrcFSRk,
}
impl MOVWI_instructionVar47 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVWI"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcFSRk.display_extend(
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
        if token_parser.TokenFieldop7().disassembly() != 127i128 {
            return None;
        }
        let srcFSRk = if let Some((len, table)) = TablesrcFSRk::parse(
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
        Some((pattern_len, Self { srcFSRk }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:790:1, end:790:2))"]
#[derive(Clone, Debug)]
struct MOVLP_instructionVar48 {
    imm7: Tableimm7,
}
impl MOVLP_instructionVar48 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVLP"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm7.display_extend(
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
        if token_parser.TokenFieldop7().disassembly() != 99i128 {
            return None;
        }
        let imm7 = if let Some((len, table)) =
            Tableimm7::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm7 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:812:1, end:812:2))"]
#[derive(Clone, Debug)]
struct MOVWF_instructionVar49 {
    srcREG: TablesrcREG,
}
impl MOVWF_instructionVar49 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldd().disassembly() != 1i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
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
        Some((pattern_len, Self { srcREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:295:1, end:295:2))"]
#[derive(Clone, Debug)]
struct ADDLW_instructionVar50 {
    imm8: Tableimm8,
}
impl ADDLW_instructionVar50 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADDLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm8.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 62i128 {
            return None;
        }
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:303:1, end:303:2))"]
#[derive(Clone, Debug)]
struct ADDWF_instructionVar51 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl ADDWF_instructionVar51 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADDWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 7i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:330:1, end:330:2))"]
#[derive(Clone, Debug)]
struct ADDWFC_instructionVar52 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl ADDWFC_instructionVar52 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADDWFC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 61i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:368:1, end:368:2))"]
#[derive(Clone, Debug)]
struct ANDLW_instructionVar53 {
    imm8: Tableimm8,
}
impl ANDLW_instructionVar53 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ANDLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm8.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 57i128 {
            return None;
        }
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:375:1, end:375:2))"]
#[derive(Clone, Debug)]
struct ANDWF_instructionVar54 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl ANDWF_instructionVar54 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ANDWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 5i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:387:1, end:387:2))"]
#[derive(Clone, Debug)]
struct ASRF_instructionVar55 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl ASRF_instructionVar55 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ASRF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 55i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:404:1, end:404:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar56 {
    IntConBits: TokenField_IntConBits,
    bit: Tablebit,
    srcREG: TablesrcREG,
}
impl BCF_instructionVar56 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.IntConBits.display(),
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
        if token_parser.TokenFieldop4().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 11i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let IntConBits = token_parser.TokenFieldIntConBits();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                bit,
                srcREG,
                IntConBits,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:463:1, end:463:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar57 {
    IntConBits: TokenField_IntConBits,
    bit: Tablebit,
    srcREG: TablesrcREG,
}
impl BSF_instructionVar57 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.IntConBits.display(),
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
        if token_parser.TokenFieldop4().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 11i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let IntConBits = token_parser.TokenFieldIntConBits();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                bit,
                srcREG,
                IntConBits,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:661:1, end:661:2))"]
#[derive(Clone, Debug)]
struct COMF_instructionVar58 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl COMF_instructionVar58 {
    fn display_extend<T>(
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
            DisplayElement::Literal("COMF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 9i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:672:1, end:672:2))"]
#[derive(Clone, Debug)]
struct DECF_instructionVar59 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl DECF_instructionVar59 {
    fn display_extend<T>(
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
            DisplayElement::Literal("DECF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 3i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:683:1, end:683:2))"]
#[derive(Clone, Debug)]
struct DECFSZ_instructionVar60 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl DECFSZ_instructionVar60 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("DECFSZ"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 11i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:701:1, end:701:2))"]
#[derive(Clone, Debug)]
struct INCF_instructionVar61 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl INCF_instructionVar61 {
    fn display_extend<T>(
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
            DisplayElement::Literal("INCF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 10i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:712:1, end:712:2))"]
#[derive(Clone, Debug)]
struct INCFSZ_instructionVar62 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl INCFSZ_instructionVar62 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("INCFSZ"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 15i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:723:1, end:723:2))"]
#[derive(Clone, Debug)]
struct IORLW_instructionVar63 {
    imm8: Tableimm8,
}
impl IORLW_instructionVar63 {
    fn display_extend<T>(
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
            DisplayElement::Literal("IORLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm8.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 56i128 {
            return None;
        }
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:730:1, end:730:2))"]
#[derive(Clone, Debug)]
struct IORWF_instructionVar64 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl IORWF_instructionVar64 {
    fn display_extend<T>(
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
            DisplayElement::Literal("IORWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 4i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:752:1, end:752:2))"]
#[derive(Clone, Debug)]
struct LSLF_instructionVar65 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl LSLF_instructionVar65 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LSLF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 53i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:760:1, end:760:2))"]
#[derive(Clone, Debug)]
struct LSRF_instructionVar66 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl LSRF_instructionVar66 {
    fn display_extend<T>(
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
            DisplayElement::Literal("LSRF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 54i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:795:1, end:795:2))"]
#[derive(Clone, Debug)]
struct MOVLW_instructionVar67 {
    imm8: Tableimm8,
}
impl MOVLW_instructionVar67 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm8.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 48i128 {
            return None;
        }
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:801:1, end:801:2))"]
#[derive(Clone, Debug)]
struct MOVF_instructionVar68 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl MOVF_instructionVar68 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 8i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:865:1, end:865:2))"]
#[derive(Clone, Debug)]
struct RLF_instructionVar69 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl RLF_instructionVar69 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RLF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 13i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:879:1, end:879:2))"]
#[derive(Clone, Debug)]
struct RRF_instructionVar70 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl RRF_instructionVar70 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RRF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 12i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:899:1, end:899:2))"]
#[derive(Clone, Debug)]
struct SUBLW_instructionVar71 {
    imm8: Tableimm8,
}
impl SUBLW_instructionVar71 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SUBLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm8.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 60i128 {
            return None;
        }
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:907:1, end:907:2))"]
#[derive(Clone, Debug)]
struct SUBWF_instructionVar72 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl SUBWF_instructionVar72 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SUBWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 2i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:920:1, end:920:2))"]
#[derive(Clone, Debug)]
struct SUBWFB_instructionVar73 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl SUBWFB_instructionVar73 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SUBWFB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 59i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:937:1, end:937:2))"]
#[derive(Clone, Debug)]
struct SWAPF_instructionVar74 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl SWAPF_instructionVar74 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SWAPF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 14i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:953:1, end:953:2))"]
#[derive(Clone, Debug)]
struct XORLW_instructionVar75 {
    imm8: Tableimm8,
}
impl XORLW_instructionVar75 {
    fn display_extend<T>(
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
            DisplayElement::Literal("XORLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm8.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 58i128 {
            return None;
        }
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:960:1, end:960:2))"]
#[derive(Clone, Debug)]
struct XORWF_instructionVar76 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl XORWF_instructionVar76 {
    fn display_extend<T>(
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
            DisplayElement::Literal("XORWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
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
        if token_parser.TokenFieldop6().disassembly() != 6i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:527:1, end:527:2))"]
#[derive(Clone, Debug)]
struct BRA_instructionVar77 {
    relAddr9: TablerelAddr9,
}
impl BRA_instructionVar77 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BRA"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.relAddr9.display_extend(
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
        if token_parser.TokenFieldop5().disassembly() != 25i128 {
            return None;
        }
        let relAddr9 = if let Some((len, table)) = TablerelAddr9::parse(
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
        Some((pattern_len, Self { relAddr9 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:395:1, end:395:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar78 {
    srcREG: TablesrcREG,
    bit: Tablebit,
}
impl BCF_instructionVar78 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
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
        if token_parser.TokenFieldop4().disassembly() != 4i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { srcREG, bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:455:1, end:455:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar79 {
    bit: Tablebit,
    srcREG: TablesrcREG,
}
impl BSF_instructionVar79 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
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
        if token_parser.TokenFieldop4().disassembly() != 5i128 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
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
        Some((pattern_len, Self { bit, srcREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:511:1, end:511:2))"]
#[derive(Clone, Debug)]
struct BTFSC_instructionVar80 {
    bit: Tablebit,
    srcREG: TablesrcREG,
}
impl BTFSC_instructionVar80 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("BTFSC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
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
        if token_parser.TokenFieldop4().disassembly() != 6i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
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
        Some((pattern_len, Self { bit, srcREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:578:1, end:578:2))"]
#[derive(Clone, Debug)]
struct BTFSS_instructionVar81 {
    bit: Tablebit,
    srcREG: TablesrcREG,
}
impl BTFSS_instructionVar81 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_possibleSkip(
            Some(inst_next),
            context.register().read_possibleSkip_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("BTFSS"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
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
        if token_parser.TokenFieldop4().disassembly() != 7i128 {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
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
        Some((pattern_len, Self { bit, srcREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:849:1, end:849:2))"]
#[derive(Clone, Debug)]
struct RETLW_instructionVar82 {
    imm8: Tableimm8,
}
impl RETLW_instructionVar82 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RETLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm8.display_extend(
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
        if token_parser.TokenFieldop4().disassembly() != 13i128 {
            return None;
        }
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:615:1, end:615:2))"]
#[derive(Clone, Debug)]
struct CALL_instructionVar83 {
    absAddr11: TableabsAddr11,
}
impl CALL_instructionVar83 {
    fn display_extend<T>(
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
        self.absAddr11.display_extend(
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
        if token_parser.TokenFieldop3().disassembly() != 4i128 {
            return None;
        }
        let absAddr11 = if let Some((len, table)) = TableabsAddr11::parse(
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
        Some((pattern_len, Self { absAddr11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:694:1, end:694:2))"]
#[derive(Clone, Debug)]
struct GOTO_instructionVar84 {
    absAddr11: TableabsAddr11,
}
impl GOTO_instructionVar84 {
    fn display_extend<T>(
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
            DisplayElement::Literal("GOTO"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.absAddr11.display_extend(
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
        if token_parser.TokenFieldop3().disassembly() != 5i128 {
            return None;
        }
        let absAddr11 = if let Some((len, table)) = TableabsAddr11::parse(
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
        Some((pattern_len, Self { absAddr11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:277:1, end:277:2))"]
#[derive(Clone, Debug)]
struct instructionVar85 {
    instruction: Box<Tableinstruction>,
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
        self.instruction.display_extend(
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
        if context_instance
            .register()
            .read_possibleSkip_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        let tmp = 0i128;
        context_instance
            .register_mut()
            .write_possibleSkip_disassembly(tmp)
            .unwrap();
        let instruction = if let Some((len, table)) = Tableinstruction::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { instruction }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(BC_instructionVar0),
    Var1(BZ_instructionVar1),
    Var2(BNC_instructionVar2),
    Var3(BNZ_instructionVar3),
    Var4(ADDWF_instructionVar4),
    Var5(ADDWFC_instructionVar5),
    Var6(BCF_instructionVar6),
    Var7(BCF_instructionVar7),
    Var8(BCF_instructionVar8),
    Var9(BCF_instructionVar9),
    Var10(BCF_instructionVar10),
    Var11(BCF_instructionVar11),
    Var12(BSF_instructionVar12),
    Var13(BSF_instructionVar13),
    Var14(BSF_instructionVar14),
    Var15(BSF_instructionVar15),
    Var16(BSF_instructionVar16),
    Var17(BSF_instructionVar17),
    Var18(BRW_instructionVar18),
    Var19(BTFSC_instructionVar19),
    Var20(SKPC_instructionVar20),
    Var21(SKPNC_instructionVar21),
    Var22(SKPZ_instructionVar22),
    Var23(SKPNZ_instructionVar23),
    Var24(BTFSC_instructionVar24),
    Var25(BTFSC_instructionVar25),
    Var26(BTFSS_instructionVar26),
    Var27(BTFSS_instructionVar27),
    Var28(BTFSS_instructionVar28),
    Var29(CALLW_instructionVar29),
    Var30(CLRF_instructionVar30),
    Var31(CLRWDT_instructionVar31),
    Var32(MOVWF_instructionVar32),
    Var33(OPTION_instructionVar33),
    Var34(RESET_instructionVar34),
    Var35(RETFIE_instructionVar35),
    Var36(RETURN_instructionVar36),
    Var37(SLEEP_instructionVar37),
    Var38(CLRW_instructionVar38),
    Var39(MOVIW_instructionVar39),
    Var40(MOVWI_instructionVar40),
    Var41(MOVLB_instructionVar41),
    Var42(NOP_instructionVar42),
    Var43(TRIS_instructionVar43),
    Var44(ADDFSR_instructionVar44),
    Var45(CLRF_instructionVar45),
    Var46(MOVIW_instructionVar46),
    Var47(MOVWI_instructionVar47),
    Var48(MOVLP_instructionVar48),
    Var49(MOVWF_instructionVar49),
    Var50(ADDLW_instructionVar50),
    Var51(ADDWF_instructionVar51),
    Var52(ADDWFC_instructionVar52),
    Var53(ANDLW_instructionVar53),
    Var54(ANDWF_instructionVar54),
    Var55(ASRF_instructionVar55),
    Var56(BCF_instructionVar56),
    Var57(BSF_instructionVar57),
    Var58(COMF_instructionVar58),
    Var59(DECF_instructionVar59),
    Var60(DECFSZ_instructionVar60),
    Var61(INCF_instructionVar61),
    Var62(INCFSZ_instructionVar62),
    Var63(IORLW_instructionVar63),
    Var64(IORWF_instructionVar64),
    Var65(LSLF_instructionVar65),
    Var66(LSRF_instructionVar66),
    Var67(MOVLW_instructionVar67),
    Var68(MOVF_instructionVar68),
    Var69(RLF_instructionVar69),
    Var70(RRF_instructionVar70),
    Var71(SUBLW_instructionVar71),
    Var72(SUBWF_instructionVar72),
    Var73(SUBWFB_instructionVar73),
    Var74(SWAPF_instructionVar74),
    Var75(XORLW_instructionVar75),
    Var76(XORWF_instructionVar76),
    Var77(BRA_instructionVar77),
    Var78(BCF_instructionVar78),
    Var79(BSF_instructionVar79),
    Var80(BTFSC_instructionVar80),
    Var81(BTFSS_instructionVar81),
    Var82(RETLW_instructionVar82),
    Var83(CALL_instructionVar83),
    Var84(GOTO_instructionVar84),
    Var85(instructionVar85),
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
        if let Some((inst_len, parsed)) = BC_instructionVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = BZ_instructionVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = BNC_instructionVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = BNZ_instructionVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = ADDWF_instructionVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) = ADDWFC_instructionVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) = BCF_instructionVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) = BCF_instructionVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) = BCF_instructionVar8::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) = BCF_instructionVar9::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) = BCF_instructionVar10::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) = BCF_instructionVar11::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) = BSF_instructionVar12::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) = BSF_instructionVar13::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) = BSF_instructionVar14::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) = BSF_instructionVar15::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        if let Some((inst_len, parsed)) = BSF_instructionVar16::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var16(parsed)));
        }
        if let Some((inst_len, parsed)) = BSF_instructionVar17::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var17(parsed)));
        }
        if let Some((inst_len, parsed)) = BRW_instructionVar18::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var18(parsed)));
        }
        if let Some((inst_len, parsed)) = BTFSC_instructionVar19::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var19(parsed)));
        }
        if let Some((inst_len, parsed)) = SKPC_instructionVar20::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var20(parsed)));
        }
        if let Some((inst_len, parsed)) = SKPNC_instructionVar21::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var21(parsed)));
        }
        if let Some((inst_len, parsed)) = SKPZ_instructionVar22::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var22(parsed)));
        }
        if let Some((inst_len, parsed)) = SKPNZ_instructionVar23::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var23(parsed)));
        }
        if let Some((inst_len, parsed)) = BTFSC_instructionVar24::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var24(parsed)));
        }
        if let Some((inst_len, parsed)) = BTFSC_instructionVar25::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var25(parsed)));
        }
        if let Some((inst_len, parsed)) = BTFSS_instructionVar26::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var26(parsed)));
        }
        if let Some((inst_len, parsed)) = BTFSS_instructionVar27::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var27(parsed)));
        }
        if let Some((inst_len, parsed)) = BTFSS_instructionVar28::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var28(parsed)));
        }
        if let Some((inst_len, parsed)) = CALLW_instructionVar29::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var29(parsed)));
        }
        if let Some((inst_len, parsed)) = CLRF_instructionVar30::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var30(parsed)));
        }
        if let Some((inst_len, parsed)) = CLRWDT_instructionVar31::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var31(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVWF_instructionVar32::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var32(parsed)));
        }
        if let Some((inst_len, parsed)) = OPTION_instructionVar33::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var33(parsed)));
        }
        if let Some((inst_len, parsed)) = RESET_instructionVar34::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var34(parsed)));
        }
        if let Some((inst_len, parsed)) = RETFIE_instructionVar35::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var35(parsed)));
        }
        if let Some((inst_len, parsed)) = RETURN_instructionVar36::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var36(parsed)));
        }
        if let Some((inst_len, parsed)) = SLEEP_instructionVar37::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var37(parsed)));
        }
        if let Some((inst_len, parsed)) = CLRW_instructionVar38::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var38(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVIW_instructionVar39::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var39(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVWI_instructionVar40::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var40(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVLB_instructionVar41::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var41(parsed)));
        }
        if let Some((inst_len, parsed)) = NOP_instructionVar42::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var42(parsed)));
        }
        if let Some((inst_len, parsed)) = TRIS_instructionVar43::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var43(parsed)));
        }
        if let Some((inst_len, parsed)) = ADDFSR_instructionVar44::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var44(parsed)));
        }
        if let Some((inst_len, parsed)) = CLRF_instructionVar45::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var45(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVIW_instructionVar46::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var46(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVWI_instructionVar47::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var47(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVLP_instructionVar48::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var48(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVWF_instructionVar49::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var49(parsed)));
        }
        if let Some((inst_len, parsed)) = ADDLW_instructionVar50::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var50(parsed)));
        }
        if let Some((inst_len, parsed)) = ADDWF_instructionVar51::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var51(parsed)));
        }
        if let Some((inst_len, parsed)) = ADDWFC_instructionVar52::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var52(parsed)));
        }
        if let Some((inst_len, parsed)) = ANDLW_instructionVar53::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var53(parsed)));
        }
        if let Some((inst_len, parsed)) = ANDWF_instructionVar54::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var54(parsed)));
        }
        if let Some((inst_len, parsed)) = ASRF_instructionVar55::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var55(parsed)));
        }
        if let Some((inst_len, parsed)) = BCF_instructionVar56::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var56(parsed)));
        }
        if let Some((inst_len, parsed)) = BSF_instructionVar57::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var57(parsed)));
        }
        if let Some((inst_len, parsed)) = COMF_instructionVar58::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var58(parsed)));
        }
        if let Some((inst_len, parsed)) = DECF_instructionVar59::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var59(parsed)));
        }
        if let Some((inst_len, parsed)) = DECFSZ_instructionVar60::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var60(parsed)));
        }
        if let Some((inst_len, parsed)) = INCF_instructionVar61::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var61(parsed)));
        }
        if let Some((inst_len, parsed)) = INCFSZ_instructionVar62::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var62(parsed)));
        }
        if let Some((inst_len, parsed)) = IORLW_instructionVar63::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var63(parsed)));
        }
        if let Some((inst_len, parsed)) = IORWF_instructionVar64::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var64(parsed)));
        }
        if let Some((inst_len, parsed)) = LSLF_instructionVar65::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var65(parsed)));
        }
        if let Some((inst_len, parsed)) = LSRF_instructionVar66::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var66(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVLW_instructionVar67::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var67(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVF_instructionVar68::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var68(parsed)));
        }
        if let Some((inst_len, parsed)) = RLF_instructionVar69::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var69(parsed)));
        }
        if let Some((inst_len, parsed)) = RRF_instructionVar70::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var70(parsed)));
        }
        if let Some((inst_len, parsed)) = SUBLW_instructionVar71::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var71(parsed)));
        }
        if let Some((inst_len, parsed)) = SUBWF_instructionVar72::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var72(parsed)));
        }
        if let Some((inst_len, parsed)) = SUBWFB_instructionVar73::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var73(parsed)));
        }
        if let Some((inst_len, parsed)) = SWAPF_instructionVar74::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var74(parsed)));
        }
        if let Some((inst_len, parsed)) = XORLW_instructionVar75::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var75(parsed)));
        }
        if let Some((inst_len, parsed)) = XORWF_instructionVar76::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var76(parsed)));
        }
        if let Some((inst_len, parsed)) = BRA_instructionVar77::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var77(parsed)));
        }
        if let Some((inst_len, parsed)) = BCF_instructionVar78::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var78(parsed)));
        }
        if let Some((inst_len, parsed)) = BSF_instructionVar79::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var79(parsed)));
        }
        if let Some((inst_len, parsed)) = BTFSC_instructionVar80::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var80(parsed)));
        }
        if let Some((inst_len, parsed)) = BTFSS_instructionVar81::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var81(parsed)));
        }
        if let Some((inst_len, parsed)) = RETLW_instructionVar82::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var82(parsed)));
        }
        if let Some((inst_len, parsed)) = CALL_instructionVar83::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var83(parsed)));
        }
        if let Some((inst_len, parsed)) = GOTO_instructionVar84::parse(
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
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:191:1, end:191:7))"]
#[derive(Clone, Debug)]
struct srcREGVar0 {
    fregCore: TokenField_fregCore,
}
impl srcREGVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.fregCore.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldf7().disassembly() != 0i128 {
            return None;
        }
        let fregCore = token_parser.TokenFieldfregCore();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fregCore }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:196:1, end:196:7))"]
#[derive(Clone, Debug)]
struct srcREGVar1 {
    fregCore: TokenField_fregCore,
}
impl srcREGVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.fregCore.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldf7().disassembly() != 1i128 {
            return None;
        }
        let fregCore = token_parser.TokenFieldfregCore();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fregCore }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:203:1, end:203:7))"]
#[derive(Clone, Debug)]
struct srcREGVar2 {
    fregCore: TokenField_fregCore,
}
impl srcREGVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.fregCore.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldf7().disassembly() != 2i128 {
            return None;
        }
        let fregCore = token_parser.TokenFieldfregCore();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fregCore }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:211:1, end:211:7))"]
#[derive(Clone, Debug)]
struct srcREGVar3 {
    fregCore: TokenField_fregCore,
}
impl srcREGVar3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.fregCore.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldf7().disassembly() != 3i128 {
            return None;
        }
        let fregCore = token_parser.TokenFieldfregCore();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fregCore }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:212:1, end:212:7))"]
#[derive(Clone, Debug)]
struct srcREGVar4 {
    fregCore: TokenField_fregCore,
}
impl srcREGVar4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.fregCore.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldf7().disassembly() != 4i128 {
            return None;
        }
        let fregCore = token_parser.TokenFieldfregCore();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fregCore }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:214:1, end:214:7))"]
#[derive(Clone, Debug)]
struct srcREGVar5 {
    fregCore: TokenField_fregCore,
}
impl srcREGVar5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.fregCore.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldf7().disassembly() != 5i128 {
            return None;
        }
        let fregCore = token_parser.TokenFieldfregCore();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fregCore }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:215:1, end:215:7))"]
#[derive(Clone, Debug)]
struct srcREGVar6 {
    fregCore: TokenField_fregCore,
}
impl srcREGVar6 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.fregCore.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldf7().disassembly() != 6i128 {
            return None;
        }
        let fregCore = token_parser.TokenFieldfregCore();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fregCore }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:216:1, end:216:7))"]
#[derive(Clone, Debug)]
struct srcREGVar7 {
    fregCore: TokenField_fregCore,
}
impl srcREGVar7 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.fregCore.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldf7().disassembly() != 7i128 {
            return None;
        }
        let fregCore = token_parser.TokenFieldfregCore();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fregCore }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:217:1, end:217:7))"]
#[derive(Clone, Debug)]
struct srcREGVar8 {
    fregCore: TokenField_fregCore,
}
impl srcREGVar8 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.fregCore.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldf7().disassembly() != 8i128 {
            return None;
        }
        let fregCore = token_parser.TokenFieldfregCore();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fregCore }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:218:1, end:218:7))"]
#[derive(Clone, Debug)]
struct srcREGVar9 {
    fregCore: TokenField_fregCore,
}
impl srcREGVar9 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.fregCore.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldf7().disassembly() != 9i128 {
            return None;
        }
        let fregCore = token_parser.TokenFieldfregCore();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fregCore }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:220:1, end:220:7))"]
#[derive(Clone, Debug)]
struct srcREGVar10 {
    fregCore: TokenField_fregCore,
}
impl srcREGVar10 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.fregCore.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldf7().disassembly() != 10i128 {
            return None;
        }
        let fregCore = token_parser.TokenFieldfregCore();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fregCore }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:221:1, end:221:7))"]
#[derive(Clone, Debug)]
struct srcREGVar11 {
    fregCore: TokenField_fregCore,
}
impl srcREGVar11 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.fregCore.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldf7().disassembly() != 11i128 {
            return None;
        }
        let fregCore = token_parser.TokenFieldfregCore();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fregCore }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:174:1, end:174:7))"]
#[derive(Clone, Debug)]
struct srcREGVar12 {
    lf7: TokenField_lf7,
}
impl srcREGVar12 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_fv: i128 = 0;
        calc_fv = 112i128.wrapping_add(self.lf7.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_fv)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut calc_fv: i128 = 0;
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFielduf7().disassembly() != 7i128 {
            return None;
        }
        calc_fv =
            112i128.wrapping_add(token_parser.TokenFieldlf7().disassembly());
        let lf7 = token_parser.TokenFieldlf7();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { lf7 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:167:1, end:167:7))"]
#[derive(Clone, Debug)]
struct srcREGVar13 {
    f7: TokenField_f7,
}
impl srcREGVar13 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.f7.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        let f7 = token_parser.TokenFieldf7();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { f7 }))
    }
}
#[derive(Clone, Debug)]
enum TablesrcREG {
    Var0(srcREGVar0),
    Var1(srcREGVar1),
    Var2(srcREGVar2),
    Var3(srcREGVar3),
    Var4(srcREGVar4),
    Var5(srcREGVar5),
    Var6(srcREGVar6),
    Var7(srcREGVar7),
    Var8(srcREGVar8),
    Var9(srcREGVar9),
    Var10(srcREGVar10),
    Var11(srcREGVar11),
    Var12(srcREGVar12),
    Var13(srcREGVar13),
}
impl TablesrcREG {
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
            srcREGVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar4::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar5::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar6::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar7::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar8::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar9::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar10::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar11::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar12::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar13::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:231:1, end:231:8))"]
#[derive(Clone, Debug)]
struct destREGVar0 {}
impl destREGVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldd().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldf7().disassembly() != 2i128 {
            return None;
        }
        let fregCore = token_parser.TokenFieldfregCore();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:225:1, end:225:8))"]
#[derive(Clone, Debug)]
struct destREGVar1 {}
impl destREGVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldd().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:228:1, end:228:8))"]
#[derive(Clone, Debug)]
struct destREGVar2 {
    srcREG: TablesrcREG,
}
impl destREGVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldd().disassembly() != 1i128 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let f7 = token_parser.TokenFieldf7();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { srcREG }))
    }
}
#[derive(Clone, Debug)]
enum TabledestREG {
    Var0(destREGVar0),
    Var1(destREGVar1),
    Var2(destREGVar2),
}
impl TabledestREG {
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
            destREGVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            destREGVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            destREGVar2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:234:1, end:234:2))"]
#[derive(Clone, Debug)]
struct DVar0 {}
impl DVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("w")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldd().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:235:1, end:235:2))"]
#[derive(Clone, Debug)]
struct DVar1 {}
impl DVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("f")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldd().disassembly() != 1i128 {
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
enum TableD {
    Var0(DVar0),
    Var1(DVar1),
}
impl TableD {
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
            DVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            DVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:238:1, end:238:10))"]
#[derive(Clone, Debug)]
struct absAddr11Var0 {
    k11: TokenField_k11,
}
impl absAddr11Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.k11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        let k11 = token_parser.TokenFieldk11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { k11 }))
    }
}
#[derive(Clone, Debug)]
enum TableabsAddr11 {
    Var0(absAddr11Var0),
}
impl TableabsAddr11 {
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
            absAddr11Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:246:1, end:246:9))"]
#[derive(Clone, Debug)]
struct relAddr9Var0 {
    sk9: TokenField_sk9,
}
impl relAddr9Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_addr: i128 = 0;
        calc_addr = i128::try_from(inst_next)
            .unwrap()
            .wrapping_add(self.sk9.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_addr)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut calc_addr: i128 = 0;
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let sk9 = token_parser.TokenFieldsk9();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sk9 }))
    }
}
#[derive(Clone, Debug)]
enum TablerelAddr9 {
    Var0(relAddr9Var0),
}
impl TablerelAddr9 {
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
            relAddr9Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:253:1, end:253:5))"]
#[derive(Clone, Debug)]
struct imm8Var0 {
    k8: TokenField_k8,
}
impl imm8Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("#"), self.k8.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        let k8 = token_parser.TokenFieldk8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { k8 }))
    }
}
#[derive(Clone, Debug)]
enum Tableimm8 {
    Var0(imm8Var0),
}
impl Tableimm8 {
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
            imm8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:257:1, end:257:5))"]
#[derive(Clone, Debug)]
struct imm7Var0 {
    k7: TokenField_k7,
}
impl imm7Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("#"), self.k7.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        let k7 = token_parser.TokenFieldk7();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { k7 }))
    }
}
#[derive(Clone, Debug)]
enum Tableimm7 {
    Var0(imm7Var0),
}
impl Tableimm7 {
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
            imm7Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:260:1, end:260:5))"]
#[derive(Clone, Debug)]
struct imm5Var0 {
    k5: TokenField_k5,
}
impl imm5Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("#"), self.k5.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        let k5 = token_parser.TokenFieldk5();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { k5 }))
    }
}
#[derive(Clone, Debug)]
enum Tableimm5 {
    Var0(imm5Var0),
}
impl Tableimm5 {
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
            imm5Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:264:1, end:264:4))"]
#[derive(Clone, Debug)]
struct bitVar0 {
    b3: TokenField_b3,
}
impl bitVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("#"), self.b3.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        let b3 = token_parser.TokenFieldb3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { b3 }))
    }
}
#[derive(Clone, Debug)]
enum Tablebit {
    Var0(bitVar0),
}
impl Tablebit {
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
            bitVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:272:1, end:272:8))"]
#[derive(Clone, Debug)]
struct trisREGVar0 {}
impl trisREGVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("5")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldl5().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:273:1, end:273:8))"]
#[derive(Clone, Debug)]
struct trisREGVar1 {}
impl trisREGVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("6")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldl5().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:274:1, end:274:8))"]
#[derive(Clone, Debug)]
struct trisREGVar2 {}
impl trisREGVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("7")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldl5().disassembly() != 7i128 {
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
enum TabletrisREG {
    Var0(trisREGVar0),
    Var1(trisREGVar1),
    Var2(trisREGVar2),
}
impl TabletrisREG {
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
            trisREGVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            trisREGVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            trisREGVar2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:743:1, end:743:7))"]
#[derive(Clone, Debug)]
struct srcFSRVar0 {
    fsr: TokenField_fsr,
}
impl srcFSRVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("++"), self.fsr.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldmm().disassembly() != 0i128 {
            return None;
        }
        let fsr = token_parser.TokenFieldfsr();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fsr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:744:1, end:744:7))"]
#[derive(Clone, Debug)]
struct srcFSRVar1 {
    fsr: TokenField_fsr,
}
impl srcFSRVar1 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("--"), self.fsr.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldmm().disassembly() != 1i128 {
            return None;
        }
        let fsr = token_parser.TokenFieldfsr();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fsr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:745:1, end:745:7))"]
#[derive(Clone, Debug)]
struct srcFSRVar2 {
    fsr: TokenField_fsr,
}
impl srcFSRVar2 {
    fn display_extend<T>(
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
            [self.fsr.display(), DisplayElement::Literal("++")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldmm().disassembly() != 2i128 {
            return None;
        }
        let fsr = token_parser.TokenFieldfsr();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fsr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:746:1, end:746:7))"]
#[derive(Clone, Debug)]
struct srcFSRVar3 {
    fsr: TokenField_fsr,
}
impl srcFSRVar3 {
    fn display_extend<T>(
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
            [self.fsr.display(), DisplayElement::Literal("--")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldmm().disassembly() != 3i128 {
            return None;
        }
        let fsr = token_parser.TokenFieldfsr();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fsr }))
    }
}
#[derive(Clone, Debug)]
enum TablesrcFSR {
    Var0(srcFSRVar0),
    Var1(srcFSRVar1),
    Var2(srcFSRVar2),
    Var3(srcFSRVar3),
}
impl TablesrcFSR {
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
            srcFSRVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcFSRVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcFSRVar2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcFSRVar3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic16_instructions.sinc, start:748:1, end:748:8))"]
#[derive(Clone, Debug)]
struct srcFSRkVar0 {
    sk6: TokenField_sk6,
    fsrk: TokenField_fsrk,
}
impl srcFSRkVar0 {
    fn display_extend<T>(
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
            self.sk6.display(),
            DisplayElement::Literal("["),
            self.fsrk.display(),
            DisplayElement::Literal("]"),
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
        let fsrk = token_parser.TokenFieldfsrk();
        let sk6 = token_parser.TokenFieldsk6();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fsrk, sk6 }))
    }
}
#[derive(Clone, Debug)]
enum TablesrcFSRk {
    Var0(srcFSRkVar0),
}
impl TablesrcFSRk {
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
            srcFSRkVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
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
