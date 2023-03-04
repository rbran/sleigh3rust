use sleigh4rust::*;
pub type AddrType = u32;
pub trait GlobalSetTrait {
    fn set_switch_low(&mut self, address: Option<u32>, value: i128);
    fn set_switch_high(&mut self, address: Option<u32>, value: i128);
    fn set_switch_num(&mut self, address: Option<u32>, value: i128);
    fn set_in_table_switch(&mut self, address: Option<u32>, value: i128);
    fn set_in_lookup_switch(&mut self, address: Option<u32>, value: i128);
    fn set_alignmentPad(&mut self, address: Option<u32>, value: i128);
    fn set_padVal(&mut self, address: Option<u32>, value: i128);
}
#[derive(Default)]
pub struct GlobalSetDefault<C: ContextTrait>(
    pub std::collections::HashMap<AddrType, C>,
);
impl<C: ContextTrait> GlobalSetTrait for GlobalSetDefault<C> {
    fn set_switch_low(&mut self, inst_start: Option<AddrType>, value: i128) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_switch_low_disassembly(value)
                .unwrap();
            context
        });
    }
    fn set_switch_high(&mut self, inst_start: Option<AddrType>, value: i128) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_switch_high_disassembly(value)
                .unwrap();
            context
        });
    }
    fn set_switch_num(&mut self, inst_start: Option<AddrType>, value: i128) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_switch_num_disassembly(value)
                .unwrap();
            context
        });
    }
    fn set_in_table_switch(
        &mut self,
        inst_start: Option<AddrType>,
        value: i128,
    ) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_in_table_switch_disassembly(value)
                .unwrap();
            context
        });
    }
    fn set_in_lookup_switch(
        &mut self,
        inst_start: Option<AddrType>,
        value: i128,
    ) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_in_lookup_switch_disassembly(value)
                .unwrap();
            context
        });
    }
    fn set_alignmentPad(&mut self, inst_start: Option<AddrType>, value: i128) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_alignmentPad_disassembly(value)
                .unwrap();
            context
        });
    }
    fn set_padVal(&mut self, inst_start: Option<AddrType>, value: i128) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_padVal_disassembly(value)
                .unwrap();
            context
        });
    }
}
pub trait ContextregisterTrait:
    MemoryRead<AddressType = u32> + MemoryWrite
{
    fn read_switch_low_raw(
        &self,
    ) -> Result<u32, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u32::<true>(12, 0, 32)?;
        Ok(u32::try_from(work_value).unwrap())
    }
    fn write_switch_low_raw(
        &mut self,
        param: u32,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u32::<true>(u32::from(param), 12, 0, 32)
    }
    fn read_switch_low_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_switch_low_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_switch_low_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_switch_low_raw(param as u32)
    }
    fn read_switch_low_execution(
        &self,
    ) -> Result<u32, MemoryReadError<Self::AddressType>> {
        self.read_switch_low_raw()
    }
    fn write_switch_low_execution(
        &mut self,
        param: u32,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_switch_low_raw(param)
    }
    fn switch_low_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_switch_low_raw()?))
    }
    fn read_switch_high_raw(
        &self,
    ) -> Result<u32, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u32::<true>(8, 0, 32)?;
        Ok(u32::try_from(work_value).unwrap())
    }
    fn write_switch_high_raw(
        &mut self,
        param: u32,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u32::<true>(u32::from(param), 8, 0, 32)
    }
    fn read_switch_high_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_switch_high_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_switch_high_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_switch_high_raw(param as u32)
    }
    fn read_switch_high_execution(
        &self,
    ) -> Result<u32, MemoryReadError<Self::AddressType>> {
        self.read_switch_high_raw()
    }
    fn write_switch_high_execution(
        &mut self,
        param: u32,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_switch_high_raw(param)
    }
    fn switch_high_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_switch_high_raw()?))
    }
    fn read_switch_num_raw(
        &self,
    ) -> Result<u32, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u32::<true>(4, 0, 32)?;
        Ok(u32::try_from(work_value).unwrap())
    }
    fn write_switch_num_raw(
        &mut self,
        param: u32,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u32::<true>(u32::from(param), 4, 0, 32)
    }
    fn read_switch_num_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_switch_num_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_switch_num_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_switch_num_raw(param as u32)
    }
    fn read_switch_num_execution(
        &self,
    ) -> Result<u32, MemoryReadError<Self::AddressType>> {
        self.read_switch_num_raw()
    }
    fn write_switch_num_execution(
        &mut self,
        param: u32,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_switch_num_raw(param)
    }
    fn switch_num_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_switch_num_raw()?))
    }
    fn read_in_table_switch_raw(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u8::<true>(3, 0, 2)?;
        Ok(u8::try_from(work_value).unwrap())
    }
    fn write_in_table_switch_raw(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u8::<true>(u8::from(param), 3, 0, 2)
    }
    fn read_in_table_switch_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_in_table_switch_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_in_table_switch_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_in_table_switch_raw(param as u8)
    }
    fn read_in_table_switch_execution(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        self.read_in_table_switch_raw()
    }
    fn write_in_table_switch_execution(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_in_table_switch_raw(param)
    }
    fn in_table_switch_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_in_table_switch_raw()?))
    }
    fn read_in_lookup_switch_raw(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u8::<true>(3, 2, 2)?;
        Ok(u8::try_from(work_value).unwrap())
    }
    fn write_in_lookup_switch_raw(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u8::<true>(u8::from(param), 3, 2, 2)
    }
    fn read_in_lookup_switch_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_in_lookup_switch_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_in_lookup_switch_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_in_lookup_switch_raw(param as u8)
    }
    fn read_in_lookup_switch_execution(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        self.read_in_lookup_switch_raw()
    }
    fn write_in_lookup_switch_execution(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_in_lookup_switch_raw(param)
    }
    fn in_lookup_switch_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_in_lookup_switch_raw()?))
    }
    fn read_alignmentPad_raw(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u8::<true>(3, 4, 2)?;
        Ok(u8::try_from(work_value).unwrap())
    }
    fn write_alignmentPad_raw(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u8::<true>(u8::from(param), 3, 4, 2)
    }
    fn read_alignmentPad_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_alignmentPad_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_alignmentPad_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_alignmentPad_raw(param as u8)
    }
    fn read_alignmentPad_execution(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        self.read_alignmentPad_raw()
    }
    fn write_alignmentPad_execution(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_alignmentPad_raw(param)
    }
    fn alignmentPad_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_alignmentPad_raw()?))
    }
    fn read_padVal_raw(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u8::<true>(3, 4, 2)?;
        Ok(u8::try_from(work_value).unwrap())
    }
    fn write_padVal_raw(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u8::<true>(u8::from(param), 3, 4, 2)
    }
    fn read_padVal_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_padVal_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_padVal_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_padVal_raw(param as u8)
    }
    fn read_padVal_execution(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        self.read_padVal_raw()
    }
    fn write_padVal_execution(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_padVal_raw(param)
    }
    fn padVal_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_padVal_raw()?))
    }
}
pub trait ContextTrait: Default {
    type Typeregister: ContextregisterTrait;
    fn register(&self) -> &Self::Typeregister;
    fn register_mut(&mut self) -> &mut Self::Typeregister;
}
#[derive(Debug, Clone, Copy)]
pub struct ContextregisterStructDebug {
    pub chunk_0x0: [Option<bool>; 128],
}
impl Default for ContextregisterStructDebug {
    fn default() -> Self {
        Self {
            chunk_0x0: [None; 128],
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
            (0..=15, 0..=16) => {
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
            (0..=15, 0..=16) => {
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
    type AddressType = u32;
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
#[derive(Clone, Copy, Debug)]
struct TokenField_op(u8);
impl TokenField_op {
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
struct TokenField_w_op(u16);
impl TokenField_w_op {
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
struct TokenField_n(u8);
impl TokenField_n {
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
struct TokenField_m(u8);
impl TokenField_m {
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
struct TokenField_atype(u8);
impl TokenField_atype {
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
struct TokenField_byte(u8);
impl TokenField_byte {
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
struct TokenField_byte1(u8);
impl TokenField_byte1 {
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
struct TokenField_byte2(u8);
impl TokenField_byte2 {
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
struct TokenField_byte3(u8);
impl TokenField_byte3 {
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
struct TokenField_byte4(u8);
impl TokenField_byte4 {
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
struct TokenField_sbyte(i8);
impl TokenField_sbyte {
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
struct TokenField_branch(i8);
impl TokenField_branch {
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
struct TokenField_branchbyte1(i8);
impl TokenField_branchbyte1 {
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
struct TokenField_branchbyte2(u8);
impl TokenField_branchbyte2 {
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
struct TokenField_branchbyte3(u8);
impl TokenField_branchbyte3 {
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
struct TokenField_branchbyte4(u8);
impl TokenField_branchbyte4 {
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
struct TokenField_index(u8);
impl TokenField_index {
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
struct TokenField_indexbyte1(u8);
impl TokenField_indexbyte1 {
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
struct TokenField_indexbyte2(u8);
impl TokenField_indexbyte2 {
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
struct TokenField_constant(u8);
impl TokenField_constant {
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
struct TokenField_constantbyte1(u8);
impl TokenField_constantbyte1 {
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
struct TokenField_constantbyte2(u8);
impl TokenField_constantbyte2 {
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
struct TokenField_nargs(u8);
impl TokenField_nargs {
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
struct TokenField_method(u8);
impl TokenField_method {
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
struct TokenField_defaultbyte1(u8);
impl TokenField_defaultbyte1 {
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
struct TokenField_defaultbyte2(u8);
impl TokenField_defaultbyte2 {
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
struct TokenField_defaultbyte3(u8);
impl TokenField_defaultbyte3 {
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
struct TokenField_defaultbyte4(u8);
impl TokenField_defaultbyte4 {
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
struct TokenField_highbyte1(u8);
impl TokenField_highbyte1 {
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
struct TokenField_highbyte2(u8);
impl TokenField_highbyte2 {
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
struct TokenField_highbyte3(u8);
impl TokenField_highbyte3 {
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
struct TokenField_highbyte4(u8);
impl TokenField_highbyte4 {
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
struct TokenField_lowbyte1(u8);
impl TokenField_lowbyte1 {
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
struct TokenField_lowbyte2(u8);
impl TokenField_lowbyte2 {
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
struct TokenField_lowbyte3(u8);
impl TokenField_lowbyte3 {
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
struct TokenField_lowbyte4(u8);
impl TokenField_lowbyte4 {
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
struct TokenField_npairsbyte1(u8);
impl TokenField_npairsbyte1 {
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
struct TokenField_npairsbyte2(u8);
impl TokenField_npairsbyte2 {
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
struct TokenField_npairsbyte3(u8);
impl TokenField_npairsbyte3 {
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
struct TokenField_npairsbyte4(u8);
impl TokenField_npairsbyte4 {
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
struct TokenField_dimensions(u8);
impl TokenField_dimensions {
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
struct TokenField_blank1(u8);
impl TokenField_blank1 {
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
struct TokenField_blank2(u8);
impl TokenField_blank2 {
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
struct TokenField_count(u8);
impl TokenField_count {
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
struct TokenField_pad(u8);
impl TokenField_pad {
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
struct TokenField_pad1(u8);
impl TokenField_pad1 {
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
struct TokenField_pad2(u8);
impl TokenField_pad2 {
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
struct TokenField_pad3(u8);
impl TokenField_pad3 {
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
struct TokenField_wide_op(u8);
impl TokenField_wide_op {
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
struct TokenField_matchbyte1(u8);
impl TokenField_matchbyte1 {
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
struct TokenField_matchbyte2(u8);
impl TokenField_matchbyte2 {
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
struct TokenField_matchbyte3(u8);
impl TokenField_matchbyte3 {
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
struct TokenField_matchbyte4(u8);
impl TokenField_matchbyte4 {
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
struct TokenField_offsetbyte1(u8);
impl TokenField_offsetbyte1 {
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
struct TokenField_offsetbyte2(u8);
impl TokenField_offsetbyte2 {
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
struct TokenField_offsetbyte3(u8);
impl TokenField_offsetbyte3 {
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
struct TokenField_offsetbyte4(u8);
impl TokenField_offsetbyte4 {
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
    fn TokenFieldop(&self) -> TokenField_op {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_op(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldw_op(&self) -> TokenField_w_op {
        let inner_value = self.read_u16::<true>(0, 0, 16).unwrap();
        TokenField_w_op(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldn(&self) -> TokenField_n {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_n(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldm(&self) -> TokenField_m {
        let inner_value = self.read_u8::<true>(0, 4, 4).unwrap();
        TokenField_m(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldatype(&self) -> TokenField_atype {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_atype(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbyte(&self) -> TokenField_byte {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_byte(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbyte1(&self) -> TokenField_byte1 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_byte1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbyte2(&self) -> TokenField_byte2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_byte2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbyte3(&self) -> TokenField_byte3 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_byte3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbyte4(&self) -> TokenField_byte4 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_byte4(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsbyte(&self) -> TokenField_sbyte {
        let inner_value = self.read_i8::<true>(0, 0, 8).unwrap();
        TokenField_sbyte(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbranch(&self) -> TokenField_branch {
        let inner_value = self.read_i8::<true>(0, 0, 8).unwrap();
        TokenField_branch(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbranchbyte1(&self) -> TokenField_branchbyte1 {
        let inner_value = self.read_i8::<true>(0, 0, 8).unwrap();
        TokenField_branchbyte1(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbranchbyte2(&self) -> TokenField_branchbyte2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_branchbyte2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbranchbyte3(&self) -> TokenField_branchbyte3 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_branchbyte3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbranchbyte4(&self) -> TokenField_branchbyte4 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_branchbyte4(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldindex(&self) -> TokenField_index {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_index(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldindexbyte1(&self) -> TokenField_indexbyte1 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_indexbyte1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldindexbyte2(&self) -> TokenField_indexbyte2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_indexbyte2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldconstant(&self) -> TokenField_constant {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_constant(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldconstantbyte1(&self) -> TokenField_constantbyte1 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_constantbyte1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldconstantbyte2(&self) -> TokenField_constantbyte2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_constantbyte2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldnargs(&self) -> TokenField_nargs {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_nargs(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldmethod(&self) -> TokenField_method {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_method(u8::try_from(inner_value).unwrap())
    }
    fn TokenFielddefaultbyte1(&self) -> TokenField_defaultbyte1 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_defaultbyte1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFielddefaultbyte2(&self) -> TokenField_defaultbyte2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_defaultbyte2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFielddefaultbyte3(&self) -> TokenField_defaultbyte3 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_defaultbyte3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFielddefaultbyte4(&self) -> TokenField_defaultbyte4 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_defaultbyte4(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldhighbyte1(&self) -> TokenField_highbyte1 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_highbyte1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldhighbyte2(&self) -> TokenField_highbyte2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_highbyte2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldhighbyte3(&self) -> TokenField_highbyte3 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_highbyte3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldhighbyte4(&self) -> TokenField_highbyte4 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_highbyte4(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlowbyte1(&self) -> TokenField_lowbyte1 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_lowbyte1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlowbyte2(&self) -> TokenField_lowbyte2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_lowbyte2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlowbyte3(&self) -> TokenField_lowbyte3 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_lowbyte3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlowbyte4(&self) -> TokenField_lowbyte4 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_lowbyte4(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldnpairsbyte1(&self) -> TokenField_npairsbyte1 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_npairsbyte1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldnpairsbyte2(&self) -> TokenField_npairsbyte2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_npairsbyte2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldnpairsbyte3(&self) -> TokenField_npairsbyte3 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_npairsbyte3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldnpairsbyte4(&self) -> TokenField_npairsbyte4 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_npairsbyte4(u8::try_from(inner_value).unwrap())
    }
    fn TokenFielddimensions(&self) -> TokenField_dimensions {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_dimensions(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldblank1(&self) -> TokenField_blank1 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_blank1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldblank2(&self) -> TokenField_blank2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_blank2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldcount(&self) -> TokenField_count {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_count(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldpad(&self) -> TokenField_pad {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_pad(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldpad1(&self) -> TokenField_pad1 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_pad1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldpad2(&self) -> TokenField_pad2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_pad2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldpad3(&self) -> TokenField_pad3 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_pad3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldwide_op(&self) -> TokenField_wide_op {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_wide_op(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldmatchbyte1(&self) -> TokenField_matchbyte1 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_matchbyte1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldmatchbyte2(&self) -> TokenField_matchbyte2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_matchbyte2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldmatchbyte3(&self) -> TokenField_matchbyte3 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_matchbyte3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldmatchbyte4(&self) -> TokenField_matchbyte4 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_matchbyte4(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldoffsetbyte1(&self) -> TokenField_offsetbyte1 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_offsetbyte1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldoffsetbyte2(&self) -> TokenField_offsetbyte2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_offsetbyte2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldoffsetbyte3(&self) -> TokenField_offsetbyte3 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_offsetbyte3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldoffsetbyte4(&self) -> TokenField_offsetbyte4 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_offsetbyte4(u8::try_from(inner_value).unwrap())
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2066:1, end:2066:2))"]
#[derive(Clone, Debug)]
struct wide_iload_instructionVar0 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl wide_iload_instructionVar0 {
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
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50197i128 {
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
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldindexbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2073:1, end:2073:2))"]
#[derive(Clone, Debug)]
struct wide_fload_instructionVar1 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl wide_fload_instructionVar1 {
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
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50199i128 {
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
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldindexbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2080:1, end:2080:2))"]
#[derive(Clone, Debug)]
struct wide_aload_instructionVar2 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl wide_aload_instructionVar2 {
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
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50201i128 {
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
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldindexbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2087:1, end:2087:2))"]
#[derive(Clone, Debug)]
struct wide_lload_instructionVar3 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl wide_lload_instructionVar3 {
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
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50198i128 {
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
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldindexbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2094:1, end:2094:2))"]
#[derive(Clone, Debug)]
struct wide_dload_instructionVar4 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl wide_dload_instructionVar4 {
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
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50200i128 {
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
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldindexbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2103:1, end:2103:2))"]
#[derive(Clone, Debug)]
struct wide_istore_instructionVar5 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl wide_istore_instructionVar5 {
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
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50230i128 {
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
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldindexbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2111:1, end:2111:2))"]
#[derive(Clone, Debug)]
struct wide_fstore_instructionVar6 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl wide_fstore_instructionVar6 {
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
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50232i128 {
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
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldindexbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2119:1, end:2119:2))"]
#[derive(Clone, Debug)]
struct wide_astore_instructionVar7 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl wide_astore_instructionVar7 {
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
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50234i128 {
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
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldindexbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2127:1, end:2127:2))"]
#[derive(Clone, Debug)]
struct wide_lstore_instructionVar8 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl wide_lstore_instructionVar8 {
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
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50231i128 {
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
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldindexbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2135:1, end:2135:2))"]
#[derive(Clone, Debug)]
struct wide_dstore_instructionVar9 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl wide_dstore_instructionVar9 {
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
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50233i128 {
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
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldindexbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2144:1, end:2144:2))"]
#[derive(Clone, Debug)]
struct wide_ret_instructionVar10 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl wide_ret_instructionVar10 {
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
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50345i128 {
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
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldindexbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2152:1, end:2152:2))"]
#[derive(Clone, Debug)]
struct iinc_w_instructionVar11 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
    constantbyte1: TokenField_constantbyte1,
    constantbyte2: TokenField_constantbyte2,
}
impl iinc_w_instructionVar11 {
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
        let mut calc_index: i128 = 0;
        let mut calc_constant: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.indexbyte2.disassembly());
        calc_constant = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.constantbyte1.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.constantbyte2.disassembly());
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("iinc_w"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        let mut calc_index: i128 = 0;
        let mut calc_constant: i128 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c28 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldw_op().disassembly() != 50308i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c28(tokens_current, &mut context_instance)?;
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
        calc_index = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldindexbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
        calc_constant = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldconstantbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:222:1, end:222:2))"]
#[derive(Clone, Debug)]
struct aaload_instructionVar12 {}
impl aaload_instructionVar12 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 50i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:233:1, end:233:2))"]
#[derive(Clone, Debug)]
struct aastore_instructionVar13 {}
impl aastore_instructionVar13 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 83i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:246:1, end:246:2))"]
#[derive(Clone, Debug)]
struct aconst_null_instructionVar14 {}
impl aconst_null_instructionVar14 {
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 1i128 {
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
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:251:1, end:251:2))"]
#[derive(Clone, Debug)]
struct aload_instructionVar15 {
    index: TokenField_index,
}
impl aload_instructionVar15 {
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 25i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:258:1, end:258:2))"]
#[derive(Clone, Debug)]
struct aload_0_instructionVar16 {}
impl aload_0_instructionVar16 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 42i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:265:1, end:265:2))"]
#[derive(Clone, Debug)]
struct aload_1_instructionVar17 {}
impl aload_1_instructionVar17 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 43i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:272:1, end:272:2))"]
#[derive(Clone, Debug)]
struct aload_2_instructionVar18 {}
impl aload_2_instructionVar18 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 44i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:279:1, end:279:2))"]
#[derive(Clone, Debug)]
struct aload_3_instructionVar19 {}
impl aload_3_instructionVar19 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 45i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:286:1, end:286:2))"]
#[derive(Clone, Debug)]
struct anewarray_instructionVar20 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl anewarray_instructionVar20 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 189i128 {
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
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:296:1, end:296:2))"]
#[derive(Clone, Debug)]
struct areturn_instructionVar21 {}
impl areturn_instructionVar21 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 176i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:302:1, end:302:2))"]
#[derive(Clone, Debug)]
struct arraylength_instructionVar22 {}
impl arraylength_instructionVar22 {
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 190i128 {
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
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:311:1, end:311:2))"]
#[derive(Clone, Debug)]
struct astore_instructionVar23 {
    index: TokenField_index,
}
impl astore_instructionVar23 {
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 58i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:319:1, end:319:2))"]
#[derive(Clone, Debug)]
struct astore_0_instructionVar24 {}
impl astore_0_instructionVar24 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 75i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:327:1, end:327:2))"]
#[derive(Clone, Debug)]
struct astore_1_instructionVar25 {}
impl astore_1_instructionVar25 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 76i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:335:1, end:335:2))"]
#[derive(Clone, Debug)]
struct astore_2_instructionVar26 {}
impl astore_2_instructionVar26 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 77i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:344:1, end:344:2))"]
#[derive(Clone, Debug)]
struct astore_3_instructionVar27 {}
impl astore_3_instructionVar27 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 78i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:352:1, end:352:2))"]
#[derive(Clone, Debug)]
struct athrow_instructionVar28 {}
impl athrow_instructionVar28 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 191i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:362:1, end:362:2))"]
#[derive(Clone, Debug)]
struct baload_instructionVar29 {}
impl baload_instructionVar29 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 51i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:375:1, end:375:2))"]
#[derive(Clone, Debug)]
struct bastore_instructionVar30 {}
impl bastore_instructionVar30 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 84i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:387:1, end:387:2))"]
#[derive(Clone, Debug)]
struct bipush_instructionVar31 {
    sbyte: TokenField_sbyte,
}
impl bipush_instructionVar31 {
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 16i128 {
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
        let sbyte = token_parser.TokenFieldsbyte();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sbyte }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:393:1, end:393:2))"]
#[derive(Clone, Debug)]
struct caload_instructionVar32 {}
impl caload_instructionVar32 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 52i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:406:1, end:406:2))"]
#[derive(Clone, Debug)]
struct castore_instructionVar33 {}
impl castore_instructionVar33 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 85i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:418:1, end:418:2))"]
#[derive(Clone, Debug)]
struct checkcast_instructionVar34 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl checkcast_instructionVar34 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 192i128 {
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
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:440:1, end:440:2))"]
#[derive(Clone, Debug)]
struct d2f_instructionVar35 {}
impl d2f_instructionVar35 {
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
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 144i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:451:1, end:451:2))"]
#[derive(Clone, Debug)]
struct d2i_instructionVar36 {}
impl d2i_instructionVar36 {
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
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 142i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:461:1, end:461:2))"]
#[derive(Clone, Debug)]
struct d2l_instructionVar37 {}
impl d2l_instructionVar37 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 143i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:470:1, end:470:2))"]
#[derive(Clone, Debug)]
struct dadd_instructionVar38 {}
impl dadd_instructionVar38 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 99i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:481:1, end:481:2))"]
#[derive(Clone, Debug)]
struct daload_instructionVar39 {}
impl daload_instructionVar39 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 49i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:493:1, end:493:2))"]
#[derive(Clone, Debug)]
struct dastore_instructionVar40 {}
impl dastore_instructionVar40 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 82i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:505:1, end:505:2))"]
#[derive(Clone, Debug)]
struct dcmpg_instructionVar41 {}
impl dcmpg_instructionVar41 {
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
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 152i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:516:1, end:516:2))"]
#[derive(Clone, Debug)]
struct dcmpl_instructionVar42 {}
impl dcmpl_instructionVar42 {
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
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 151i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:527:1, end:527:2))"]
#[derive(Clone, Debug)]
struct dconst_0_instructionVar43 {}
impl dconst_0_instructionVar43 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 14i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:534:1, end:534:2))"]
#[derive(Clone, Debug)]
struct dconst_1_instructionVar44 {}
impl dconst_1_instructionVar44 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 15i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:541:1, end:541:2))"]
#[derive(Clone, Debug)]
struct ddiv_instructionVar45 {}
impl ddiv_instructionVar45 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 111i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:552:1, end:552:2))"]
#[derive(Clone, Debug)]
struct dload_instructionVar46 {
    index: TokenField_index,
}
impl dload_instructionVar46 {
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 24i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:559:1, end:559:2))"]
#[derive(Clone, Debug)]
struct dload_0_instructionVar47 {}
impl dload_0_instructionVar47 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 38i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:566:1, end:566:2))"]
#[derive(Clone, Debug)]
struct dload_1_instructionVar48 {}
impl dload_1_instructionVar48 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 39i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:573:1, end:573:2))"]
#[derive(Clone, Debug)]
struct dload_2_instructionVar49 {}
impl dload_2_instructionVar49 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 40i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:580:1, end:580:2))"]
#[derive(Clone, Debug)]
struct dload_3_instructionVar50 {}
impl dload_3_instructionVar50 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 41i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:587:1, end:587:2))"]
#[derive(Clone, Debug)]
struct dmul_instructionVar51 {}
impl dmul_instructionVar51 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 107i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:598:1, end:598:2))"]
#[derive(Clone, Debug)]
struct dneg_instructionVar52 {}
impl dneg_instructionVar52 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 119i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:607:1, end:607:2))"]
#[derive(Clone, Debug)]
struct drem_instructionVar53 {}
impl drem_instructionVar53 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 115i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:618:1, end:618:2))"]
#[derive(Clone, Debug)]
struct dreturn_instructionVar54 {}
impl dreturn_instructionVar54 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 175i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:624:1, end:624:2))"]
#[derive(Clone, Debug)]
struct dstore_instructionVar55 {
    index: TokenField_index,
}
impl dstore_instructionVar55 {
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 57i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:632:1, end:632:2))"]
#[derive(Clone, Debug)]
struct dstore_0_instructionVar56 {}
impl dstore_0_instructionVar56 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 71i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:640:1, end:640:2))"]
#[derive(Clone, Debug)]
struct dstore_1_instructionVar57 {}
impl dstore_1_instructionVar57 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 72i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:648:1, end:648:2))"]
#[derive(Clone, Debug)]
struct dstore_2_instructionVar58 {}
impl dstore_2_instructionVar58 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 73i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:656:1, end:656:2))"]
#[derive(Clone, Debug)]
struct dstore_3_instructionVar59 {}
impl dstore_3_instructionVar59 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 74i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:664:1, end:664:2))"]
#[derive(Clone, Debug)]
struct dsub_instructionVar60 {}
impl dsub_instructionVar60 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 103i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:675:1, end:675:2))"]
#[derive(Clone, Debug)]
struct dup_instructionVar61 {}
impl dup_instructionVar61 {
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
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 89i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:681:1, end:681:2))"]
#[derive(Clone, Debug)]
struct dup_x1_instructionVar62 {}
impl dup_x1_instructionVar62 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 90i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:692:1, end:692:2))"]
#[derive(Clone, Debug)]
struct dup_x2_instructionVar63 {}
impl dup_x2_instructionVar63 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 91i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:706:1, end:706:2))"]
#[derive(Clone, Debug)]
struct dup2_instructionVar64 {}
impl dup2_instructionVar64 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 92i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:718:1, end:718:2))"]
#[derive(Clone, Debug)]
struct dup2_x1_instructionVar65 {}
impl dup2_x1_instructionVar65 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 93i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:733:1, end:733:2))"]
#[derive(Clone, Debug)]
struct dup2_x2_instructionVar66 {}
impl dup2_x2_instructionVar66 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 94i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:751:1, end:751:2))"]
#[derive(Clone, Debug)]
struct f2d_instructionVar67 {}
impl f2d_instructionVar67 {
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
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 141i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:761:1, end:761:2))"]
#[derive(Clone, Debug)]
struct f2i_instructionVar68 {}
impl f2i_instructionVar68 {
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
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 139i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:771:1, end:771:2))"]
#[derive(Clone, Debug)]
struct f2l_instructionVar69 {}
impl f2l_instructionVar69 {
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
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 140i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:782:1, end:782:2))"]
#[derive(Clone, Debug)]
struct fadd_instructionVar70 {}
impl fadd_instructionVar70 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 98i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:793:1, end:793:2))"]
#[derive(Clone, Debug)]
struct faload_instructionVar71 {}
impl faload_instructionVar71 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 48i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:805:1, end:805:2))"]
#[derive(Clone, Debug)]
struct fastore_instructionVar72 {}
impl fastore_instructionVar72 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 81i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:817:1, end:817:2))"]
#[derive(Clone, Debug)]
struct fcmpg_instructionVar73 {}
impl fcmpg_instructionVar73 {
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
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 150i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:828:1, end:828:2))"]
#[derive(Clone, Debug)]
struct fcmpl_instructionVar74 {}
impl fcmpl_instructionVar74 {
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
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 149i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:839:1, end:839:2))"]
#[derive(Clone, Debug)]
struct fconst_0_instructionVar75 {}
impl fconst_0_instructionVar75 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 11i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:846:1, end:846:2))"]
#[derive(Clone, Debug)]
struct fconst_1_instructionVar76 {}
impl fconst_1_instructionVar76 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 12i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:853:1, end:853:2))"]
#[derive(Clone, Debug)]
struct fconst_2_instructionVar77 {}
impl fconst_2_instructionVar77 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 13i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:860:1, end:860:2))"]
#[derive(Clone, Debug)]
struct fdiv_instructionVar78 {}
impl fdiv_instructionVar78 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 110i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:871:1, end:871:2))"]
#[derive(Clone, Debug)]
struct fload_instructionVar79 {
    index: TokenField_index,
}
impl fload_instructionVar79 {
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 23i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:878:1, end:878:2))"]
#[derive(Clone, Debug)]
struct fload_0_instructionVar80 {}
impl fload_0_instructionVar80 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 34i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:885:1, end:885:2))"]
#[derive(Clone, Debug)]
struct fload_1_instructionVar81 {}
impl fload_1_instructionVar81 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 35i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:892:1, end:892:2))"]
#[derive(Clone, Debug)]
struct fload_2_instructionVar82 {}
impl fload_2_instructionVar82 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 36i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:899:1, end:899:2))"]
#[derive(Clone, Debug)]
struct fload_3_instructionVar83 {}
impl fload_3_instructionVar83 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 37i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:906:1, end:906:2))"]
#[derive(Clone, Debug)]
struct fmul_instructionVar84 {}
impl fmul_instructionVar84 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 106i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:917:1, end:917:2))"]
#[derive(Clone, Debug)]
struct fneg_instructionVar85 {}
impl fneg_instructionVar85 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 118i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:926:1, end:926:2))"]
#[derive(Clone, Debug)]
struct frem_instructionVar86 {}
impl frem_instructionVar86 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 114i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:937:1, end:937:2))"]
#[derive(Clone, Debug)]
struct freturn_instructionVar87 {}
impl freturn_instructionVar87 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 174i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:943:1, end:943:2))"]
#[derive(Clone, Debug)]
struct fstore_instructionVar88 {
    index: TokenField_index,
}
impl fstore_instructionVar88 {
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 56i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:951:1, end:951:2))"]
#[derive(Clone, Debug)]
struct fstore_0_instructionVar89 {}
impl fstore_0_instructionVar89 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 67i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:959:1, end:959:2))"]
#[derive(Clone, Debug)]
struct fstore_1_instructionVar90 {}
impl fstore_1_instructionVar90 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 68i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:966:1, end:966:2))"]
#[derive(Clone, Debug)]
struct fstore_2_instructionVar91 {}
impl fstore_2_instructionVar91 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 69i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:973:1, end:973:2))"]
#[derive(Clone, Debug)]
struct fstore_3_instructionVar92 {}
impl fstore_3_instructionVar92 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 70i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:981:1, end:981:2))"]
#[derive(Clone, Debug)]
struct fsub_instructionVar93 {}
impl fsub_instructionVar93 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 102i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:992:1, end:992:2))"]
#[derive(Clone, Debug)]
struct getfield_instructionVar94 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl getfield_instructionVar94 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("getfield"),
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 180i128 {
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
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:998:1, end:998:2))"]
#[derive(Clone, Debug)]
struct getstatic_instructionVar95 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl getstatic_instructionVar95 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("getstatic"),
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 178i128 {
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
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1003:1, end:1003:2))"]
#[derive(Clone, Debug)]
struct goto_instructionVar96 {
    Branch: TableBranch,
}
impl goto_instructionVar96 {
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 167i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1008:1, end:1008:2))"]
#[derive(Clone, Debug)]
struct goto_w_instructionVar97 {
    Branch_w: TableBranch_w,
}
impl goto_w_instructionVar97 {
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
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 200i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1013:1, end:1013:2))"]
#[derive(Clone, Debug)]
struct i2b_instructionVar98 {}
impl i2b_instructionVar98 {
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
        let mut sub_pattern_c9 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 145i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c9(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1023:1, end:1023:2))"]
