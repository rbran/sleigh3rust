use sleigh4rust::*;
pub type AddrType = u32;
pub trait GlobalSetTrait {
    fn set_fctx(&mut self, address: Option<u32>, value: i128);
    fn set_nfctx(&mut self, address: Option<u32>, value: i128);
    fn set_phase(&mut self, address: Option<u32>, value: i128);
    fn set_counter(&mut self, address: Option<u32>, value: i128);
}
#[derive(Default)]
pub struct GlobalSetDefault<C: ContextTrait>(
    pub std::collections::HashMap<AddrType, C>,
);
impl<C: ContextTrait> GlobalSetTrait for GlobalSetDefault<C> {
    fn set_fctx(&mut self, inst_start: Option<AddrType>, value: i128) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_fctx_disassembly(value)
                .unwrap();
            context
        });
    }
    fn set_nfctx(&mut self, inst_start: Option<AddrType>, value: i128) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_nfctx_disassembly(value)
                .unwrap();
            context
        });
    }
    fn set_phase(&mut self, inst_start: Option<AddrType>, value: i128) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_phase_disassembly(value)
                .unwrap();
            context
        });
    }
    fn set_counter(&mut self, inst_start: Option<AddrType>, value: i128) {
        let Some (inst_start) = inst_start else { return } ;
        self.0.entry(inst_start).or_insert_with(|| {
            let mut context = C::default();
            context
                .register_mut()
                .write_counter_disassembly(value)
                .unwrap();
            context
        });
    }
}
pub trait ContextregisterTrait:
    MemoryRead<AddressType = u16> + MemoryWrite
{
    fn read_fctx_raw(&self) -> Result<u8, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u8::<true>(7, 0, 4)?;
        Ok(u8::try_from(work_value).unwrap())
    }
    fn write_fctx_raw(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u8::<true>(u8::from(param), 7, 0, 4)
    }
    fn read_fctx_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_fctx_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_fctx_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_fctx_raw(param as u8)
    }
    fn read_fctx_execution(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        self.read_fctx_raw()
    }
    fn write_fctx_execution(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_fctx_raw(param)
    }
    fn fctx_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_fctx_raw()?))
    }
    fn read_nfctx_raw(&self) -> Result<u8, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u8::<true>(7, 4, 4)?;
        Ok(u8::try_from(work_value).unwrap())
    }
    fn write_nfctx_raw(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u8::<true>(u8::from(param), 7, 4, 4)
    }
    fn read_nfctx_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_nfctx_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_nfctx_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_nfctx_raw(param as u8)
    }
    fn read_nfctx_execution(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        self.read_nfctx_raw()
    }
    fn write_nfctx_execution(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_nfctx_raw(param)
    }
    fn nfctx_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_nfctx_raw()?))
    }
    fn read_phase_raw(&self) -> Result<u8, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u8::<true>(6, 0, 2)?;
        Ok(u8::try_from(work_value).unwrap())
    }
    fn write_phase_raw(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u8::<true>(u8::from(param), 6, 0, 2)
    }
    fn read_phase_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_phase_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_phase_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_phase_raw(param as u8)
    }
    fn read_phase_execution(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        self.read_phase_raw()
    }
    fn write_phase_execution(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_phase_raw(param)
    }
    fn phase_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_phase_raw()?))
    }
    fn read_counter_raw(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        let work_value = self.read_u8::<true>(6, 2, 4)?;
        Ok(u8::try_from(work_value).unwrap())
    }
    fn write_counter_raw(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_u8::<true>(u8::from(param), 6, 2, 4)
    }
    fn read_counter_disassembly(
        &self,
    ) -> Result<i128, MemoryReadError<Self::AddressType>> {
        let raw_value = self.read_counter_raw()?;
        Ok(i128::try_from(raw_value).unwrap())
    }
    fn write_counter_disassembly(
        &mut self,
        param: i128,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_counter_raw(param as u8)
    }
    fn read_counter_execution(
        &self,
    ) -> Result<u8, MemoryReadError<Self::AddressType>> {
        self.read_counter_raw()
    }
    fn write_counter_execution(
        &mut self,
        param: u8,
    ) -> Result<(), MemoryWriteError<Self::AddressType>> {
        self.write_counter_raw(param)
    }
    fn counter_display(
        &self,
    ) -> Result<DisplayElement, MemoryReadError<Self::AddressType>> {
        Ok(meaning_number(true, self.read_counter_raw()?))
    }
}
pub trait ContextTrait: Default {
    type Typeregister: ContextregisterTrait;
    fn register(&self) -> &Self::Typeregister;
    fn register_mut(&mut self) -> &mut Self::Typeregister;
}
#[derive(Debug, Clone, Copy)]
pub struct ContextregisterStructDebug {
    pub chunk_0x0: [Option<bool>; 64],
}
impl Default for ContextregisterStructDebug {
    fn default() -> Self {
        Self {
            chunk_0x0: [None; 64],
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
            (0..=7, 0..=8) => {
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
            (0..=7, 0..=8) => {
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
        0 => Register::r0,
        1 => Register::r1,
        2 => Register::r2,
        3 => Register::r3,
        4 => Register::r4,
        5 => Register::r5,
        6 => Register::r6,
        7 => Register::r7,
        8 => Register::r8,
        9 => Register::r9,
        10 => Register::r10,
        11 => Register::r11,
        12 => Register::r12,
        13 => Register::sp,
        14 => Register::lr,
        15 => Register::pc,
        _ => unreachable!("Invalid Attach Value"),
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op8(u8);
impl TokenField_op8 {
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
struct TokenField_xsimm8(i8);
impl TokenField_xsimm8 {
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
struct TokenField_nnnn(u8);
impl TokenField_nnnn {
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
struct TokenField_op1515(u8);
impl TokenField_op1515 {
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
struct TokenField_op1415(u8);
impl TokenField_op1415 {
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
struct TokenField_op1215(u8);
impl TokenField_op1215 {
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
struct TokenField_op1111(u8);
impl TokenField_op1111 {
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
struct TokenField_op0811(u8);
impl TokenField_op0811 {
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
struct TokenField_op0407(u8);
impl TokenField_op0407 {
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
struct TokenField_op0007(u8);
impl TokenField_op0007 {
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
struct TokenField_op0003(u8);
impl TokenField_op0003 {
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
struct TokenField_op0303(u8);
impl TokenField_op0303 {
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
struct TokenField_rd(u8);
impl TokenField_rd {
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
struct TokenField_rs(u8);
impl TokenField_rs {
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
struct TokenField_rt(u8);
impl TokenField_rt {
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
struct TokenField_imm1213(u8);
impl TokenField_imm1213 {
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
struct TokenField_imm0007(u8);
impl TokenField_imm0007 {
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
struct TokenField_imm0003(u8);
impl TokenField_imm0003 {
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
struct TokenField_simm1213(i8);
impl TokenField_simm1213 {
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
struct TokenField_simm0010(i16);
impl TokenField_simm0010 {
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
struct TokenField_simm0411(i8);
impl TokenField_simm0411 {
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
struct TokenField_simm0007(i8);
impl TokenField_simm0007 {
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
struct TokenField_simm0003(i8);
impl TokenField_simm0003 {
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
struct TokenField_cc0911(u8);
impl TokenField_cc0911 {
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
struct TokenField_cc0002(u8);
impl TokenField_cc0002 {
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
    fn TokenFieldop8(&self) -> TokenField_op8 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_op8(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldxsimm8(&self) -> TokenField_xsimm8 {
        let inner_value = self.read_i8::<true>(1, 0, 8).unwrap();
        TokenField_xsimm8(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldnnnn(&self) -> TokenField_nnnn {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_nnnn(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1515(&self) -> TokenField_op1515 {
        let inner_value = self.read_u8::<true>(0, 7, 1).unwrap();
        TokenField_op1515(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1415(&self) -> TokenField_op1415 {
        let inner_value = self.read_u8::<true>(0, 6, 2).unwrap();
        TokenField_op1415(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1215(&self) -> TokenField_op1215 {
        let inner_value = self.read_u8::<true>(0, 4, 4).unwrap();
        TokenField_op1215(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1111(&self) -> TokenField_op1111 {
        let inner_value = self.read_u8::<true>(0, 3, 1).unwrap();
        TokenField_op1111(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0811(&self) -> TokenField_op0811 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_op0811(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0407(&self) -> TokenField_op0407 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_op0407(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0007(&self) -> TokenField_op0007 {
        let inner_value = self.read_u8::<true>(1, 0, 8).unwrap();
        TokenField_op0007(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0003(&self) -> TokenField_op0003 {
        let inner_value = self.read_u8::<true>(1, 0, 4).unwrap();
        TokenField_op0003(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0303(&self) -> TokenField_op0303 {
        let inner_value = self.read_u8::<true>(1, 3, 1).unwrap();
        TokenField_op0303(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrd(&self) -> TokenField_rd {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_rd(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrs(&self) -> TokenField_rs {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_rs(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrt(&self) -> TokenField_rt {
        let inner_value = self.read_u8::<true>(1, 0, 4).unwrap();
        TokenField_rt(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldimm1213(&self) -> TokenField_imm1213 {
        let inner_value = self.read_u8::<true>(0, 4, 2).unwrap();
        TokenField_imm1213(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldimm0007(&self) -> TokenField_imm0007 {
        let inner_value = self.read_u8::<true>(1, 0, 8).unwrap();
        TokenField_imm0007(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldimm0003(&self) -> TokenField_imm0003 {
        let inner_value = self.read_u8::<true>(1, 0, 4).unwrap();
        TokenField_imm0003(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsimm1213(&self) -> TokenField_simm1213 {
        let inner_value = self.read_i8::<true>(0, 4, 2).unwrap();
        TokenField_simm1213(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsimm0010(&self) -> TokenField_simm0010 {
        let inner_value = self.read_i16::<true>(0, 0, 11).unwrap();
        TokenField_simm0010(i16::try_from(inner_value).unwrap())
    }
    fn TokenFieldsimm0411(&self) -> TokenField_simm0411 {
        let inner_value = self.read_i16::<true>(0, 4, 8).unwrap();
        TokenField_simm0411(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsimm0007(&self) -> TokenField_simm0007 {
        let inner_value = self.read_i8::<true>(1, 0, 8).unwrap();
        TokenField_simm0007(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsimm0003(&self) -> TokenField_simm0003 {
        let inner_value = self.read_i8::<true>(1, 0, 4).unwrap();
        TokenField_simm0003(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldcc0911(&self) -> TokenField_cc0911 {
        let inner_value = self.read_u8::<true>(0, 1, 3).unwrap();
        TokenField_cc0911(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldcc0002(&self) -> TokenField_cc0002 {
        let inner_value = self.read_u8::<true>(1, 0, 3).unwrap();
        TokenField_cc0002(u8::try_from(inner_value).unwrap())
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    r0,
    r1,
    r2,
    r3,
    r4,
    r5,
    r6,
    r7,
    r8,
    r9,
    r10,
    r11,
    r12,
    sp,
    lr,
    pc,
    C,
    Z,
    N,
    V,
    r0h,
    r0l,
    r1h,
    r1l,
    r2h,
    r2l,
    r3h,
    r3l,
    r4h,
    r4l,
    r5h,
    r5l,
    r6h,
    r6l,
    r7h,
    r7l,
    r8h,
    r8l,
    r9h,
    r9l,
    r10h,
    r10l,
    r11h,
    r11l,
    r12h,
    r12l,
    sph,
    spl,
    lrh,
    lrl,
    pch,
    pcl,
    contextreg,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::r0 => write!(f, "r0"),
            Self::r1 => write!(f, "r1"),
            Self::r2 => write!(f, "r2"),
            Self::r3 => write!(f, "r3"),
            Self::r4 => write!(f, "r4"),
            Self::r5 => write!(f, "r5"),
            Self::r6 => write!(f, "r6"),
            Self::r7 => write!(f, "r7"),
            Self::r8 => write!(f, "r8"),
            Self::r9 => write!(f, "r9"),
            Self::r10 => write!(f, "r10"),
            Self::r11 => write!(f, "r11"),
            Self::r12 => write!(f, "r12"),
            Self::sp => write!(f, "sp"),
            Self::lr => write!(f, "lr"),
            Self::pc => write!(f, "pc"),
            Self::C => write!(f, "C"),
            Self::Z => write!(f, "Z"),
            Self::N => write!(f, "N"),
            Self::V => write!(f, "V"),
            Self::r0h => write!(f, "r0h"),
            Self::r0l => write!(f, "r0l"),
            Self::r1h => write!(f, "r1h"),
            Self::r1l => write!(f, "r1l"),
            Self::r2h => write!(f, "r2h"),
            Self::r2l => write!(f, "r2l"),
            Self::r3h => write!(f, "r3h"),
            Self::r3l => write!(f, "r3l"),
            Self::r4h => write!(f, "r4h"),
            Self::r4l => write!(f, "r4l"),
            Self::r5h => write!(f, "r5h"),
            Self::r5l => write!(f, "r5l"),
            Self::r6h => write!(f, "r6h"),
            Self::r6l => write!(f, "r6l"),
            Self::r7h => write!(f, "r7h"),
            Self::r7l => write!(f, "r7l"),
            Self::r8h => write!(f, "r8h"),
            Self::r8l => write!(f, "r8l"),
            Self::r9h => write!(f, "r9h"),
            Self::r9l => write!(f, "r9l"),
            Self::r10h => write!(f, "r10h"),
            Self::r10l => write!(f, "r10l"),
            Self::r11h => write!(f, "r11h"),
            Self::r11l => write!(f, "r11l"),
            Self::r12h => write!(f, "r12h"),
            Self::r12l => write!(f, "r12l"),
            Self::sph => write!(f, "sph"),
            Self::spl => write!(f, "spl"),
            Self::lrh => write!(f, "lrh"),
            Self::lrl => write!(f, "lrl"),
            Self::pch => write!(f, "pch"),
            Self::pcl => write!(f, "pcl"),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:217:1, end:217:2))"]
#[derive(Clone, Debug)]
struct ret_instructionVar0 {}
impl ret_instructionVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ret")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldop0007().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:227:1, end:227:2))"]
#[derive(Clone, Debug)]
struct user_three_instructionVar1 {}
impl user_three_instructionVar1 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("user_three")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldop0007().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:232:1, end:232:2))"]
#[derive(Clone, Debug)]
struct unimpl_instructionVar2 {}
impl unimpl_instructionVar2 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("unimpl")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 8i128 {
            return None;
        }
        if token_parser.TokenFieldop0007().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:197:1, end:197:2))"]
#[derive(Clone, Debug)]
struct inv_instructionVar3 {
    rs: TokenField_rs,
}
impl inv_instructionVar3 {
    fn display_extend<T>(
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
            DisplayElement::Literal("inv"),
            DisplayElement::Literal(" "),
            self.rs.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 14i128 {
            return None;
        }
        if token_parser.TokenFieldop0003().disassembly() != 0i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:198:1, end:198:2))"]
#[derive(Clone, Debug)]
struct neg_instructionVar4 {
    rs: TokenField_rs,
}
impl neg_instructionVar4 {
    fn display_extend<T>(
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
            DisplayElement::Literal("neg"),
            DisplayElement::Literal(" "),
            self.rs.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 14i128 {
            return None;
        }
        if token_parser.TokenFieldop0003().disassembly() != 1i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:204:1, end:204:2))"]
#[derive(Clone, Debug)]
struct sk_instructionVar5 {
    CC: TableCC,
    Rel82: TableRel82,
}
impl sk_instructionVar5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("sk")];
        display.extend_from_slice(&extend);
        self.CC.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 8i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop0407().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop0303().disassembly() != 0i128 {
            return None;
        }
        let CC = if let Some((len, table)) =
            TableCC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Rel82 = if let Some((len, table)) =
            TableRel82::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { CC, Rel82 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:214:1, end:214:2))"]
#[derive(Clone, Debug)]
struct push_instructionVar6 {
    rs: TokenField_rs,
}
impl push_instructionVar6 {
    fn display_extend<T>(
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
            DisplayElement::Literal("push"),
            DisplayElement::Literal(" "),
            self.rs.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldop0003().disassembly() != 0i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:215:1, end:215:2))"]
#[derive(Clone, Debug)]
struct pop_instructionVar7 {
    rs: TokenField_rs,
}
impl pop_instructionVar7 {
    fn display_extend<T>(
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
            DisplayElement::Literal("pop"),
            DisplayElement::Literal(" "),
            self.rs.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldop0003().disassembly() != 0i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:219:1, end:219:2))"]
#[derive(Clone, Debug)]
struct call_instructionVar8 {
    rs: TokenField_rs,
}
impl call_instructionVar8 {
    fn display_extend<T>(
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
            DisplayElement::Literal("call"),
            DisplayElement::Literal(" "),
            self.rs.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldop0003().disassembly() != 0i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:225:1, end:225:2))"]
#[derive(Clone, Debug)]
struct user_one_instructionVar9 {
    rs: TokenField_rs,
}
impl user_one_instructionVar9 {
    fn display_extend<T>(
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
            DisplayElement::Literal("user_one"),
            DisplayElement::Literal(" "),
            self.rs.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldop0003().disassembly() != 0i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:226:1, end:226:2))"]
#[derive(Clone, Debug)]
struct user_two_instructionVar10 {
    rs: TokenField_rs,
}
impl user_two_instructionVar10 {
    fn display_extend<T>(
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
            DisplayElement::Literal("user_two"),
            DisplayElement::Literal(" "),
            self.rs.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldop0003().disassembly() != 0i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:230:1, end:230:2))"]
#[derive(Clone, Debug)]
struct user_six_instructionVar11 {
    rs: TokenField_rs,
}
impl user_six_instructionVar11 {
    fn display_extend<T>(
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
            DisplayElement::Literal("user_six"),
            DisplayElement::Literal(" "),
            self.rs.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldop0003().disassembly() != 0i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:65:1, end:65:2))"]
#[derive(Clone, Debug)]
struct cop1_instructionVar12 {
    rs: TokenField_rs,
}
impl cop1_instructionVar12 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cop1"),
            DisplayElement::Literal(" "),
            self.rs.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldop0003().disassembly() != 0i128 {
            return None;
        }
        if context_instance
            .register()
            .read_nfctx_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:66:1, end:66:2))"]
#[derive(Clone, Debug)]
struct cop2_instructionVar13 {
    rs: TokenField_rs,
}
impl cop2_instructionVar13 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cop2"),
            DisplayElement::Literal(" "),
            self.rs.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldop0003().disassembly() != 0i128 {
            return None;
        }
        if context_instance
            .register()
            .read_nfctx_disassembly()
            .unwrap()
            != 2i128
        {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:67:1, end:67:2))"]
#[derive(Clone, Debug)]
struct cop3_instructionVar14 {
    rs: TokenField_rs,
}
impl cop3_instructionVar14 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cop3"),
            DisplayElement::Literal(" "),
            self.rs.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldop0003().disassembly() != 0i128 {
            return None;
        }
        if context_instance
            .register()
            .read_nfctx_disassembly()
            .unwrap()
            != 3i128
        {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:208:1, end:208:2))"]
#[derive(Clone, Debug)]
struct br_instructionVar15 {
    rs: TokenField_rs,
    COND: TableCOND,
}
impl br_instructionVar15 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("br")];
        display.extend_from_slice(&extend);
        self.COND.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(" "), self.rs.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop0303().disassembly() != 0i128 {
            return None;
        }
        let COND = if let Some((len, table)) =
            TableCOND::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { COND, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:209:1, end:209:2))"]
#[derive(Clone, Debug)]
struct brds_instructionVar16 {
    rs: TokenField_rs,
    COND: TableCOND,
}
impl brds_instructionVar16 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("brds")];
        display.extend_from_slice(&extend);
        self.COND.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(" "), self.rs.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldop0303().disassembly() != 0i128 {
            return None;
        }
        let COND = if let Some((len, table)) =
            TableCOND::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { COND, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:223:1, end:223:2))"]
#[derive(Clone, Debug)]
struct call_instructionVar17 {
    rs: TokenField_rs,
    COND: TableCOND,
}
impl call_instructionVar17 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("call")];
        display.extend_from_slice(&extend);
        self.COND.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(" "), self.rs.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldop0303().disassembly() != 1i128 {
            return None;
        }
        let COND = if let Some((len, table)) =
            TableCOND::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { COND, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:54:1, end:54:2))"]
#[derive(Clone, Debug)]
struct fctx_instructionVar18 {
    imm0003: TokenField_imm0003,
    Imm4: TableImm4,
}
impl fctx_instructionVar18 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_fctx(
            Some(inst_next),
            context.register().read_fctx_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fctx"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm4.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldrs().disassembly() != 0i128 {
            return None;
        }
        let tmp = token_parser.TokenFieldimm0003().disassembly();
        context_instance
            .register_mut()
            .write_fctx_disassembly(tmp)
            .unwrap();
        let Imm4 = if let Some((len, table)) =
            TableImm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let imm0003 = token_parser.TokenFieldimm0003();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm4, imm0003 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:58:1, end:58:2))"]
#[derive(Clone, Debug)]
struct nfctx_instructionVar19 {
    Imm4: TableImm4,
    nfctxSetAddr: TablenfctxSetAddr,
}
impl nfctx_instructionVar19 {
    fn display_extend<T>(
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
            DisplayElement::Literal("nfctx"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.nfctxSetAddr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Imm4.display_extend(
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if context_instance
                .register()
                .read_phase_disassembly()
                .unwrap()
                != 1i128
            {
                return None;
            }
            if token_parser.TokenFieldop1215().disassembly() != 13i128 {
                return None;
            }
            if token_parser.TokenFieldop0811().disassembly() != 9i128 {
                return None;
            }
            if token_parser.TokenFieldrs().disassembly() != 2i128 {
                return None;
            }
            let Imm4 = if let Some((len, table)) =
                TableImm4::parse(tokens, &mut context_instance, inst_start)
            {
                block_0_len = block_0_len.max(len as u32);
                table
            } else {
                return None;
            };
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((Imm4), (), pattern_len))
        };
        let ((mut Imm4), (), sub_len) =
            sub_pattern_c29(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let nfctxSetAddr = if let Some((len, table)) = TablenfctxSetAddr::parse(
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
        *context = context_instance;
        Some((pattern_len, Self { Imm4, nfctxSetAddr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:59:1, end:59:2))"]
#[derive(Clone, Debug)]
struct nfctx_instructionVar20 {
    imm0003: TokenField_imm0003,
    Imm4: TableImm4,
}
impl nfctx_instructionVar20 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        global_set.set_nfctx(
            Some(inst_next),
            context.register().read_nfctx_disassembly().unwrap(),
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("nfctx"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm4.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldrs().disassembly() != 1i128 {
            return None;
        }
        let tmp = token_parser.TokenFieldimm0003().disassembly();
        context_instance
            .register_mut()
            .write_nfctx_disassembly(tmp)
            .unwrap();
        let Imm4 = if let Some((len, table)) =
            TableImm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let imm0003 = token_parser.TokenFieldimm0003();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm4, imm0003 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:77:1, end:77:2))"]
#[derive(Clone, Debug)]
struct nop_instructionVar21 {
    imm0003: TokenField_imm0003,
    NopCnt: TableNopCnt,
    NopByte: TableNopByte,
}
impl nop_instructionVar21 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("nop"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.NopCnt.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldrs().disassembly() != 3i128 {
            return None;
        }
        let tmp = token_parser.TokenFieldimm0003().disassembly();
        context_instance
            .register_mut()
            .write_counter_disassembly(tmp)
            .unwrap();
        let NopCnt = if let Some((len, table)) = TableNopCnt::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let imm0003 = token_parser.TokenFieldimm0003();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let NopByte = if let Some((len, table)) = TableNopByte::parse(
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
        Some((
            pattern_len,
            Self {
                NopCnt,
                NopByte,
                imm0003,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:158:1, end:158:2))"]
#[derive(Clone, Debug)]
struct add_instructionVar22 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl add_instructionVar22 {
    fn display_extend<T>(
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
            DisplayElement::Literal("add"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 0i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:159:1, end:159:2))"]
#[derive(Clone, Debug)]
struct sub_instructionVar23 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl sub_instructionVar23 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sub"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 1i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:160:1, end:160:2))"]
#[derive(Clone, Debug)]
struct rsub_instructionVar24 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl rsub_instructionVar24 {
    fn display_extend<T>(
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
            DisplayElement::Literal("rsub"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 2i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:161:1, end:161:2))"]
#[derive(Clone, Debug)]
struct mul_instructionVar25 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl mul_instructionVar25 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mul"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 3i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:162:1, end:162:2))"]
#[derive(Clone, Debug)]
struct div_instructionVar26 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl div_instructionVar26 {
    fn display_extend<T>(
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
            DisplayElement::Literal("div"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 4i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:163:1, end:163:2))"]
#[derive(Clone, Debug)]
struct mod_instructionVar27 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl mod_instructionVar27 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mod"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 5i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:164:1, end:164:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar28 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl cmp_instructionVar28 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 6i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:165:1, end:165:2))"]
#[derive(Clone, Debug)]
struct ucmp_instructionVar29 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl ucmp_instructionVar29 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ucmp"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 7i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:167:1, end:167:2))"]
#[derive(Clone, Debug)]
struct add_instructionVar30 {
    rs: TokenField_rs,
    Simm4: TableSimm4,
}
impl add_instructionVar30 {
    fn display_extend<T>(
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
            DisplayElement::Literal("add"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm4.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 8i128 {
            return None;
        }
        let Simm4 = if let Some((len, table)) =
            TableSimm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:168:1, end:168:2))"]
#[derive(Clone, Debug)]
struct sub_instructionVar31 {
    rs: TokenField_rs,
    Simm4: TableSimm4,
}
impl sub_instructionVar31 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sub"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm4.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 9i128 {
            return None;
        }
        let Simm4 = if let Some((len, table)) =
            TableSimm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:169:1, end:169:2))"]
#[derive(Clone, Debug)]
struct rsub_instructionVar32 {
    rs: TokenField_rs,
    Simm4: TableSimm4,
}
impl rsub_instructionVar32 {
    fn display_extend<T>(
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
            DisplayElement::Literal("rsub"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm4.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 10i128 {
            return None;
        }
        let Simm4 = if let Some((len, table)) =
            TableSimm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:170:1, end:170:2))"]
#[derive(Clone, Debug)]
struct mul_instructionVar33 {
    rs: TokenField_rs,
    Simm4: TableSimm4,
}
impl mul_instructionVar33 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mul"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm4.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 11i128 {
            return None;
        }
        let Simm4 = if let Some((len, table)) =
            TableSimm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:171:1, end:171:2))"]
#[derive(Clone, Debug)]
struct div_instructionVar34 {
    rs: TokenField_rs,
    Simm4: TableSimm4,
}
impl div_instructionVar34 {
    fn display_extend<T>(
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
            DisplayElement::Literal("div"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm4.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 12i128 {
            return None;
        }
        let Simm4 = if let Some((len, table)) =
            TableSimm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:172:1, end:172:2))"]
#[derive(Clone, Debug)]
struct mod_instructionVar35 {
    rs: TokenField_rs,
    Imm4: TableImm4,
}
impl mod_instructionVar35 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mod"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm4.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 13i128 {
            return None;
        }
        let Imm4 = if let Some((len, table)) =
            TableImm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:173:1, end:173:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar36 {
    rs: TokenField_rs,
    Simm4: TableSimm4,
}
impl cmp_instructionVar36 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm4.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 14i128 {
            return None;
        }
        let Simm4 = if let Some((len, table)) =
            TableSimm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:174:1, end:174:2))"]
#[derive(Clone, Debug)]
struct ucmp_instructionVar37 {
    rs: TokenField_rs,
    Imm4: TableImm4,
}
impl ucmp_instructionVar37 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ucmp"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm4.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 15i128 {
            return None;
        }
        let Imm4 = if let Some((len, table)) =
            TableImm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:176:1, end:176:2))"]
#[derive(Clone, Debug)]
struct and_instructionVar38 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl and_instructionVar38 {
    fn display_extend<T>(
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
            DisplayElement::Literal("and"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 0i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:177:1, end:177:2))"]
#[derive(Clone, Debug)]
struct or_instructionVar39 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl or_instructionVar39 {
    fn display_extend<T>(
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
            DisplayElement::Literal("or"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 1i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:178:1, end:178:2))"]
#[derive(Clone, Debug)]
struct xor_instructionVar40 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl xor_instructionVar40 {
    fn display_extend<T>(
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
            DisplayElement::Literal("xor"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 2i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:179:1, end:179:2))"]
#[derive(Clone, Debug)]
struct lsr_instructionVar41 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl lsr_instructionVar41 {
    fn display_extend<T>(
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
            DisplayElement::Literal("lsr"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 3i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:180:1, end:180:2))"]
#[derive(Clone, Debug)]
struct asr_instructionVar42 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl asr_instructionVar42 {
    fn display_extend<T>(
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
            DisplayElement::Literal("asr"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 4i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:181:1, end:181:2))"]
#[derive(Clone, Debug)]
struct lsl_instructionVar43 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl lsl_instructionVar43 {
    fn display_extend<T>(
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
            DisplayElement::Literal("lsl"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 5i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:192:1, end:192:2))"]
#[derive(Clone, Debug)]
struct saa_instructionVar44 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl saa_instructionVar44 {
    fn display_extend<T>(
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
            DisplayElement::Literal("saa"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 8i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:194:1, end:194:2))"]
#[derive(Clone, Debug)]
struct lsr_instructionVar45 {
    rs: TokenField_rs,
    Imm4: TableImm4,
}
impl lsr_instructionVar45 {
    fn display_extend<T>(
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
            DisplayElement::Literal("lsr"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm4.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 11i128 {
            return None;
        }
        let Imm4 = if let Some((len, table)) =
            TableImm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:195:1, end:195:2))"]
#[derive(Clone, Debug)]
struct asr_instructionVar46 {
    rs: TokenField_rs,
    Imm4: TableImm4,
}
impl asr_instructionVar46 {
    fn display_extend<T>(
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
            DisplayElement::Literal("asr"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm4.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 12i128 {
            return None;
        }
        let Imm4 = if let Some((len, table)) =
            TableImm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:196:1, end:196:2))"]
#[derive(Clone, Debug)]
struct lsl_instructionVar47 {
    rs: TokenField_rs,
    Imm4: TableImm4,
}
impl lsl_instructionVar47 {
    fn display_extend<T>(
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
            DisplayElement::Literal("lsl"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm4.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 13i128 {
            return None;
        }
        let Imm4 = if let Some((len, table)) =
            TableImm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:200:1, end:200:2))"]
#[derive(Clone, Debug)]
struct load_instructionVar48 {
    rs: TokenField_rs,
    RT: TableRT,
}
impl load_instructionVar48 {
    fn display_extend<T>(
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
            DisplayElement::Literal("load"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.RT.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 6i128 {
            return None;
        }
        let RT = if let Some((len, table)) =
            TableRT::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RT, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:201:1, end:201:2))"]
#[derive(Clone, Debug)]
struct store_instructionVar49 {
    rt: TokenField_rt,
    RS: TableRS,
}
impl store_instructionVar49 {
    fn display_extend<T>(
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
            DisplayElement::Literal("store"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.RS.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 7i128 {
            return None;
        }
        let RS = if let Some((len, table)) =
            TableRS::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RS, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:202:1, end:202:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar50 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl mov_instructionVar50 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mov"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 15i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:206:1, end:206:2))"]
#[derive(Clone, Debug)]
struct br_instructionVar51 {
    COND: TableCOND,
    Rel82: TableRel82,
}
impl br_instructionVar51 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("br")];
        display.extend_from_slice(&extend);
        self.COND.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Rel82.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 14i128 {
            return None;
        }
        if token_parser.TokenFieldop0303().disassembly() != 0i128 {
            return None;
        }
        let COND = if let Some((len, table)) =
            TableCOND::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Rel82 = if let Some((len, table)) =
            TableRel82::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { COND, Rel82 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:207:1, end:207:2))"]
#[derive(Clone, Debug)]
struct brds_instructionVar52 {
    COND: TableCOND,
    Rel82: TableRel82,
}
impl brds_instructionVar52 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("brds")];
        display.extend_from_slice(&extend);
        self.COND.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Rel82.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 14i128 {
            return None;
        }
        if token_parser.TokenFieldop0303().disassembly() != 1i128 {
            return None;
        }
        let COND = if let Some((len, table)) =
            TableCOND::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Rel82 = if let Some((len, table)) =
            TableRel82::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { COND, Rel82 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:218:1, end:218:2))"]
#[derive(Clone, Debug)]
struct callds_instructionVar53 {
    Rel8: TableRel8,
}
impl callds_instructionVar53 {
    fn display_extend<T>(
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
            DisplayElement::Literal("callds"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 5i128 {
            return None;
        }
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:228:1, end:228:2))"]
#[derive(Clone, Debug)]
struct user_four_instructionVar54 {
    rs: TokenField_rs,
    rt: TokenField_rt,
}
impl user_four_instructionVar54 {
    fn display_extend<T>(
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
            DisplayElement::Literal("user_four"),
            DisplayElement::Literal(" "),
            self.rs.display(),
            DisplayElement::Literal(" "),
            self.rt.display(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 4i128 {
            return None;
        }
        let rs = token_parser.TokenFieldrs();
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:229:1, end:229:2))"]
#[derive(Clone, Debug)]
struct user_five_instructionVar55 {
    Rel8: TableRel8,
}
impl user_five_instructionVar55 {
    fn display_extend<T>(
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
            DisplayElement::Literal("user_five"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldop0811().disassembly() != 5i128 {
            return None;
        }
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:220:1, end:220:2))"]
#[derive(Clone, Debug)]
struct call_instructionVar56 {
    Rel11: TableRel11,
}
impl call_instructionVar56 {
    fn display_extend<T>(
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
            DisplayElement::Literal("call"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Rel11.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1215().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldop1111().disassembly() != 1i128 {
            return None;
        }
        let Rel11 = if let Some((len, table)) =
            TableRel11::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:156:1, end:156:2))"]
#[derive(Clone, Debug)]
struct simm_instructionVar57 {
    rd: TokenField_rd,
    Simm10: TableSimm10,
}
impl simm_instructionVar57 {
    fn display_extend<T>(
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
            DisplayElement::Literal("simm"),
            DisplayElement::Literal(" "),
            self.rd.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm10.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1415().disassembly() != 2i128 {
            return None;
        }
        let Simm10 = if let Some((len, table)) = TableSimm10::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rd = token_parser.TokenFieldrd();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm10, rd }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:155:1, end:155:2))"]
#[derive(Clone, Debug)]
struct imm_instructionVar58 {
    rd: TokenField_rd,
    Imm10: TableImm10,
}
impl imm_instructionVar58 {
    fn display_extend<T>(
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
            DisplayElement::Literal("imm"),
            DisplayElement::Literal(" "),
            self.rd.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm10.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop1515().disassembly() != 0i128 {
            return None;
        }
        let Imm10 = if let Some((len, table)) =
            TableImm10::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rd = token_parser.TokenFieldrd();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm10, rd }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:76:1, end:76:2))"]
#[derive(Clone, Debug)]
struct nop_instructionVar59 {
    One: TableOne,
}
impl nop_instructionVar59 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("nop"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.One.display_extend(
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
            .read_phase_disassembly()
            .unwrap()
            != 1i128
        {
            return None;
        }
        if token_parser.TokenFieldop8().disassembly() != 247i128 {
            return None;
        }
        let One = if let Some((len, table)) =
            TableOne::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { One }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:31:1, end:31:2))"]
#[derive(Clone, Debug)]
struct instructionVar60 {
    instruction: Box<Tableinstruction>,
}
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
        if context_instance
            .register()
            .read_phase_disassembly()
            .unwrap()
            != 0i128
        {
            return None;
        }
        let tmp = 1i128;
        context_instance
            .register_mut()
            .write_phase_disassembly(tmp)
            .unwrap();
        let instruction = if let Some((len, table)) = Tableinstruction::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
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
    Var0(ret_instructionVar0),
    Var1(user_three_instructionVar1),
    Var2(unimpl_instructionVar2),
    Var3(inv_instructionVar3),
    Var4(neg_instructionVar4),
    Var5(sk_instructionVar5),
    Var6(push_instructionVar6),
    Var7(pop_instructionVar7),
    Var8(call_instructionVar8),
    Var9(user_one_instructionVar9),
    Var10(user_two_instructionVar10),
    Var11(user_six_instructionVar11),
    Var12(cop1_instructionVar12),
    Var13(cop2_instructionVar13),
    Var14(cop3_instructionVar14),
    Var15(br_instructionVar15),
    Var16(brds_instructionVar16),
    Var17(call_instructionVar17),
    Var18(fctx_instructionVar18),
    Var19(nfctx_instructionVar19),
    Var20(nfctx_instructionVar20),
    Var21(nop_instructionVar21),
    Var22(add_instructionVar22),
    Var23(sub_instructionVar23),
    Var24(rsub_instructionVar24),
    Var25(mul_instructionVar25),
    Var26(div_instructionVar26),
    Var27(mod_instructionVar27),
    Var28(cmp_instructionVar28),
    Var29(ucmp_instructionVar29),
    Var30(add_instructionVar30),
    Var31(sub_instructionVar31),
    Var32(rsub_instructionVar32),
    Var33(mul_instructionVar33),
    Var34(div_instructionVar34),
    Var35(mod_instructionVar35),
    Var36(cmp_instructionVar36),
    Var37(ucmp_instructionVar37),
    Var38(and_instructionVar38),
    Var39(or_instructionVar39),
    Var40(xor_instructionVar40),
    Var41(lsr_instructionVar41),
    Var42(asr_instructionVar42),
    Var43(lsl_instructionVar43),
    Var44(saa_instructionVar44),
    Var45(lsr_instructionVar45),
    Var46(asr_instructionVar46),
    Var47(lsl_instructionVar47),
    Var48(load_instructionVar48),
    Var49(store_instructionVar49),
    Var50(mov_instructionVar50),
    Var51(br_instructionVar51),
    Var52(brds_instructionVar52),
    Var53(callds_instructionVar53),
    Var54(user_four_instructionVar54),
    Var55(user_five_instructionVar55),
    Var56(call_instructionVar56),
    Var57(simm_instructionVar57),
    Var58(imm_instructionVar58),
    Var59(nop_instructionVar59),
    Var60(instructionVar60),
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
        if let Some((inst_len, parsed)) = ret_instructionVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = user_three_instructionVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = unimpl_instructionVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = inv_instructionVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = neg_instructionVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) = sk_instructionVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) = push_instructionVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) = pop_instructionVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) = call_instructionVar8::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) = user_one_instructionVar9::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) = user_two_instructionVar10::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) = user_six_instructionVar11::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) = cop1_instructionVar12::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) = cop2_instructionVar13::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) = cop3_instructionVar14::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) = br_instructionVar15::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        if let Some((inst_len, parsed)) = brds_instructionVar16::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var16(parsed)));
        }
        if let Some((inst_len, parsed)) = call_instructionVar17::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var17(parsed)));
        }
        if let Some((inst_len, parsed)) = fctx_instructionVar18::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var18(parsed)));
        }
        if let Some((inst_len, parsed)) = nfctx_instructionVar19::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var19(parsed)));
        }
        if let Some((inst_len, parsed)) = nfctx_instructionVar20::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var20(parsed)));
        }
        if let Some((inst_len, parsed)) = nop_instructionVar21::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var21(parsed)));
        }
        if let Some((inst_len, parsed)) = add_instructionVar22::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var22(parsed)));
        }
        if let Some((inst_len, parsed)) = sub_instructionVar23::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var23(parsed)));
        }
        if let Some((inst_len, parsed)) = rsub_instructionVar24::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var24(parsed)));
        }
        if let Some((inst_len, parsed)) = mul_instructionVar25::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var25(parsed)));
        }
        if let Some((inst_len, parsed)) = div_instructionVar26::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var26(parsed)));
        }
        if let Some((inst_len, parsed)) = mod_instructionVar27::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var27(parsed)));
        }
        if let Some((inst_len, parsed)) = cmp_instructionVar28::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var28(parsed)));
        }
        if let Some((inst_len, parsed)) = ucmp_instructionVar29::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var29(parsed)));
        }
        if let Some((inst_len, parsed)) = add_instructionVar30::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var30(parsed)));
        }
        if let Some((inst_len, parsed)) = sub_instructionVar31::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var31(parsed)));
        }
        if let Some((inst_len, parsed)) = rsub_instructionVar32::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var32(parsed)));
        }
        if let Some((inst_len, parsed)) = mul_instructionVar33::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var33(parsed)));
        }
        if let Some((inst_len, parsed)) = div_instructionVar34::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var34(parsed)));
        }
        if let Some((inst_len, parsed)) = mod_instructionVar35::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var35(parsed)));
        }
        if let Some((inst_len, parsed)) = cmp_instructionVar36::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var36(parsed)));
        }
        if let Some((inst_len, parsed)) = ucmp_instructionVar37::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var37(parsed)));
        }
        if let Some((inst_len, parsed)) = and_instructionVar38::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var38(parsed)));
        }
        if let Some((inst_len, parsed)) = or_instructionVar39::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var39(parsed)));
        }
        if let Some((inst_len, parsed)) = xor_instructionVar40::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var40(parsed)));
        }
        if let Some((inst_len, parsed)) = lsr_instructionVar41::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var41(parsed)));
        }
        if let Some((inst_len, parsed)) = asr_instructionVar42::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var42(parsed)));
        }
        if let Some((inst_len, parsed)) = lsl_instructionVar43::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var43(parsed)));
        }
        if let Some((inst_len, parsed)) = saa_instructionVar44::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var44(parsed)));
        }
        if let Some((inst_len, parsed)) = lsr_instructionVar45::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var45(parsed)));
        }
        if let Some((inst_len, parsed)) = asr_instructionVar46::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var46(parsed)));
        }
        if let Some((inst_len, parsed)) = lsl_instructionVar47::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var47(parsed)));
        }
        if let Some((inst_len, parsed)) = load_instructionVar48::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var48(parsed)));
        }
        if let Some((inst_len, parsed)) = store_instructionVar49::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var49(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_instructionVar50::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var50(parsed)));
        }
        if let Some((inst_len, parsed)) = br_instructionVar51::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var51(parsed)));
        }
        if let Some((inst_len, parsed)) = brds_instructionVar52::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var52(parsed)));
        }
        if let Some((inst_len, parsed)) = callds_instructionVar53::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var53(parsed)));
        }
        if let Some((inst_len, parsed)) = user_four_instructionVar54::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var54(parsed)));
        }
        if let Some((inst_len, parsed)) = user_five_instructionVar55::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var55(parsed)));
        }
        if let Some((inst_len, parsed)) = call_instructionVar56::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var56(parsed)));
        }
        if let Some((inst_len, parsed)) = simm_instructionVar57::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var57(parsed)));
        }
        if let Some((inst_len, parsed)) = imm_instructionVar58::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var58(parsed)));
        }
        if let Some((inst_len, parsed)) = nop_instructionVar59::parse(
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
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:107:1, end:107:6))"]
#[derive(Clone, Debug)]
struct Simm4Var0 {
    simm0003: TokenField_simm0003,
}
impl Simm4Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("#"), self.simm0003.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let simm0003 = token_parser.TokenFieldsimm0003();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm0003 }))
    }
}
#[derive(Clone, Debug)]
enum TableSimm4 {
    Var0(Simm4Var0),
}
impl TableSimm4 {
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
            Simm4Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:108:1, end:108:7))"]
