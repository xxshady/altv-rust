use std::collections::HashMap;

use shared::MemoryBufferId;

const SIMULTANEOUS_IDS: usize = u8::MAX as usize;

pub struct MemoryBufferManager {
    free_ids: [bool; SIMULTANEOUS_IDS],
    buffers: HashMap<MemoryBufferId, Vec<u8>>,
}

impl MemoryBufferManager {
    pub fn new() -> Self {
        Self {
            free_ids: [true; SIMULTANEOUS_IDS],
            buffers: HashMap::with_capacity(u8::MAX.into()),
        }
    }

    pub fn alloc(&mut self, size: u16) -> MemoryBufferId {
        let free_id = self.free_ids.iter().enumerate().find(|(_, free)| **free);
        let Some((index, _)) = free_id else {
            return 0;
        };
        self.free_ids[index] = false;

        let id: MemoryBufferId = (index as u8) + 1;
        self.buffers.insert(id, vec![0; size as usize]);

        println!(
            "after alloc id: {id} free_ids: {:?} buffers: {:?}",
            self.free_ids, self.buffers
        );
        id
    }

    pub fn dealloc(&mut self, id: u8) -> Vec<u8> {
        let index = (id - 1) as usize;
        self.free_ids[index] = true;
        let content = self.buffers.remove(&id).unwrap();

        println!(
            "after dealloc id: {id} free_ids: {:?} buffers: {:?}",
            self.free_ids, self.buffers
        );

        content
    }

    pub fn read(&mut self, id: MemoryBufferId) -> Vec<u8> {
        self.dealloc(id)
    }

    pub fn get_mut_ptr(&mut self, id: MemoryBufferId) -> *mut u8 {
        self.buffers.get_mut(&id).unwrap().as_mut_ptr()
    }
}