#[derive(Clone, Debug)]
struct i2c_instructionVar99 {}
impl i2c_instructionVar99 {
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
        let mut sub_pattern_c9 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 146i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c9(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1033:1, end:1033:2))"]
#[derive(Clone, Debug)]
struct i2d_instructionVar100 {}
impl i2d_instructionVar100 {
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
        let mut sub_pattern_c9 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 135i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c9(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1044:1, end:1044:2))"]
#[derive(Clone, Debug)]
struct i2f_instructionVar101 {}
impl i2f_instructionVar101 {
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
        let mut sub_pattern_c9 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 134i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c9(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1053:1, end:1053:2))"]
#[derive(Clone, Debug)]
struct i2l_instructionVar102 {}
impl i2l_instructionVar102 {
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
        let mut sub_pattern_c9 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 133i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c9(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1062:1, end:1062:2))"]
#[derive(Clone, Debug)]
struct i2s_instructionVar103 {}
impl i2s_instructionVar103 {
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
        let mut sub_pattern_c9 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 147i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c9(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1072:1, end:1072:2))"]
#[derive(Clone, Debug)]
struct iadd_instructionVar104 {}
impl iadd_instructionVar104 {
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
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 96i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1083:1, end:1083:2))"]
#[derive(Clone, Debug)]
struct iaload_instructionVar105 {}
impl iaload_instructionVar105 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 46i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1095:1, end:1095:2))"]
#[derive(Clone, Debug)]
struct iand_instructionVar106 {}
impl iand_instructionVar106 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 126i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1106:1, end:1106:2))"]
#[derive(Clone, Debug)]
struct iastore_instructionVar107 {}
impl iastore_instructionVar107 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 79i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1118:1, end:1118:2))"]
#[derive(Clone, Debug)]
struct iconst_m1_instructionVar108 {}
impl iconst_m1_instructionVar108 {
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
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 2i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1124:1, end:1124:2))"]
#[derive(Clone, Debug)]
struct iconst_0_instructionVar109 {}
impl iconst_0_instructionVar109 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 3i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1130:1, end:1130:2))"]
#[derive(Clone, Debug)]
struct iconst_1_instructionVar110 {}
impl iconst_1_instructionVar110 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 4i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1136:1, end:1136:2))"]
#[derive(Clone, Debug)]
struct iconst_2_instructionVar111 {}
impl iconst_2_instructionVar111 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 5i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1142:1, end:1142:2))"]
#[derive(Clone, Debug)]
struct iconst_3_instructionVar112 {}
impl iconst_3_instructionVar112 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 6i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1148:1, end:1148:2))"]
#[derive(Clone, Debug)]
struct iconst_4_instructionVar113 {}
impl iconst_4_instructionVar113 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 7i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1154:1, end:1154:2))"]
#[derive(Clone, Debug)]
struct iconst_5_instructionVar114 {}
impl iconst_5_instructionVar114 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 8i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1160:1, end:1160:2))"]
#[derive(Clone, Debug)]
struct idiv_instructionVar115 {}
impl idiv_instructionVar115 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 108i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1171:1, end:1171:2))"]
#[derive(Clone, Debug)]
struct if_acmpeq_instructionVar116 {
    Branch: TableBranch,
}
impl if_acmpeq_instructionVar116 {
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 165i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1180:1, end:1180:2))"]
#[derive(Clone, Debug)]
struct if_acmpne_instructionVar117 {
    Branch: TableBranch,
}
impl if_acmpne_instructionVar117 {
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 166i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1189:1, end:1189:2))"]
#[derive(Clone, Debug)]
struct if_icmpeq_instructionVar118 {
    Branch: TableBranch,
}
impl if_icmpeq_instructionVar118 {
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 159i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1198:1, end:1198:2))"]
#[derive(Clone, Debug)]
struct if_icmpne_instructionVar119 {
    Branch: TableBranch,
}
impl if_icmpne_instructionVar119 {
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 160i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1207:1, end:1207:2))"]
#[derive(Clone, Debug)]
struct if_icmplt_instructionVar120 {
    Branch: TableBranch,
}
impl if_icmplt_instructionVar120 {
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 161i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1216:1, end:1216:2))"]
#[derive(Clone, Debug)]
struct if_icmpge_instructionVar121 {
    Branch: TableBranch,
}
impl if_icmpge_instructionVar121 {
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 162i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1225:1, end:1225:2))"]
#[derive(Clone, Debug)]
struct if_icmpgt_instructionVar122 {
    Branch: TableBranch,
}
impl if_icmpgt_instructionVar122 {
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 163i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1234:1, end:1234:2))"]
#[derive(Clone, Debug)]
struct if_icmple_instructionVar123 {
    Branch: TableBranch,
}
impl if_icmple_instructionVar123 {
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 164i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1243:1, end:1243:2))"]
#[derive(Clone, Debug)]
struct ifeq_instructionVar124 {
    Branch: TableBranch,
}
impl ifeq_instructionVar124 {
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 153i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1250:1, end:1250:2))"]
#[derive(Clone, Debug)]
struct ifne_instructionVar125 {
    Branch: TableBranch,
}
impl ifne_instructionVar125 {
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 154i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1257:1, end:1257:2))"]
#[derive(Clone, Debug)]
struct iflt_instructionVar126 {
    Branch: TableBranch,
}
impl iflt_instructionVar126 {
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 155i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1264:1, end:1264:2))"]
#[derive(Clone, Debug)]
struct ifge_instructionVar127 {
    Branch: TableBranch,
}
impl ifge_instructionVar127 {
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 156i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1271:1, end:1271:2))"]
#[derive(Clone, Debug)]
struct ifgt_instructionVar128 {
    Branch: TableBranch,
}
impl ifgt_instructionVar128 {
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 157i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1278:1, end:1278:2))"]
#[derive(Clone, Debug)]
struct ifle_instructionVar129 {
    Branch: TableBranch,
}
impl ifle_instructionVar129 {
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 158i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1285:1, end:1285:2))"]
#[derive(Clone, Debug)]
struct ifnonnull_instructionVar130 {
    Branch: TableBranch,
}
impl ifnonnull_instructionVar130 {
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 199i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1292:1, end:1292:2))"]
#[derive(Clone, Debug)]
struct ifnull_instructionVar131 {
    Branch: TableBranch,
}
impl ifnull_instructionVar131 {
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
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 198i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1299:1, end:1299:2))"]
#[derive(Clone, Debug)]
struct iinc_instructionVar132 {
    index: TokenField_index,
    constant: TokenField_constant,
}
impl iinc_instructionVar132 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("iinc"),
            DisplayElement::Literal(" "),
            self.index.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        let mut sub_pattern_c26 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 132i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1307:1, end:1307:2))"]