#[derive(Clone, Debug)]
struct Simm10Var0 {
    simm1213: TokenField_simm1213,
    imm0007: TokenField_imm0007,
}
impl Simm10Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_computed: i128 = 0;
        calc_computed = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.simm1213.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.imm0007.disassembly());
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Number(true, calc_computed),
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
        let mut calc_computed: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_computed = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldsimm1213()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldimm0007().disassembly());
        let simm1213 = token_parser.TokenFieldsimm1213();
        let imm0007 = token_parser.TokenFieldimm0007();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm1213, imm0007 }))
    }
}
#[derive(Clone, Debug)]
enum TableSimm10 {
    Var0(Simm10Var0),
}
impl TableSimm10 {
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
            Simm10Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:110:1, end:110:5))"]
#[derive(Clone, Debug)]
struct Imm4Var0 {
    imm0003: TokenField_imm0003,
}
impl Imm4Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("#"), self.imm0003.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm0003 = token_parser.TokenFieldimm0003();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm0003 }))
    }
}
#[derive(Clone, Debug)]
enum TableImm4 {
    Var0(Imm4Var0),
}
impl TableImm4 {
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
            Imm4Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:111:1, end:111:6))"]
#[derive(Clone, Debug)]
struct Imm10Var0 {
    imm1213: TokenField_imm1213,
    imm0007: TokenField_imm0007,
}
impl Imm10Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_computed: i128 = 0;
        calc_computed = (u32::try_from(8i128)
            .ok()
            .map(|shl| self.imm1213.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.imm0007.disassembly());
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Number(true, calc_computed),
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
        let mut calc_computed: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_computed = (u32::try_from(8i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldimm1213()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldimm0007().disassembly());
        let imm1213 = token_parser.TokenFieldimm1213();
        let imm0007 = token_parser.TokenFieldimm0007();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm1213, imm0007 }))
    }
}
#[derive(Clone, Debug)]
enum TableImm10 {
    Var0(Imm10Var0),
}
impl TableImm10 {
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
            Imm10Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:113:1, end:113:5))"]
