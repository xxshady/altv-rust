pub trait BytesNum<const S: usize> {
    fn from_le_bytes(bytes: [u8; S]) -> Self;
    fn to_le_bytes(self) -> [u8; S];
}

impl BytesNum<4> for f32 {
    fn from_le_bytes(bytes: [u8; 4]) -> Self {
        f32::from_le_bytes(bytes)
    }

    fn to_le_bytes(self) -> [u8; 4] {
        self.to_le_bytes()
    }
}

impl BytesNum<1> for u8 {
    fn from_le_bytes(bytes: [u8; 1]) -> Self {
        u8::from_le_bytes(bytes)
    }

    fn to_le_bytes(self) -> [u8; 1] {
        self.to_le_bytes()
    }
}

pub fn to_byte_buf<const N: usize, const S: usize, T: BytesNum<S>>(array: [T; N]) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(std::mem::size_of::<T>() * array.len());
    for v in array {
        buf.extend(v.to_le_bytes())
    }
    buf
}

pub fn from_byte_buf<T, const S: usize, const N: usize>(buf: &[u8]) -> [T; N]
where
    T: BytesNum<S> + Default + Copy,
{
    let mut array: [T; N] = [Default::default(); N];
    let num_size = std::mem::size_of::<T>();

    for i in 0..N {
        array[i] = T::from_le_bytes(
            buf[(i * num_size)..((1 + i) * num_size)]
                .try_into()
                .unwrap(),
        );
    }
    array
}