#[derive(Clone, Debug)]
struct iload_instructionVar133 {
    index: TokenField_index,
}
impl iload_instructionVar133 {
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 21i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1314:1, end:1314:2))"]
#[derive(Clone, Debug)]
struct iload_0_instructionVar134 {}
impl iload_0_instructionVar134 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 26i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1321:1, end:1321:2))"]
#[derive(Clone, Debug)]
struct iload_1_instructionVar135 {}
impl iload_1_instructionVar135 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 27i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1328:1, end:1328:2))"]
#[derive(Clone, Debug)]
struct iload_2_instructionVar136 {}
impl iload_2_instructionVar136 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 28i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1334:1, end:1334:2))"]
#[derive(Clone, Debug)]
struct iload_3_instructionVar137 {}
impl iload_3_instructionVar137 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 29i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1341:1, end:1341:2))"]
#[derive(Clone, Debug)]
struct imul_instructionVar138 {}
impl imul_instructionVar138 {
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
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 104i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1352:1, end:1352:2))"]
#[derive(Clone, Debug)]
struct ineg_instructionVar139 {}
impl ineg_instructionVar139 {
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
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 116i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1361:1, end:1361:2))"]
#[derive(Clone, Debug)]
struct instanceof_instructionVar140 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl instanceof_instructionVar140 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 193i128 {
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
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1371:1, end:1371:2))"]
#[derive(Clone, Debug)]
struct invokedynamic_instructionVar141 {
    blank1: TokenField_blank1,
    blank2: TokenField_blank2,
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl invokedynamic_instructionVar141 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0);
        let extend: [DisplayElement; 9usize] = [
            DisplayElement::Literal("invokedynamic"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.blank1.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        let mut calc_index: i128 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c41 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 186i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c41(tokens_current, &mut context_instance)?;
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
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1376:1, end:1376:2))"]
#[derive(Clone, Debug)]
struct invokeinterface_instructionVar142 {
    count: TokenField_count,
    blank1: TokenField_blank1,
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl invokeinterface_instructionVar142 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0);
        let extend: [DisplayElement; 9usize] = [
            DisplayElement::Literal("invokeinterface"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_index),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.count.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 185i128 {
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
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1381:1, end:1381:2))"]
#[derive(Clone, Debug)]
struct invokespecial_instructionVar143 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl invokespecial_instructionVar143 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 183i128 {
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
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1386:1, end:1386:2))"]
#[derive(Clone, Debug)]
struct invokestatic_instructionVar144 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl invokestatic_instructionVar144 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 184i128 {
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
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1391:1, end:1391:2))"]
#[derive(Clone, Debug)]
struct invokevirtual_instructionVar145 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl invokevirtual_instructionVar145 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 182i128 {
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
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1397:1, end:1397:2))"]
#[derive(Clone, Debug)]
struct ior_instructionVar146 {}
impl ior_instructionVar146 {
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
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 128i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1409:1, end:1409:2))"]
#[derive(Clone, Debug)]
struct irem_instructionVar147 {}
impl irem_instructionVar147 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 112i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1421:1, end:1421:2))"]
#[derive(Clone, Debug)]
struct ireturn_instructionVar148 {}
impl ireturn_instructionVar148 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 172i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1427:1, end:1427:2))"]
#[derive(Clone, Debug)]
struct ishl_instructionVar149 {}
impl ishl_instructionVar149 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 120i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1439:1, end:1439:2))"]
#[derive(Clone, Debug)]
struct ishr_instructionVar150 {}
impl ishr_instructionVar150 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 122i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1451:1, end:1451:2))"]
#[derive(Clone, Debug)]
struct istore_instructionVar151 {
    index: TokenField_index,
}
impl istore_instructionVar151 {
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 54i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1459:1, end:1459:2))"]
#[derive(Clone, Debug)]
struct istore_0_instructionVar152 {}
impl istore_0_instructionVar152 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 59i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1467:1, end:1467:2))"]
#[derive(Clone, Debug)]
struct istore_1_instructionVar153 {}
impl istore_1_instructionVar153 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 60i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1475:1, end:1475:2))"]
#[derive(Clone, Debug)]
struct istore_2_instructionVar154 {}
impl istore_2_instructionVar154 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 61i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1483:1, end:1483:2))"]
#[derive(Clone, Debug)]
struct istore_3_instructionVar155 {}
impl istore_3_instructionVar155 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 62i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1491:1, end:1491:2))"]
#[derive(Clone, Debug)]
struct isub_instructionVar156 {}
impl isub_instructionVar156 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 100i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1503:1, end:1503:2))"]
#[derive(Clone, Debug)]
struct iushr_instructionVar157 {}
impl iushr_instructionVar157 {
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
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 124i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1515:1, end:1515:2))"]
#[derive(Clone, Debug)]
struct ixor_instructionVar158 {}
impl ixor_instructionVar158 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 130i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1527:1, end:1527:2))"]
#[derive(Clone, Debug)]
struct jsr_instructionVar159 {
    Branch: TableBranch,
}
impl jsr_instructionVar159 {
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
        let mut sub_pattern_c16 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 168i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1534:1, end:1534:2))"]