#[derive(Clone, Debug)]
struct Rel8Var0 {
    simm0007: TokenField_simm0007,
}
impl Rel8Var0 {
    fn display_extend<T>(
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
        calc_addr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(self.simm0007.disassembly());
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_addr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(token_parser.TokenFieldsimm0007().disassembly());
        let simm0007 = token_parser.TokenFieldsimm0007();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm0007 }))
    }
}
#[derive(Clone, Debug)]
enum TableRel8 {
    Var0(Rel8Var0),
}
impl TableRel8 {
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
            Rel8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:114:1, end:114:6))"]
#[derive(Clone, Debug)]
struct Rel82Var0 {
    simm0411: TokenField_simm0411,
}
impl Rel82Var0 {
    fn display_extend<T>(
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
        calc_addr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(self.simm0411.disassembly());
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_addr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(token_parser.TokenFieldsimm0411().disassembly());
        let simm0411 = token_parser.TokenFieldsimm0411();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm0411 }))
    }
}
#[derive(Clone, Debug)]
enum TableRel82 {
    Var0(Rel82Var0),
}
impl TableRel82 {
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
            Rel82Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:115:1, end:115:6))"]
#[derive(Clone, Debug)]
struct Rel11Var0 {
    simm0010: TokenField_simm0010,
}
impl Rel11Var0 {
    fn display_extend<T>(
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
        calc_addr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(self.simm0010.disassembly());
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_addr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(token_parser.TokenFieldsimm0010().disassembly());
        let simm0010 = token_parser.TokenFieldsimm0010();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm0010 }))
    }
}
#[derive(Clone, Debug)]
enum TableRel11 {
    Var0(Rel11Var0),
}
impl TableRel11 {
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
            Rel11Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:117:1, end:117:3))"]
