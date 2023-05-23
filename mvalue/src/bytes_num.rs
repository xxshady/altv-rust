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

pub fn from_byte_buf<T, const S: usize, const N: usize>(buf: &[u8]) -> Option<[T; N]>
where
    T: BytesNum<S> + Default + Copy,
{
    let mut array: [T; N] = [Default::default(); N];
    let num_size = std::mem::size_of::<T>();

    for (idx, e) in array.iter_mut().enumerate() {
        let slice = buf.get((idx * num_size)..((1 + idx) * num_size));
        let Some(slice) = slice else {
            return None;
        };

        let result = slice.try_into();

        match result {
            Ok(v) => {
                *e = T::from_le_bytes(v);
            }
            Err(e) => {
                logger::error!("Failed to convert slice to static array, error: {e:?}");
                return None;
            }
        }
    }

    Some(array)
}
