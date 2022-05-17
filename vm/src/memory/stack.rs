use thiserror::Error;

use crate::{settings::VmSettings, state::VmState};

use super::{VirtualMemoryAddress, MemoryArea};

#[derive(Error, Debug)]
pub enum StackError {
    #[error("the size limit of the stack limit was reached")]
    SizeLimitReached,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryStack {
    pub inner: Vec<u64>,
    pub size_limit: usize,
    pub start: Option<VirtualMemoryAddress>,
    pub end: Option<VirtualMemoryAddress>, 
}

impl MemoryStack {
    pub fn new(settings: &VmSettings) -> Self {
        let inner = Vec::new();
        let size_limit = settings.stack_size_limit;

        Self {
            inner,
            size_limit,
            start: None,
            end: None,
        }
    }

    #[inline]
    pub fn push(&mut self, state: &mut VmState, element: u64) -> Result<(), StackError> {
        if self.inner.len() + 1 >= self.size_limit {
            return Err(StackError::SizeLimitReached);
        }
        self.inner.push(element);
        state.registers.s.extend();
        self.end = Some(self.end.unwrap() - 8);
        Ok(())
    }

    #[inline]
    pub fn pop(&mut self, state: &mut VmState) -> Option<u64> {
        state.registers.s.reduce();
        self.end = Some(self.end.unwrap() + 8);
        self.inner.pop()
    }

    #[inline(always)]
    fn calc_offseted(&self, v_addres: VirtualMemoryAddress) -> VirtualMemoryAddress {
        v_addres - self.end.unwrap()
    }

    fn word_offset_address(&self, v_addres: VirtualMemoryAddress) -> (usize, usize) {
        let tmp = self.calc_offseted(v_addres);
        let word_address: usize = (tmp / 8).try_into().unwrap();
        let offset_address: usize = (tmp % 8).try_into().unwrap();
        (word_address, offset_address)
    }
}


impl MemoryArea for MemoryStack {
    fn get_size(&self) -> u64 {
        self.inner.len() as u64
    }

    fn contains(&self, v_addres: VirtualMemoryAddress) -> bool {
        let range = (self.start.unwrap())..(self.end.unwrap());
        range.contains(&v_addres)
    }

    fn read_byte(&self, v_addres: VirtualMemoryAddress) -> Option<u8> {
        if !self.contains(v_addres) {
            return None;
        }

        let (word_address, offset) = self.word_offset_address(v_addres);
        self.inner.get(word_address)
            .map(|e: &u64| {
                let le_bytes = e.to_le_bytes();
                le_bytes[offset]
            })
    }

    fn read_word(&self, v_addres: VirtualMemoryAddress) -> Option<u64> {
        if !self.contains(v_addres) {
            return None;
        }

        let (word_address, _) = self.word_offset_address(v_addres);
        self.inner
            .get(word_address)
            .cloned()
    }

    fn write_byte(&mut self, v_addres: VirtualMemoryAddress, content: u8) -> Option<()> {
        if !self.contains(v_addres) {
            return None;
        }


        let (word_address, offset) = self.word_offset_address(v_addres);

        let cur = self
            .inner
            .get(word_address)
            .unwrap();

        let mut cur_bytes = cur.to_le_bytes();

        cur_bytes[offset] = content;

        self.inner[word_address] = u64::from_le_bytes(cur_bytes);
        Some(())
    }

    fn write_word(&mut self, v_addres: VirtualMemoryAddress, content: u64) -> Option<()> {
        if !self.contains(v_addres) {
            return None;
        }

        let (word_address, _) = self.word_offset_address(v_addres);

        self.inner[word_address] = content;

        Some(())
    }

    fn set_end(&mut self, v_addres: VirtualMemoryAddress) {
        self.end = Some(v_addres);
    }

    fn set_start(&mut self, v_addres: VirtualMemoryAddress) {
        self.start = Some(v_addres);
    }

    fn get_end(&self) -> VirtualMemoryAddress {
        self.end.unwrap()
    }

    fn get_start(&self) -> VirtualMemoryAddress {
        self.start.unwrap()
    }
}