#[derive(Clone, Debug)]
struct RSVar0 {
    rs: TokenField_rs,
}
impl RSVar0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("["),
            self.rs.display(),
            DisplayElement::Literal("]"),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let rs = token_parser.TokenFieldrs();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[derive(Clone, Debug)]
enum TableRS {
    Var0(RSVar0),
}
impl TableRS {
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
            RSVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:118:1, end:118:3))"]
#[derive(Clone, Debug)]
struct RTVar0 {
    rt: TokenField_rt,
}
impl RTVar0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("["),
            self.rt.display(),
            DisplayElement::Literal("]"),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let rt = token_parser.TokenFieldrt();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rt }))
    }
}
#[derive(Clone, Debug)]
enum TableRT {
    Var0(RTVar0),
}
impl TableRT {
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
            RTVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:120:1, end:120:3))"]
#[derive(Clone, Debug)]
struct CCVar0 {}
impl CCVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("eq")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcc0002().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:121:1, end:121:3))"]
#[derive(Clone, Debug)]
struct CCVar1 {}
impl CCVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ne")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcc0002().disassembly() != 1i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:122:1, end:122:3))"]
#[derive(Clone, Debug)]
struct CCVar2 {}
impl CCVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("lt")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcc0002().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:123:1, end:123:3))"]
#[derive(Clone, Debug)]
struct CCVar3 {}
impl CCVar3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("le")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcc0002().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:124:1, end:124:3))"]
#[derive(Clone, Debug)]
struct CCVar4 {}
impl CCVar4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("lo")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcc0002().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:125:1, end:125:3))"]
#[derive(Clone, Debug)]
struct CCVar5 {}
impl CCVar5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("mi")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcc0002().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:126:1, end:126:3))"]
#[derive(Clone, Debug)]
struct CCVar6 {}
impl CCVar6 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("vs")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcc0002().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:127:1, end:127:3))"]
#[derive(Clone, Debug)]
struct CCVar7 {}
impl CCVar7 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcc0002().disassembly() != 7i128 {
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
enum TableCC {
    Var0(CCVar0),
    Var1(CCVar1),
    Var2(CCVar2),
    Var3(CCVar3),
    Var4(CCVar4),
    Var5(CCVar5),
    Var6(CCVar6),
    Var7(CCVar7),
}
impl TableCC {
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
            CCVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            CCVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            CCVar2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            CCVar3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) =
            CCVar4::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) =
            CCVar5::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) =
            CCVar6::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) =
            CCVar7::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:130:1, end:130:5))"]
