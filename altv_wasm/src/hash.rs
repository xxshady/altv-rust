// credits to altv-rs creator
// https://github.com/justdimaa/altv-rs/blob/f5cf1733493466634793804dfb1ca6d387fbe687/altv-sdk/src/lib.rs#L24
/// joaat hash function
pub fn hash(str: &str) -> u32 {
    let str = str.to_lowercase();
    let bytes = str.as_bytes();
    let mut num: std::num::Wrapping<u32> = std::num::Wrapping(0u32);

    for n in bytes {
        num += std::num::Wrapping(*n as u32);
        num += num << 10;
        num ^= num >> 6;
    }

    num += num << 3;
    num ^= num >> 11;

    (num + (num << 15)).0
}

pub type Hash = u32;

pub trait IntoHash {
    fn into_hash(self) -> Hash;
}

impl IntoHash for Hash {
    fn into_hash(self) -> Hash {
        self
    }
}

impl IntoHash for &str {
    fn into_hash(self) -> Hash {
        hash(self)
    }
}

impl IntoHash for String {
    fn into_hash(self) -> Hash {
        hash(&self)
    }
}
