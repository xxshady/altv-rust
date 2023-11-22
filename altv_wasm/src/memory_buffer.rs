use shared::MemoryBufferId;

use crate::__imports;

#[derive(Debug)]
pub struct MemoryBuffer {
    id: MemoryBufferId,
}

#[derive(Debug)]
pub enum MemoryBufferCreateError {
    SizeCannotBeLargerThan1Kb,
    SimultaneousBufferLimitReached,
}

impl MemoryBuffer {
    pub fn new(size: u16) -> Result<Self, MemoryBufferCreateError> {
        if size > 1024 {
            return Err(MemoryBufferCreateError::SizeCannotBeLargerThan1Kb);
        }

        let id = __imports::alloc_memory_buffer(size);

        if id == 0 {
            return Err(MemoryBufferCreateError::SimultaneousBufferLimitReached);
        }

        Ok(Self { id })
    }

    pub fn id(&self) -> MemoryBufferId {
        self.id
    }

    pub fn read(mut self) -> Vec<u8> {
        let content = __imports::read_memory_buffer(self.id);
        self.id = 0;
        content
    }
}

impl Drop for MemoryBuffer {
    fn drop(&mut self) {
        if self.id == 0 {
            return;
        }
        __imports::dealloc_memory_buffer(self.id);
    }
}
