use std::mem::size_of;

const _: () = assert!(size_of::<usize>() == size_of::<u64>());