#[derive(Clone, Debug)]
struct CONDVar0 {
    CC: TableCC,
}
impl CONDVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.CC.display_extend(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldcc0002().disassembly() != 7i128 {
            return None;
        }
        let CC = if let Some((len, table)) =
            TableCC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { CC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:129:1, end:129:5))"]
#[derive(Clone, Debug)]
struct CONDVar1 {
    CC: TableCC,
}
impl CONDVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.CC.display_extend(
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
        let CC = if let Some((len, table)) =
            TableCC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { CC }))
    }
}
#[derive(Clone, Debug)]
enum TableCOND {
    Var0(CONDVar0),
    Var1(CONDVar1),
}
impl TableCOND {
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
            CONDVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            CONDVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:56:1, end:56:13))"]
#[derive(Clone, Debug)]
struct nfctxSetAddrVar0 {
    xsimm8: TokenField_xsimm8,
    imm0003: TokenField_imm0003,
    Imm4: TableImm4,
}
impl nfctxSetAddrVar0 {
    fn display_extend<T>(
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
        calc_addr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(self.xsimm8.disassembly());
        global_set.set_nfctx(
            Some(u32::try_from(calc_addr).unwrap()),
            context.register().read_nfctx_disassembly().unwrap(),
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let Imm4 = if let Some((len, table)) =
            TableImm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let imm0003 = token_parser.TokenFieldimm0003();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_addr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(token_parser.TokenFieldxsimm8().disassembly());
        let tmp = token_parser.TokenFieldimm0003().disassembly();
        context_instance
            .register_mut()
            .write_nfctx_disassembly(tmp)
            .unwrap();
        let xsimm8 = token_parser.TokenFieldxsimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                Imm4,
                imm0003,
                xsimm8,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TablenfctxSetAddr {
    Var0(nfctxSetAddrVar0),
}
impl TablenfctxSetAddr {
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
        if let Some((inst_len, parsed)) = nfctxSetAddrVar0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:69:1, end:69:7))"]
#[derive(Clone, Debug)]
struct NopCntVar0 {
    imm0003: TokenField_imm0003,
}
impl NopCntVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_cnt: i128 = 0;
        calc_cnt = self.imm0003.disassembly().wrapping_add(2i128);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Number(true, calc_cnt),
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
        let mut calc_cnt: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_cnt = token_parser
            .TokenFieldimm0003()
            .disassembly()
            .wrapping_add(2i128);
        let imm0003 = token_parser.TokenFieldimm0003();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm0003 }))
    }
}
#[derive(Clone, Debug)]
enum TableNopCnt {
    Var0(NopCntVar0),
}
impl TableNopCnt {
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
            NopCntVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:71:1, end:71:8))"]
