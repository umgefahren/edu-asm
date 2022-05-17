pub mod stack;

pub use stack::*;

use rand::{thread_rng, RngCore};

use crate::settings::VmSettings;

type VirtualMemoryAddress = u64;

#[cfg(unix)]
fn get_page_size_unix() -> u64 {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE).try_into().unwrap() }
}

#[cfg(windows)]
fn get_page_size_windows() -> u64 {
    let mut info =  windows::Win32::System::SystemInformation::SYSTEM_INFO::default();
    let info_ptr: *mut windows::Win32::System::SystemInformation::SYSTEM_INFO = &mut info;
    unsafe { windows::Win32::System::SystemInformation::GetSystemInfo(info_ptr); }
    let page_size = info.dwPageSize;
    page_size as u64
}

#[cfg(wasm)]
fn get_page_size_wasm() -> u64 {
    const WASM_PAGE_SIZE: u64 = 2.pow(16);
    WASM_PAGE_SIZE
}

#[cfg(not(any(windows, wasm, unix)))]
const FALL_BACK_PAGE_SIZE: u64 = 2_u64.pow(16);

pub fn get_host_page_size() -> u64 {
    cfg_if::cfg_if! {
        if #[cfg(unix)] {
            get_page_size_unix()
        } else if #[cfg(windows)] {
            get_page_size_windows()
        } else if #[cfg(wasm)] {
            get_page_size_wasm()
        } else {
            FALL_BACK_PAGE_SIZE
        }
    }
}

lazy_static::lazy_static! {
    pub static ref HOST_PAGE_SIZE: u64 = get_host_page_size();
}

pub trait MemoryArea {
    fn get_size(&self) -> u64;
    fn contains(&self, v_addres: VirtualMemoryAddress) -> bool;
    fn read_byte(&self, v_addres: VirtualMemoryAddress) -> Option<u8>;
    fn read_word(&self, v_addres: VirtualMemoryAddress) -> Option<u64>;
    fn write_byte(&mut self, v_addres: VirtualMemoryAddress, content: u8) -> Option<()>;
    fn write_word(&mut self, v_addres: VirtualMemoryAddress, content: u64) -> Option<()>;
    fn set_end(&mut self, v_addres: VirtualMemoryAddress);
    fn set_start(&mut self, v_addres: VirtualMemoryAddress);
    fn get_end(&self) -> VirtualMemoryAddress;
    fn get_start(&self) -> VirtualMemoryAddress;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryPage {
    content: Vec<u8>,
    size: u64,
    start: Option<VirtualMemoryAddress>,
    end: Option<VirtualMemoryAddress>,
}

impl MemoryPage {
    pub fn new(settings: &VmSettings) -> Self {
        let size: usize = settings.page_size.try_into().unwrap();
        let mut content = vec![0u8; size];

        if !settings.zero_initialized_page {
            let mut rng = thread_rng();

            rng.fill_bytes(&mut content);
        }

        Self {
            content,
            size: settings.page_size,
            start: None,
            end: None
        }
    }

    pub fn calc_off(&self, v_addres: VirtualMemoryAddress) -> VirtualMemoryAddress {
        v_addres - self.start.unwrap()
    }
}

impl MemoryArea for MemoryPage {
    fn get_size(&self) -> u64 {
        self.size
    }

    fn contains(&self, v_addres: VirtualMemoryAddress) -> bool {
        let range = (self.start.unwrap())..(self.end.unwrap());
        range.contains(&v_addres)
    }

    fn read_byte(&self, v_addres: VirtualMemoryAddress) -> Option<u8> {
        if !self.contains(v_addres) {
            return None;
        }
        let offset = self.calc_off(v_addres);
        self.content.get(offset as usize).cloned()
    }

    fn read_word(&self, v_addres: VirtualMemoryAddress) -> Option<u64> {
        if !self.contains(v_addres) || !self.contains(v_addres + 8) {
            return None;
        }

        let offset = self.calc_off(v_addres) as usize;
        let mut res_buf = [0u8; 8];
        res_buf.copy_from_slice(&self.content[offset..(offset + 8)]);
        Some(u64::from_le_bytes(res_buf))
    }

    fn write_byte(&mut self, v_addres: VirtualMemoryAddress, content: u8) -> Option<()> {
        if !self.contains(v_addres) {
            return None;
        }

        let offset = self.calc_off(v_addres) as usize;
        self.content[offset] = content;

        Some(())
    }

    fn write_word(&mut self, v_addres: VirtualMemoryAddress, content: u64) -> Option<()> {
        if !self.contains(v_addres) || !self.contains(v_addres + 8) {
            return None;
        }

        let offset = self.calc_off(v_addres) as usize;

        let content_bytes = content.to_le_bytes();

        for i in offset..(offset + 8) {
            self.content[i] = content_bytes[i - offset];
        }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryCollection {
    pub stack: MemoryStack
}