#[derive(Clone, Debug)]
struct jsr_w_instructionVar160 {
    Branch_w: TableBranch_w,
}
impl jsr_w_instructionVar160 {
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
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 201i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1541:1, end:1541:2))"]
#[derive(Clone, Debug)]
struct l2d_instructionVar161 {}
impl l2d_instructionVar161 {
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
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 138i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1550:1, end:1550:2))"]
#[derive(Clone, Debug)]
struct l2f_instructionVar162 {}
impl l2f_instructionVar162 {
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
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 137i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1559:1, end:1559:2))"]
#[derive(Clone, Debug)]
struct l2i_instructionVar163 {}
impl l2i_instructionVar163 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 136i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1568:1, end:1568:2))"]
#[derive(Clone, Debug)]
struct ladd_instructionVar164 {}
impl ladd_instructionVar164 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 97i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1579:1, end:1579:2))"]
#[derive(Clone, Debug)]
struct laload_instructionVar165 {}
impl laload_instructionVar165 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 47i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1592:1, end:1592:2))"]
#[derive(Clone, Debug)]
struct land_instructionVar166 {}
impl land_instructionVar166 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 127i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1603:1, end:1603:2))"]
#[derive(Clone, Debug)]
struct lastore_instructionVar167 {}
impl lastore_instructionVar167 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 80i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1616:1, end:1616:2))"]
#[derive(Clone, Debug)]
struct lcmp_instructionVar168 {}
impl lcmp_instructionVar168 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 148i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1627:1, end:1627:2))"]
#[derive(Clone, Debug)]
struct lconst_0_instructionVar169 {}
impl lconst_0_instructionVar169 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 9i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1633:1, end:1633:2))"]
#[derive(Clone, Debug)]
struct lconst_1_instructionVar170 {}
impl lconst_1_instructionVar170 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 10i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1639:1, end:1639:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar171 {
    index: TokenField_index,
}
impl ldc_instructionVar171 {
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
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 18i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1644:1, end:1644:2))"]
#[derive(Clone, Debug)]
struct ldc_w_instructionVar172 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl ldc_w_instructionVar172 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("ldc_w"),
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 19i128 {
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
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1649:1, end:1649:2))"]
#[derive(Clone, Debug)]
struct ldc2_w_instructionVar173 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl ldc2_w_instructionVar173 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0);
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("ldc2_w"),
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 20i128 {
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
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1654:1, end:1654:2))"]
#[derive(Clone, Debug)]
struct ldiv_instructionVar174 {}
impl ldiv_instructionVar174 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 109i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1665:1, end:1665:2))"]
#[derive(Clone, Debug)]
struct lload_instructionVar175 {
    index: TokenField_index,
}
impl lload_instructionVar175 {
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 22i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1672:1, end:1672:2))"]
#[derive(Clone, Debug)]
struct lload_0_instructionVar176 {}
impl lload_0_instructionVar176 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 30i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1679:1, end:1679:2))"]
#[derive(Clone, Debug)]
struct lload_1_instructionVar177 {}
impl lload_1_instructionVar177 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 31i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1686:1, end:1686:2))"]
#[derive(Clone, Debug)]
struct lload_2_instructionVar178 {}
impl lload_2_instructionVar178 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 32i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1693:1, end:1693:2))"]
#[derive(Clone, Debug)]
struct lload_3_instructionVar179 {}
impl lload_3_instructionVar179 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 33i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1700:1, end:1700:2))"]
#[derive(Clone, Debug)]
struct lmul_instructionVar180 {}
impl lmul_instructionVar180 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 105i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1711:1, end:1711:2))"]
#[derive(Clone, Debug)]
struct lneg_instructionVar181 {}
impl lneg_instructionVar181 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 117i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1757:1, end:1757:2))"]
#[derive(Clone, Debug)]
struct lookupswitch_instructionVar182 {
    dolookupswitch: Tabledolookupswitch,
    instruction: Box<Tableinstruction>,
}
impl lookupswitch_instructionVar182 {
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
            .unwrap()
            != 0i128
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_table_switch_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 171i128 {
            return None;
        }
        if context_instance
            .register()
            .read_alignmentPad_disassembly()
            .unwrap()
            != 0i128
        {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1761:1, end:1761:2))"]
