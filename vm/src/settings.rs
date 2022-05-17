use crate::memory::HOST_PAGE_SIZE;

const DEFAULT_STACK_SIZE_LIMIT: usize = 4_000 / 8; 

#[derive(Copy, Clone, Debug)]
pub struct VmSettings {
    pub zero_initialized_register: bool,
    pub zero_initialized_page: bool,
    pub page_size: u64,
    pub stack_size_limit: usize,
}

impl VmSettings {
    pub fn new_default() -> Self {
        Self {
            zero_initialized_register: true,
            zero_initialized_page: true,
            page_size: *HOST_PAGE_SIZE,
            stack_size_limit: DEFAULT_STACK_SIZE_LIMIT
        }
    }
}

impl Default for VmSettings {
    fn default() -> Self {
        Self::new_default()
    }
}