#[derive(Clone, Debug)]
struct NopByteVar0 {}
impl NopByteVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 0u64 as u32;
        if context_instance
            .register()
            .read_counter_disassembly()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:72:1, end:72:8))"]
#[derive(Clone, Debug)]
struct NopByteVar1 {
    NopByte: Box<TableNopByte>,
}
impl NopByteVar1 {
    fn display_extend<T>(
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
        let mut block_0_len = 0u64 as u32;
        let tmp = context_instance
            .register()
            .read_counter_disassembly()
            .unwrap()
            .wrapping_sub(1i128);
        context_instance
            .register_mut()
            .write_counter_disassembly(tmp)
            .unwrap();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let nnnn = token_parser.TokenFieldnnnn();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let NopByte = if let Some((len, table)) = TableNopByte::parse(
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
        Some((pattern_len, Self { NopByte }))
    }
}
#[derive(Clone, Debug)]
enum TableNopByte {
    Var0(NopByteVar0),
    Var1(NopByteVar1),
}
impl TableNopByte {
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
            NopByteVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            NopByteVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toy_builder.sinc, start:74:1, end:74:4))"]
#[derive(Clone, Debug)]
struct OneVar0 {}
impl OneVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_cnt: i128 = 0;
        calc_cnt = 1i128;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Number(true, calc_cnt),
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
        let mut calc_cnt: i128 = 0;
        let mut block_0_len = 0u64 as u32;
        calc_cnt = 1i128;
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableOne {
    Var0(OneVar0),
}
impl TableOne {
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
            OneVar0::parse(tokens_param, &mut context_current, inst_start)
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