#[derive(Clone, Debug)]
struct lookupswitch_instructionVar183 {
    dolookupswitch: Tabledolookupswitch,
    instruction: Box<Tableinstruction>,
}
impl lookupswitch_instructionVar183 {
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
            .unwrap()
            != 0i128
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_table_switch_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 171i128 {
            return None;
        }
        if context_instance
            .register()
            .read_alignmentPad_disassembly()
            .unwrap()
            != 1i128
        {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1765:1, end:1765:2))"]
#[derive(Clone, Debug)]
struct lookupswitch_instructionVar184 {
    dolookupswitch: Tabledolookupswitch,
    instruction: Box<Tableinstruction>,
}
impl lookupswitch_instructionVar184 {
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
            .unwrap()
            != 0i128
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_table_switch_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 171i128 {
            return None;
        }
        if context_instance
            .register()
            .read_alignmentPad_disassembly()
            .unwrap()
            != 2i128
        {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1769:1, end:1769:2))"]
#[derive(Clone, Debug)]
struct lookupswitch_instructionVar185 {
    dolookupswitch: Tabledolookupswitch,
    instruction: Box<Tableinstruction>,
}
impl lookupswitch_instructionVar185 {
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
            .unwrap()
            != 0i128
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_table_switch_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 171i128 {
            return None;
        }
        if context_instance
            .register()
            .read_alignmentPad_disassembly()
            .unwrap()
            != 3i128
        {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1775:1, end:1775:2))"]
#[derive(Clone, Debug)]
struct lor_instructionVar186 {}
impl lor_instructionVar186 {
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
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 129i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1786:1, end:1786:2))"]
#[derive(Clone, Debug)]
struct lrem_instructionVar187 {}
impl lrem_instructionVar187 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 113i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1797:1, end:1797:2))"]
#[derive(Clone, Debug)]
struct lreturn_instructionVar188 {}
impl lreturn_instructionVar188 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 173i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1803:1, end:1803:2))"]
#[derive(Clone, Debug)]
struct lshl_instructionVar189 {}
impl lshl_instructionVar189 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 121i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1814:1, end:1814:2))"]
#[derive(Clone, Debug)]
struct lshr_instructionVar190 {}
impl lshr_instructionVar190 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 123i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1825:1, end:1825:2))"]
#[derive(Clone, Debug)]
struct lstore_instructionVar191 {
    index: TokenField_index,
}
impl lstore_instructionVar191 {
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 55i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1833:1, end:1833:2))"]
#[derive(Clone, Debug)]
struct lstore_0_instructionVar192 {}
impl lstore_0_instructionVar192 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 63i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1841:1, end:1841:2))"]
#[derive(Clone, Debug)]
struct lstore_1_instructionVar193 {}
impl lstore_1_instructionVar193 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 64i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1849:1, end:1849:2))"]
#[derive(Clone, Debug)]
struct lstore_2_instructionVar194 {}
impl lstore_2_instructionVar194 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 65i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1857:1, end:1857:2))"]
#[derive(Clone, Debug)]
struct lstore_3_instructionVar195 {}
impl lstore_3_instructionVar195 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 66i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1865:1, end:1865:2))"]
#[derive(Clone, Debug)]
struct lsub_instructionVar196 {}
impl lsub_instructionVar196 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 101i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1876:1, end:1876:2))"]
#[derive(Clone, Debug)]
struct lushr_instructionVar197 {}
impl lushr_instructionVar197 {
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
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 125i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1887:1, end:1887:2))"]
#[derive(Clone, Debug)]
struct lxor_instructionVar198 {}
impl lxor_instructionVar198 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 131i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1898:1, end:1898:2))"]
#[derive(Clone, Debug)]
struct monitorenter_instructionVar199 {}
impl monitorenter_instructionVar199 {
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 194i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1905:1, end:1905:2))"]
#[derive(Clone, Debug)]
struct monitorexit_instructionVar200 {}
impl monitorexit_instructionVar200 {
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 195i128 {
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
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1912:1, end:1912:2))"]
#[derive(Clone, Debug)]
struct multianewarray_instructionVar201 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl multianewarray_instructionVar201 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 197i128 {
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
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1917:1, end:1917:2))"]
#[derive(Clone, Debug)]
struct new_instructionVar202 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl new_instructionVar202 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 187i128 {
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
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let indexbyte1 = token_parser.TokenFieldindexbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1924:1, end:1924:2))"]
#[derive(Clone, Debug)]
struct newarray_instructionVar203 {
    atype: TokenField_atype,
}
impl newarray_instructionVar203 {
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
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 188i128 {
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
        let atype = token_parser.TokenFieldatype();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { atype }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1934:1, end:1934:2))"]
#[derive(Clone, Debug)]
struct nop_instructionVar204 {}
impl nop_instructionVar204 {
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
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 0i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1938:1, end:1938:2))"]
#[derive(Clone, Debug)]
struct pop_instructionVar205 {}
impl pop_instructionVar205 {
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
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 87i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1943:1, end:1943:2))"]
#[derive(Clone, Debug)]
struct pop2_instructionVar206 {}
impl pop2_instructionVar206 {
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
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 88i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1948:1, end:1948:2))"]
#[derive(Clone, Debug)]
struct putfield_instructionVar207 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl putfield_instructionVar207 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 181i128 {
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
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1953:1, end:1953:2))"]
#[derive(Clone, Debug)]
struct putstatic_instructionVar208 {
    indexbyte1: TokenField_indexbyte1,
    indexbyte2: TokenField_indexbyte2,
}
impl putstatic_instructionVar208 {
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
        let mut calc_index: i128 = 0;
        calc_index = u32::try_from((8i128 | self.indexbyte2.disassembly()))
            .ok()
            .map(|shl| self.indexbyte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_index: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 179i128 {
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
        calc_index = u32::try_from(
            (8i128 | token_parser.TokenFieldindexbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldindexbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1958:1, end:1958:2))"]
#[derive(Clone, Debug)]
struct ret_instructionVar209 {
    index: TokenField_index,
}
impl ret_instructionVar209 {
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
        let mut sub_pattern_c16 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 169i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1965:1, end:1965:2))"]
#[derive(Clone, Debug)]
struct return_instructionVar210 {}
impl return_instructionVar210 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 177i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1970:1, end:1970:2))"]
#[derive(Clone, Debug)]
struct saload_instructionVar211 {}
impl saload_instructionVar211 {
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 53i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1983:1, end:1983:2))"]
#[derive(Clone, Debug)]
struct sastore_instructionVar212 {}
impl sastore_instructionVar212 {
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
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 86i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1995:1, end:1995:2))"]
#[derive(Clone, Debug)]
struct sipush_instructionVar213 {
    byte1: TokenField_byte1,
    byte2: TokenField_byte2,
}
impl sipush_instructionVar213 {
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
        let mut calc_short: i128 = 0;
        calc_short = u32::try_from((8i128 | self.byte2.disassembly()))
            .ok()
            .map(|shl| self.byte1.disassembly().checked_shl(shl))
            .flatten()
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
        let mut calc_short: i128 = 0;
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
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 17i128 {
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
        let byte1 = token_parser.TokenFieldbyte1();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_short = u32::try_from(
            (8i128 | token_parser.TokenFieldbyte2().disassembly()),
        )
        .ok()
        .map(|shl| {
            token_parser
                .TokenFieldbyte1()
                .disassembly()
                .checked_shl(shl)
        })
        .flatten()
        .unwrap_or(0);
        let byte2 = token_parser.TokenFieldbyte2();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { byte1, byte2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2001:1, end:2001:2))"]
#[derive(Clone, Debug)]
struct swap_instructionVar214 {}
impl swap_instructionVar214 {
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
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u32;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if token_parser.TokenFieldop().disassembly() != 95i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2048:1, end:2048:2))"]
#[derive(Clone, Debug)]
struct tableswitch_instructionVar215 {
    dotableswitch: Tabledotableswitch,
    instruction: Box<Tableinstruction>,
}
impl tableswitch_instructionVar215 {
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
            .unwrap()
            != 0i128
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_lookup_switch_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 170i128 {
            return None;
        }
        if context_instance
            .register()
            .read_alignmentPad_disassembly()
            .unwrap()
            != 0i128
        {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2052:1, end:2052:2))"]
#[derive(Clone, Debug)]
struct tableswitch_instructionVar216 {
    dotableswitch: Tabledotableswitch,
    instruction: Box<Tableinstruction>,
}
impl tableswitch_instructionVar216 {
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
            .unwrap()
            != 0i128
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_lookup_switch_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 170i128 {
            return None;
        }
        if context_instance
            .register()
            .read_alignmentPad_disassembly()
            .unwrap()
            != 1i128
        {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2056:1, end:2056:2))"]
#[derive(Clone, Debug)]
struct tableswitch_instructionVar217 {
    dotableswitch: Tabledotableswitch,
    instruction: Box<Tableinstruction>,
}
impl tableswitch_instructionVar217 {
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
            .unwrap()
            != 0i128
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_lookup_switch_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 170i128 {
            return None;
        }
        if context_instance
            .register()
            .read_alignmentPad_disassembly()
            .unwrap()
            != 2i128
        {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2060:1, end:2060:2))"]
#[derive(Clone, Debug)]
struct tableswitch_instructionVar218 {
    dotableswitch: Tabledotableswitch,
    instruction: Box<Tableinstruction>,
}
impl tableswitch_instructionVar218 {
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
            .unwrap()
            != 0i128
        {
            return None;
        }
        if context_instance
            .register()
            .read_in_lookup_switch_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        if token_parser.TokenFieldop().disassembly() != 170i128 {
            return None;
        }
        if context_instance
            .register()
            .read_alignmentPad_disassembly()
            .unwrap()
            != 3i128
        {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1730:1, end:1730:2))"]
#[derive(Clone, Debug)]
struct instructionVar219 {
    LookupSwitch_match: TableLookupSwitch_match,
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
        self.LookupSwitch_match.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
            .unwrap()
            .wrapping_sub(1i128);
        context_instance
            .register_mut()
            .write_switch_num_disassembly(tmp)
            .unwrap();
        let mut sub_pattern_c38 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 0u64 as u32;
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 1i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c38(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1735:1, end:1735:2))"]
#[derive(Clone, Debug)]
struct instructionVar220 {
    LookupSwitch_match: TableLookupSwitch_match,
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
        self.LookupSwitch_match.display_extend(
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
        let tmp = 0i128;
        context_instance
            .register_mut()
            .write_in_lookup_switch_disassembly(tmp)
            .unwrap();
        let mut sub_pattern_c26 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 0u64 as u32;
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 1i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_switch_num_disassembly()
                .unwrap()
                != 1i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 0i128
            {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2023:1, end:2023:2))"]
#[derive(Clone, Debug)]
struct instructionVar221 {
    Switch_offset: TableSwitch_offset,
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
        self.Switch_offset.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
            .unwrap()
            .wrapping_sub(1i128);
        context_instance
            .register_mut()
            .write_switch_num_disassembly(tmp)
            .unwrap();
        let mut sub_pattern_c33 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 0u64 as u32;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 1i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c33(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2028:1, end:2028:2))"]
#[derive(Clone, Debug)]
struct instructionVar222 {
    Switch_offset: TableSwitch_offset,
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
        self.Switch_offset.display_extend(
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
        let tmp = 0i128;
        context_instance
            .register_mut()
            .write_in_table_switch_disassembly(tmp)
            .unwrap();
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 0u64 as u32;
            if context_instance
                .register()
                .read_in_table_switch_disassembly()
                .unwrap()
                != 1i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_in_lookup_switch_disassembly()
                .unwrap()
                != 0i128
            {
                return None;
            }
            if context_instance
                .register()
                .read_switch_num_disassembly()
                .unwrap()
                != 0i128
            {
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
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(wide_iload_instructionVar0),
    Var1(wide_fload_instructionVar1),
    Var2(wide_aload_instructionVar2),
    Var3(wide_lload_instructionVar3),
    Var4(wide_dload_instructionVar4),
    Var5(wide_istore_instructionVar5),
    Var6(wide_fstore_instructionVar6),
    Var7(wide_astore_instructionVar7),
    Var8(wide_lstore_instructionVar8),
    Var9(wide_dstore_instructionVar9),
    Var10(wide_ret_instructionVar10),
    Var11(iinc_w_instructionVar11),
    Var12(aaload_instructionVar12),
    Var13(aastore_instructionVar13),
    Var14(aconst_null_instructionVar14),
    Var15(aload_instructionVar15),
    Var16(aload_0_instructionVar16),
    Var17(aload_1_instructionVar17),
    Var18(aload_2_instructionVar18),
    Var19(aload_3_instructionVar19),
    Var20(anewarray_instructionVar20),
    Var21(areturn_instructionVar21),
    Var22(arraylength_instructionVar22),
    Var23(astore_instructionVar23),
    Var24(astore_0_instructionVar24),
    Var25(astore_1_instructionVar25),
    Var26(astore_2_instructionVar26),
    Var27(astore_3_instructionVar27),
    Var28(athrow_instructionVar28),
    Var29(baload_instructionVar29),
    Var30(bastore_instructionVar30),
    Var31(bipush_instructionVar31),
    Var32(caload_instructionVar32),
    Var33(castore_instructionVar33),
    Var34(checkcast_instructionVar34),
    Var35(d2f_instructionVar35),
    Var36(d2i_instructionVar36),
    Var37(d2l_instructionVar37),
    Var38(dadd_instructionVar38),
    Var39(daload_instructionVar39),
    Var40(dastore_instructionVar40),
    Var41(dcmpg_instructionVar41),
    Var42(dcmpl_instructionVar42),
    Var43(dconst_0_instructionVar43),
    Var44(dconst_1_instructionVar44),
    Var45(ddiv_instructionVar45),
    Var46(dload_instructionVar46),
    Var47(dload_0_instructionVar47),
    Var48(dload_1_instructionVar48),
    Var49(dload_2_instructionVar49),
    Var50(dload_3_instructionVar50),
    Var51(dmul_instructionVar51),
    Var52(dneg_instructionVar52),
    Var53(drem_instructionVar53),
    Var54(dreturn_instructionVar54),
    Var55(dstore_instructionVar55),
    Var56(dstore_0_instructionVar56),
    Var57(dstore_1_instructionVar57),
    Var58(dstore_2_instructionVar58),
    Var59(dstore_3_instructionVar59),
    Var60(dsub_instructionVar60),
    Var61(dup_instructionVar61),
    Var62(dup_x1_instructionVar62),
    Var63(dup_x2_instructionVar63),
    Var64(dup2_instructionVar64),
    Var65(dup2_x1_instructionVar65),
    Var66(dup2_x2_instructionVar66),
    Var67(f2d_instructionVar67),
    Var68(f2i_instructionVar68),
    Var69(f2l_instructionVar69),
    Var70(fadd_instructionVar70),
    Var71(faload_instructionVar71),
    Var72(fastore_instructionVar72),
    Var73(fcmpg_instructionVar73),
    Var74(fcmpl_instructionVar74),
    Var75(fconst_0_instructionVar75),
    Var76(fconst_1_instructionVar76),
    Var77(fconst_2_instructionVar77),
    Var78(fdiv_instructionVar78),
    Var79(fload_instructionVar79),
    Var80(fload_0_instructionVar80),
    Var81(fload_1_instructionVar81),
    Var82(fload_2_instructionVar82),
    Var83(fload_3_instructionVar83),
    Var84(fmul_instructionVar84),
    Var85(fneg_instructionVar85),
    Var86(frem_instructionVar86),
    Var87(freturn_instructionVar87),
    Var88(fstore_instructionVar88),
    Var89(fstore_0_instructionVar89),
    Var90(fstore_1_instructionVar90),
    Var91(fstore_2_instructionVar91),
    Var92(fstore_3_instructionVar92),
    Var93(fsub_instructionVar93),
    Var94(getfield_instructionVar94),
    Var95(getstatic_instructionVar95),
    Var96(goto_instructionVar96),
    Var97(goto_w_instructionVar97),
    Var98(i2b_instructionVar98),
    Var99(i2c_instructionVar99),
    Var100(i2d_instructionVar100),
    Var101(i2f_instructionVar101),
    Var102(i2l_instructionVar102),
    Var103(i2s_instructionVar103),
    Var104(iadd_instructionVar104),
    Var105(iaload_instructionVar105),
    Var106(iand_instructionVar106),
    Var107(iastore_instructionVar107),
    Var108(iconst_m1_instructionVar108),
    Var109(iconst_0_instructionVar109),
    Var110(iconst_1_instructionVar110),
    Var111(iconst_2_instructionVar111),
    Var112(iconst_3_instructionVar112),
    Var113(iconst_4_instructionVar113),
    Var114(iconst_5_instructionVar114),
    Var115(idiv_instructionVar115),
    Var116(if_acmpeq_instructionVar116),
    Var117(if_acmpne_instructionVar117),
    Var118(if_icmpeq_instructionVar118),
    Var119(if_icmpne_instructionVar119),
    Var120(if_icmplt_instructionVar120),
    Var121(if_icmpge_instructionVar121),
    Var122(if_icmpgt_instructionVar122),
    Var123(if_icmple_instructionVar123),
    Var124(ifeq_instructionVar124),
    Var125(ifne_instructionVar125),
    Var126(iflt_instructionVar126),
    Var127(ifge_instructionVar127),
    Var128(ifgt_instructionVar128),
    Var129(ifle_instructionVar129),
    Var130(ifnonnull_instructionVar130),
    Var131(ifnull_instructionVar131),
    Var132(iinc_instructionVar132),
    Var133(iload_instructionVar133),
    Var134(iload_0_instructionVar134),
    Var135(iload_1_instructionVar135),
    Var136(iload_2_instructionVar136),
    Var137(iload_3_instructionVar137),
    Var138(imul_instructionVar138),
    Var139(ineg_instructionVar139),
    Var140(instanceof_instructionVar140),
    Var141(invokedynamic_instructionVar141),
    Var142(invokeinterface_instructionVar142),
    Var143(invokespecial_instructionVar143),
    Var144(invokestatic_instructionVar144),
    Var145(invokevirtual_instructionVar145),
    Var146(ior_instructionVar146),
    Var147(irem_instructionVar147),
    Var148(ireturn_instructionVar148),
    Var149(ishl_instructionVar149),
    Var150(ishr_instructionVar150),
    Var151(istore_instructionVar151),
    Var152(istore_0_instructionVar152),
    Var153(istore_1_instructionVar153),
    Var154(istore_2_instructionVar154),
    Var155(istore_3_instructionVar155),
    Var156(isub_instructionVar156),
    Var157(iushr_instructionVar157),
    Var158(ixor_instructionVar158),
    Var159(jsr_instructionVar159),
    Var160(jsr_w_instructionVar160),
    Var161(l2d_instructionVar161),
    Var162(l2f_instructionVar162),
    Var163(l2i_instructionVar163),
    Var164(ladd_instructionVar164),
    Var165(laload_instructionVar165),
    Var166(land_instructionVar166),
    Var167(lastore_instructionVar167),
    Var168(lcmp_instructionVar168),
    Var169(lconst_0_instructionVar169),
    Var170(lconst_1_instructionVar170),
    Var171(ldc_instructionVar171),
    Var172(ldc_w_instructionVar172),
    Var173(ldc2_w_instructionVar173),
    Var174(ldiv_instructionVar174),
    Var175(lload_instructionVar175),
    Var176(lload_0_instructionVar176),
    Var177(lload_1_instructionVar177),
    Var178(lload_2_instructionVar178),
    Var179(lload_3_instructionVar179),
    Var180(lmul_instructionVar180),
    Var181(lneg_instructionVar181),
    Var182(lookupswitch_instructionVar182),
    Var183(lookupswitch_instructionVar183),
    Var184(lookupswitch_instructionVar184),
    Var185(lookupswitch_instructionVar185),
    Var186(lor_instructionVar186),
    Var187(lrem_instructionVar187),
    Var188(lreturn_instructionVar188),
    Var189(lshl_instructionVar189),
    Var190(lshr_instructionVar190),
    Var191(lstore_instructionVar191),
    Var192(lstore_0_instructionVar192),
    Var193(lstore_1_instructionVar193),
    Var194(lstore_2_instructionVar194),
    Var195(lstore_3_instructionVar195),
    Var196(lsub_instructionVar196),
    Var197(lushr_instructionVar197),
    Var198(lxor_instructionVar198),
    Var199(monitorenter_instructionVar199),
    Var200(monitorexit_instructionVar200),
    Var201(multianewarray_instructionVar201),
    Var202(new_instructionVar202),
    Var203(newarray_instructionVar203),
    Var204(nop_instructionVar204),
    Var205(pop_instructionVar205),
    Var206(pop2_instructionVar206),
    Var207(putfield_instructionVar207),
    Var208(putstatic_instructionVar208),
    Var209(ret_instructionVar209),
    Var210(return_instructionVar210),
    Var211(saload_instructionVar211),
    Var212(sastore_instructionVar212),
    Var213(sipush_instructionVar213),
    Var214(swap_instructionVar214),
    Var215(tableswitch_instructionVar215),
    Var216(tableswitch_instructionVar216),
    Var217(tableswitch_instructionVar217),
    Var218(tableswitch_instructionVar218),
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
        if let Some((inst_len, parsed)) = wide_iload_instructionVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = wide_fload_instructionVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = wide_aload_instructionVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = wide_lload_instructionVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = wide_dload_instructionVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) = wide_istore_instructionVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) = wide_fstore_instructionVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) = wide_astore_instructionVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) = wide_lstore_instructionVar8::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) = wide_dstore_instructionVar9::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) = wide_ret_instructionVar10::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) = iinc_w_instructionVar11::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) = aaload_instructionVar12::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) = aastore_instructionVar13::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) = aconst_null_instructionVar14::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) = aload_instructionVar15::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        if let Some((inst_len, parsed)) = aload_0_instructionVar16::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var16(parsed)));
        }
        if let Some((inst_len, parsed)) = aload_1_instructionVar17::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var17(parsed)));
        }
        if let Some((inst_len, parsed)) = aload_2_instructionVar18::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var18(parsed)));
        }
        if let Some((inst_len, parsed)) = aload_3_instructionVar19::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var19(parsed)));
        }
        if let Some((inst_len, parsed)) = anewarray_instructionVar20::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var20(parsed)));
        }
        if let Some((inst_len, parsed)) = areturn_instructionVar21::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var21(parsed)));
        }
        if let Some((inst_len, parsed)) = arraylength_instructionVar22::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var22(parsed)));
        }
        if let Some((inst_len, parsed)) = astore_instructionVar23::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var23(parsed)));
        }
        if let Some((inst_len, parsed)) = astore_0_instructionVar24::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var24(parsed)));
        }
        if let Some((inst_len, parsed)) = astore_1_instructionVar25::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var25(parsed)));
        }
        if let Some((inst_len, parsed)) = astore_2_instructionVar26::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var26(parsed)));
        }
        if let Some((inst_len, parsed)) = astore_3_instructionVar27::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var27(parsed)));
        }
        if let Some((inst_len, parsed)) = athrow_instructionVar28::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var28(parsed)));
        }
        if let Some((inst_len, parsed)) = baload_instructionVar29::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var29(parsed)));
        }
        if let Some((inst_len, parsed)) = bastore_instructionVar30::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var30(parsed)));
        }
        if let Some((inst_len, parsed)) = bipush_instructionVar31::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var31(parsed)));
        }
        if let Some((inst_len, parsed)) = caload_instructionVar32::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var32(parsed)));
        }
        if let Some((inst_len, parsed)) = castore_instructionVar33::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var33(parsed)));
        }
        if let Some((inst_len, parsed)) = checkcast_instructionVar34::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var34(parsed)));
        }
        if let Some((inst_len, parsed)) = d2f_instructionVar35::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var35(parsed)));
        }
        if let Some((inst_len, parsed)) = d2i_instructionVar36::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var36(parsed)));
        }
        if let Some((inst_len, parsed)) = d2l_instructionVar37::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var37(parsed)));
        }
        if let Some((inst_len, parsed)) = dadd_instructionVar38::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var38(parsed)));
        }
        if let Some((inst_len, parsed)) = daload_instructionVar39::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var39(parsed)));
        }
        if let Some((inst_len, parsed)) = dastore_instructionVar40::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var40(parsed)));
        }
        if let Some((inst_len, parsed)) = dcmpg_instructionVar41::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var41(parsed)));
        }
        if let Some((inst_len, parsed)) = dcmpl_instructionVar42::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var42(parsed)));
        }
        if let Some((inst_len, parsed)) = dconst_0_instructionVar43::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var43(parsed)));
        }
        if let Some((inst_len, parsed)) = dconst_1_instructionVar44::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var44(parsed)));
        }
        if let Some((inst_len, parsed)) = ddiv_instructionVar45::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var45(parsed)));
        }
        if let Some((inst_len, parsed)) = dload_instructionVar46::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var46(parsed)));
        }
        if let Some((inst_len, parsed)) = dload_0_instructionVar47::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var47(parsed)));
        }
        if let Some((inst_len, parsed)) = dload_1_instructionVar48::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var48(parsed)));
        }
        if let Some((inst_len, parsed)) = dload_2_instructionVar49::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var49(parsed)));
        }
        if let Some((inst_len, parsed)) = dload_3_instructionVar50::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var50(parsed)));
        }
        if let Some((inst_len, parsed)) = dmul_instructionVar51::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var51(parsed)));
        }
        if let Some((inst_len, parsed)) = dneg_instructionVar52::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var52(parsed)));
        }
        if let Some((inst_len, parsed)) = drem_instructionVar53::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var53(parsed)));
        }
        if let Some((inst_len, parsed)) = dreturn_instructionVar54::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var54(parsed)));
        }
        if let Some((inst_len, parsed)) = dstore_instructionVar55::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var55(parsed)));
        }
        if let Some((inst_len, parsed)) = dstore_0_instructionVar56::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var56(parsed)));
        }
        if let Some((inst_len, parsed)) = dstore_1_instructionVar57::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var57(parsed)));
        }
        if let Some((inst_len, parsed)) = dstore_2_instructionVar58::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var58(parsed)));
        }
        if let Some((inst_len, parsed)) = dstore_3_instructionVar59::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var59(parsed)));
        }
        if let Some((inst_len, parsed)) = dsub_instructionVar60::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var60(parsed)));
        }
        if let Some((inst_len, parsed)) = dup_instructionVar61::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var61(parsed)));
        }
        if let Some((inst_len, parsed)) = dup_x1_instructionVar62::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var62(parsed)));
        }
        if let Some((inst_len, parsed)) = dup_x2_instructionVar63::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var63(parsed)));
        }
        if let Some((inst_len, parsed)) = dup2_instructionVar64::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var64(parsed)));
        }
        if let Some((inst_len, parsed)) = dup2_x1_instructionVar65::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var65(parsed)));
        }
        if let Some((inst_len, parsed)) = dup2_x2_instructionVar66::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var66(parsed)));
        }
        if let Some((inst_len, parsed)) = f2d_instructionVar67::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var67(parsed)));
        }
        if let Some((inst_len, parsed)) = f2i_instructionVar68::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var68(parsed)));
        }
        if let Some((inst_len, parsed)) = f2l_instructionVar69::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var69(parsed)));
        }
        if let Some((inst_len, parsed)) = fadd_instructionVar70::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var70(parsed)));
        }
        if let Some((inst_len, parsed)) = faload_instructionVar71::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var71(parsed)));
        }
        if let Some((inst_len, parsed)) = fastore_instructionVar72::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var72(parsed)));
        }
        if let Some((inst_len, parsed)) = fcmpg_instructionVar73::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var73(parsed)));
        }
        if let Some((inst_len, parsed)) = fcmpl_instructionVar74::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var74(parsed)));
        }
        if let Some((inst_len, parsed)) = fconst_0_instructionVar75::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var75(parsed)));
        }
        if let Some((inst_len, parsed)) = fconst_1_instructionVar76::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var76(parsed)));
        }
        if let Some((inst_len, parsed)) = fconst_2_instructionVar77::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var77(parsed)));
        }
        if let Some((inst_len, parsed)) = fdiv_instructionVar78::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var78(parsed)));
        }
        if let Some((inst_len, parsed)) = fload_instructionVar79::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var79(parsed)));
        }
        if let Some((inst_len, parsed)) = fload_0_instructionVar80::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var80(parsed)));
        }
        if let Some((inst_len, parsed)) = fload_1_instructionVar81::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var81(parsed)));
        }
        if let Some((inst_len, parsed)) = fload_2_instructionVar82::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var82(parsed)));
        }
        if let Some((inst_len, parsed)) = fload_3_instructionVar83::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var83(parsed)));
        }
        if let Some((inst_len, parsed)) = fmul_instructionVar84::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var84(parsed)));
        }
        if let Some((inst_len, parsed)) = fneg_instructionVar85::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var85(parsed)));
        }
        if let Some((inst_len, parsed)) = frem_instructionVar86::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var86(parsed)));
        }
        if let Some((inst_len, parsed)) = freturn_instructionVar87::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var87(parsed)));
        }
        if let Some((inst_len, parsed)) = fstore_instructionVar88::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var88(parsed)));
        }
        if let Some((inst_len, parsed)) = fstore_0_instructionVar89::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var89(parsed)));
        }
        if let Some((inst_len, parsed)) = fstore_1_instructionVar90::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var90(parsed)));
        }
        if let Some((inst_len, parsed)) = fstore_2_instructionVar91::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var91(parsed)));
        }
        if let Some((inst_len, parsed)) = fstore_3_instructionVar92::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var92(parsed)));
        }
        if let Some((inst_len, parsed)) = fsub_instructionVar93::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var93(parsed)));
        }
        if let Some((inst_len, parsed)) = getfield_instructionVar94::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var94(parsed)));
        }
        if let Some((inst_len, parsed)) = getstatic_instructionVar95::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var95(parsed)));
        }
        if let Some((inst_len, parsed)) = goto_instructionVar96::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var96(parsed)));
        }
        if let Some((inst_len, parsed)) = goto_w_instructionVar97::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var97(parsed)));
        }
        if let Some((inst_len, parsed)) = i2b_instructionVar98::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var98(parsed)));
        }
        if let Some((inst_len, parsed)) = i2c_instructionVar99::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var99(parsed)));
        }
        if let Some((inst_len, parsed)) = i2d_instructionVar100::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var100(parsed)));
        }
        if let Some((inst_len, parsed)) = i2f_instructionVar101::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var101(parsed)));
        }
        if let Some((inst_len, parsed)) = i2l_instructionVar102::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var102(parsed)));
        }
        if let Some((inst_len, parsed)) = i2s_instructionVar103::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var103(parsed)));
        }
        if let Some((inst_len, parsed)) = iadd_instructionVar104::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var104(parsed)));
        }
        if let Some((inst_len, parsed)) = iaload_instructionVar105::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var105(parsed)));
        }
        if let Some((inst_len, parsed)) = iand_instructionVar106::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var106(parsed)));
        }
        if let Some((inst_len, parsed)) = iastore_instructionVar107::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var107(parsed)));
        }
        if let Some((inst_len, parsed)) = iconst_m1_instructionVar108::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var108(parsed)));
        }
        if let Some((inst_len, parsed)) = iconst_0_instructionVar109::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var109(parsed)));
        }
        if let Some((inst_len, parsed)) = iconst_1_instructionVar110::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var110(parsed)));
        }
        if let Some((inst_len, parsed)) = iconst_2_instructionVar111::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var111(parsed)));
        }
        if let Some((inst_len, parsed)) = iconst_3_instructionVar112::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var112(parsed)));
        }
        if let Some((inst_len, parsed)) = iconst_4_instructionVar113::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var113(parsed)));
        }
        if let Some((inst_len, parsed)) = iconst_5_instructionVar114::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var114(parsed)));
        }
        if let Some((inst_len, parsed)) = idiv_instructionVar115::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var115(parsed)));
        }
        if let Some((inst_len, parsed)) = if_acmpeq_instructionVar116::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var116(parsed)));
        }
        if let Some((inst_len, parsed)) = if_acmpne_instructionVar117::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var117(parsed)));
        }
        if let Some((inst_len, parsed)) = if_icmpeq_instructionVar118::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var118(parsed)));
        }
        if let Some((inst_len, parsed)) = if_icmpne_instructionVar119::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var119(parsed)));
        }
        if let Some((inst_len, parsed)) = if_icmplt_instructionVar120::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var120(parsed)));
        }
        if let Some((inst_len, parsed)) = if_icmpge_instructionVar121::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var121(parsed)));
        }
        if let Some((inst_len, parsed)) = if_icmpgt_instructionVar122::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var122(parsed)));
        }
        if let Some((inst_len, parsed)) = if_icmple_instructionVar123::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var123(parsed)));
        }
        if let Some((inst_len, parsed)) = ifeq_instructionVar124::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var124(parsed)));
        }
        if let Some((inst_len, parsed)) = ifne_instructionVar125::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var125(parsed)));
        }
        if let Some((inst_len, parsed)) = iflt_instructionVar126::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var126(parsed)));
        }
        if let Some((inst_len, parsed)) = ifge_instructionVar127::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var127(parsed)));
        }
        if let Some((inst_len, parsed)) = ifgt_instructionVar128::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var128(parsed)));
        }
        if let Some((inst_len, parsed)) = ifle_instructionVar129::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var129(parsed)));
        }
        if let Some((inst_len, parsed)) = ifnonnull_instructionVar130::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var130(parsed)));
        }
        if let Some((inst_len, parsed)) = ifnull_instructionVar131::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var131(parsed)));
        }
        if let Some((inst_len, parsed)) = iinc_instructionVar132::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var132(parsed)));
        }
        if let Some((inst_len, parsed)) = iload_instructionVar133::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var133(parsed)));
        }
        if let Some((inst_len, parsed)) = iload_0_instructionVar134::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var134(parsed)));
        }
        if let Some((inst_len, parsed)) = iload_1_instructionVar135::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var135(parsed)));
        }
        if let Some((inst_len, parsed)) = iload_2_instructionVar136::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var136(parsed)));
        }
        if let Some((inst_len, parsed)) = iload_3_instructionVar137::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var137(parsed)));
        }
        if let Some((inst_len, parsed)) = imul_instructionVar138::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var138(parsed)));
        }
        if let Some((inst_len, parsed)) = ineg_instructionVar139::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var139(parsed)));
        }
        if let Some((inst_len, parsed)) = instanceof_instructionVar140::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var140(parsed)));
        }
        if let Some((inst_len, parsed)) = invokedynamic_instructionVar141::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var141(parsed)));
        }
        if let Some((inst_len, parsed)) =
            invokeinterface_instructionVar142::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            )
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var142(parsed)));
        }
        if let Some((inst_len, parsed)) = invokespecial_instructionVar143::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var143(parsed)));
        }
        if let Some((inst_len, parsed)) = invokestatic_instructionVar144::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var144(parsed)));
        }
        if let Some((inst_len, parsed)) = invokevirtual_instructionVar145::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var145(parsed)));
        }
        if let Some((inst_len, parsed)) = ior_instructionVar146::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var146(parsed)));
        }
        if let Some((inst_len, parsed)) = irem_instructionVar147::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var147(parsed)));
        }
        if let Some((inst_len, parsed)) = ireturn_instructionVar148::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var148(parsed)));
        }
        if let Some((inst_len, parsed)) = ishl_instructionVar149::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var149(parsed)));
        }
        if let Some((inst_len, parsed)) = ishr_instructionVar150::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var150(parsed)));
        }
        if let Some((inst_len, parsed)) = istore_instructionVar151::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var151(parsed)));
        }
        if let Some((inst_len, parsed)) = istore_0_instructionVar152::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var152(parsed)));
        }
        if let Some((inst_len, parsed)) = istore_1_instructionVar153::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var153(parsed)));
        }
        if let Some((inst_len, parsed)) = istore_2_instructionVar154::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var154(parsed)));
        }
        if let Some((inst_len, parsed)) = istore_3_instructionVar155::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var155(parsed)));
        }
        if let Some((inst_len, parsed)) = isub_instructionVar156::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var156(parsed)));
        }
        if let Some((inst_len, parsed)) = iushr_instructionVar157::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var157(parsed)));
        }
        if let Some((inst_len, parsed)) = ixor_instructionVar158::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var158(parsed)));
        }
        if let Some((inst_len, parsed)) = jsr_instructionVar159::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var159(parsed)));
        }
        if let Some((inst_len, parsed)) = jsr_w_instructionVar160::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var160(parsed)));
        }
        if let Some((inst_len, parsed)) = l2d_instructionVar161::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var161(parsed)));
        }
        if let Some((inst_len, parsed)) = l2f_instructionVar162::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var162(parsed)));
        }
        if let Some((inst_len, parsed)) = l2i_instructionVar163::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var163(parsed)));
        }
        if let Some((inst_len, parsed)) = ladd_instructionVar164::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var164(parsed)));
        }
        if let Some((inst_len, parsed)) = laload_instructionVar165::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var165(parsed)));
        }
        if let Some((inst_len, parsed)) = land_instructionVar166::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var166(parsed)));
        }
        if let Some((inst_len, parsed)) = lastore_instructionVar167::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var167(parsed)));
        }
        if let Some((inst_len, parsed)) = lcmp_instructionVar168::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var168(parsed)));
        }
        if let Some((inst_len, parsed)) = lconst_0_instructionVar169::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var169(parsed)));
        }
        if let Some((inst_len, parsed)) = lconst_1_instructionVar170::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var170(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_instructionVar171::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var171(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_w_instructionVar172::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var172(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc2_w_instructionVar173::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var173(parsed)));
        }
        if let Some((inst_len, parsed)) = ldiv_instructionVar174::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var174(parsed)));
        }
        if let Some((inst_len, parsed)) = lload_instructionVar175::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var175(parsed)));
        }
        if let Some((inst_len, parsed)) = lload_0_instructionVar176::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var176(parsed)));
        }
        if let Some((inst_len, parsed)) = lload_1_instructionVar177::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var177(parsed)));
        }
        if let Some((inst_len, parsed)) = lload_2_instructionVar178::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var178(parsed)));
        }
        if let Some((inst_len, parsed)) = lload_3_instructionVar179::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var179(parsed)));
        }
        if let Some((inst_len, parsed)) = lmul_instructionVar180::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var180(parsed)));
        }
        if let Some((inst_len, parsed)) = lneg_instructionVar181::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var181(parsed)));
        }
        if let Some((inst_len, parsed)) = lookupswitch_instructionVar182::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var182(parsed)));
        }
        if let Some((inst_len, parsed)) = lookupswitch_instructionVar183::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var183(parsed)));
        }
        if let Some((inst_len, parsed)) = lookupswitch_instructionVar184::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var184(parsed)));
        }
        if let Some((inst_len, parsed)) = lookupswitch_instructionVar185::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var185(parsed)));
        }
        if let Some((inst_len, parsed)) = lor_instructionVar186::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var186(parsed)));
        }
        if let Some((inst_len, parsed)) = lrem_instructionVar187::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var187(parsed)));
        }
        if let Some((inst_len, parsed)) = lreturn_instructionVar188::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var188(parsed)));
        }
        if let Some((inst_len, parsed)) = lshl_instructionVar189::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var189(parsed)));
        }
        if let Some((inst_len, parsed)) = lshr_instructionVar190::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var190(parsed)));
        }
        if let Some((inst_len, parsed)) = lstore_instructionVar191::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var191(parsed)));
        }
        if let Some((inst_len, parsed)) = lstore_0_instructionVar192::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var192(parsed)));
        }
        if let Some((inst_len, parsed)) = lstore_1_instructionVar193::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var193(parsed)));
        }
        if let Some((inst_len, parsed)) = lstore_2_instructionVar194::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var194(parsed)));
        }
        if let Some((inst_len, parsed)) = lstore_3_instructionVar195::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var195(parsed)));
        }
        if let Some((inst_len, parsed)) = lsub_instructionVar196::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var196(parsed)));
        }
        if let Some((inst_len, parsed)) = lushr_instructionVar197::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var197(parsed)));
        }
        if let Some((inst_len, parsed)) = lxor_instructionVar198::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var198(parsed)));
        }
        if let Some((inst_len, parsed)) = monitorenter_instructionVar199::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var199(parsed)));
        }
        if let Some((inst_len, parsed)) = monitorexit_instructionVar200::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var200(parsed)));
        }
        if let Some((inst_len, parsed)) =
            multianewarray_instructionVar201::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            )
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var201(parsed)));
        }
        if let Some((inst_len, parsed)) = new_instructionVar202::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var202(parsed)));
        }
        if let Some((inst_len, parsed)) = newarray_instructionVar203::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var203(parsed)));
        }
        if let Some((inst_len, parsed)) = nop_instructionVar204::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var204(parsed)));
        }
        if let Some((inst_len, parsed)) = pop_instructionVar205::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var205(parsed)));
        }
        if let Some((inst_len, parsed)) = pop2_instructionVar206::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var206(parsed)));
        }
        if let Some((inst_len, parsed)) = putfield_instructionVar207::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var207(parsed)));
        }
        if let Some((inst_len, parsed)) = putstatic_instructionVar208::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var208(parsed)));
        }
        if let Some((inst_len, parsed)) = ret_instructionVar209::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var209(parsed)));
        }
        if let Some((inst_len, parsed)) = return_instructionVar210::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var210(parsed)));
        }
        if let Some((inst_len, parsed)) = saload_instructionVar211::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var211(parsed)));
        }
        if let Some((inst_len, parsed)) = sastore_instructionVar212::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var212(parsed)));
        }
        if let Some((inst_len, parsed)) = sipush_instructionVar213::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var213(parsed)));
        }
        if let Some((inst_len, parsed)) = swap_instructionVar214::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var214(parsed)));
        }
        if let Some((inst_len, parsed)) = tableswitch_instructionVar215::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var215(parsed)));
        }
        if let Some((inst_len, parsed)) = tableswitch_instructionVar216::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var216(parsed)));
        }
        if let Some((inst_len, parsed)) = tableswitch_instructionVar217::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var217(parsed)));
        }
        if let Some((inst_len, parsed)) = tableswitch_instructionVar218::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:204:1, end:204:7))"]
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
        let mut calc_addr: i128 = 0;
        calc_addr = i128::try_from(inst_start).unwrap().wrapping_add(
            u32::try_from((8i128 | self.branchbyte2.disassembly()))
                .ok()
                .map(|shl| self.branchbyte1.disassembly().checked_shl(shl))
                .flatten()
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
        let mut calc_addr: i128 = 0;
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let branchbyte1 = token_parser.TokenFieldbranchbyte1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_addr = i128::try_from(inst_start).unwrap().wrapping_add(
            u32::try_from(
                (8i128 | token_parser.TokenFieldbranchbyte2().disassembly()),
            )
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldbranchbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:209:1, end:209:9))"]
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
        let mut calc_addr: i128 = 0;
        calc_addr = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .map(|shl| self.branchbyte1.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .map(|shl| self.branchbyte2.disassembly().checked_shl(shl))
                    .flatten()
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .map(|shl| self.branchbyte3.disassembly().checked_shl(shl))
                    .flatten()
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
        let mut calc_addr: i128 = 0;
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
        calc_addr = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldbranchbyte1()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .map(|shl| {
                        token_parser
                            .TokenFieldbranchbyte2()
                            .disassembly()
                            .checked_shl(shl)
                    })
                    .flatten()
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .map(|shl| {
                        token_parser
                            .TokenFieldbranchbyte3()
                            .disassembly()
                            .checked_shl(shl)
                    })
                    .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:214:1, end:214:8))"]
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
        let mut calc_addr: i128 = 0;
        calc_addr = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .map(|shl| self.defaultbyte1.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .map(|shl| self.defaultbyte2.disassembly().checked_shl(shl))
                    .flatten()
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .map(|shl| self.defaultbyte3.disassembly().checked_shl(shl))
                    .flatten()
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
        let mut calc_addr: i128 = 0;
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
        calc_addr = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFielddefaultbyte1()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .map(|shl| {
                        token_parser
                            .TokenFielddefaultbyte2()
                            .disassembly()
                            .checked_shl(shl)
                    })
                    .flatten()
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .map(|shl| {
                        token_parser
                            .TokenFielddefaultbyte3()
                            .disassembly()
                            .checked_shl(shl)
                    })
                    .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1725:1, end:1725:19))"]
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
        let mut calc_match: i128 = 0;
        let mut calc__offset: i128 = 0;
        calc_match = (((u32::try_from(24i128)
            .ok()
            .map(|shl| self.matchbyte1.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .map(|shl| self.matchbyte2.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .map(|shl| self.matchbyte3.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0))
            | self.matchbyte4.disassembly());
        calc__offset = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .map(|shl| self.offsetbyte1.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .map(|shl| self.offsetbyte2.disassembly().checked_shl(shl))
                    .flatten()
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .map(|shl| self.offsetbyte3.disassembly().checked_shl(shl))
                    .flatten()
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
        let mut calc_match: i128 = 0;
        let mut calc__offset: i128 = 0;
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
        calc_match = (((u32::try_from(24i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldmatchbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldmatchbyte2()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldmatchbyte3()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
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
        calc__offset = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldoffsetbyte1()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .map(|shl| {
                        token_parser
                            .TokenFieldoffsetbyte2()
                            .disassembly()
                            .checked_shl(shl)
                    })
                    .flatten()
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .map(|shl| {
                        token_parser
                            .TokenFieldoffsetbyte3()
                            .disassembly()
                            .checked_shl(shl)
                    })
                    .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1741:1, end:1741:10))"]
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
            .read_alignmentPad_disassembly()
            .unwrap()
            != 3i128
        {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1742:1, end:1742:10))"]
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
            .read_alignmentPad_disassembly()
            .unwrap()
            != 2i128
        {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1743:1, end:1743:10))"]
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
            .read_alignmentPad_disassembly()
            .unwrap()
            != 1i128
        {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1744:1, end:1744:10))"]
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
            .read_alignmentPad_disassembly()
            .unwrap()
            != 0i128
        {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1747:1, end:1747:15))"]
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
        let mut calc_npairs: i128 = 0;
        let mut calc__default: i128 = 0;
        calc_npairs = (((u32::try_from(24i128)
            .ok()
            .map(|shl| self.npairsbyte1.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .map(|shl| self.npairsbyte2.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .map(|shl| self.npairsbyte3.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0))
            | self.npairsbyte4.disassembly());
        calc__default = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .map(|shl| self.defaultbyte1.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .map(|shl| self.defaultbyte2.disassembly().checked_shl(shl))
                    .flatten()
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .map(|shl| self.defaultbyte3.disassembly().checked_shl(shl))
                    .flatten()
                    .unwrap_or(0))
                | self.defaultbyte4.disassembly()),
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Number(true, calc__default),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        let mut calc_npairs: i128 = 0;
        let mut calc__default: i128 = 0;
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
        calc_npairs = (((u32::try_from(24i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldnpairsbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldnpairsbyte2()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldnpairsbyte3()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0))
            | token_parser.TokenFieldnpairsbyte4().disassembly());
        calc__default = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFielddefaultbyte1()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .map(|shl| {
                        token_parser
                            .TokenFielddefaultbyte2()
                            .disassembly()
                            .checked_shl(shl)
                    })
                    .flatten()
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .map(|shl| {
                        token_parser
                            .TokenFielddefaultbyte3()
                            .disassembly()
                            .checked_shl(shl)
                    })
                    .flatten()
                    .unwrap_or(0))
                | token_parser.TokenFielddefaultbyte4().disassembly()),
        );
        let tmp = calc_npairs;
        context_instance
            .register_mut()
            .write_switch_num_disassembly(tmp)
            .unwrap();
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_in_lookup_switch_disassembly(tmp)
            .unwrap();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2016:1, end:2016:14))"]
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
        let mut calc__offset: i128 = 0;
        calc__offset = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .map(|shl| self.offsetbyte1.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .map(|shl| self.offsetbyte2.disassembly().checked_shl(shl))
                    .flatten()
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .map(|shl| self.offsetbyte3.disassembly().checked_shl(shl))
                    .flatten()
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
        let mut calc__offset: i128 = 0;
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
        calc__offset = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldoffsetbyte1()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .map(|shl| {
                        token_parser
                            .TokenFieldoffsetbyte2()
                            .disassembly()
                            .checked_shl(shl)
                    })
                    .flatten()
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .map(|shl| {
                        token_parser
                            .TokenFieldoffsetbyte3()
                            .disassembly()
                            .checked_shl(shl)
                    })
                    .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2032:1, end:2032:14))"]
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
        let mut calc_low: i128 = 0;
        let mut calc_high: i128 = 0;
        calc_low = (((u32::try_from(24i128)
            .ok()
            .map(|shl| self.lowbyte1.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .map(|shl| self.lowbyte2.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .map(|shl| self.lowbyte3.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0))
            | self.lowbyte4.disassembly());
        calc_high = (((u32::try_from(24i128)
            .ok()
            .map(|shl| self.highbyte1.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .map(|shl| self.highbyte2.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .map(|shl| self.highbyte3.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0))
            | self.highbyte4.disassembly());
        self.Default.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_low),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        let mut calc_low: i128 = 0;
        let mut calc_high: i128 = 0;
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
        calc_low = (((u32::try_from(24i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldlowbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldlowbyte2()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldlowbyte3()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
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
        calc_high = (((u32::try_from(24i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldhighbyte1()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldhighbyte2()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldhighbyte3()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0))
            | token_parser.TokenFieldhighbyte4().disassembly());
        let tmp = calc_low;
        context_instance
            .register_mut()
            .write_switch_low_disassembly(tmp)
            .unwrap();
        let tmp = calc_high.wrapping_sub(calc_low);
        context_instance
            .register_mut()
            .write_switch_num_disassembly(tmp)
            .unwrap();
        let tmp = calc_high;
        context_instance
            .register_mut()
            .write_switch_high_disassembly(tmp)
            .unwrap();
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_in_table_switch_disassembly(tmp)
            .unwrap();
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
