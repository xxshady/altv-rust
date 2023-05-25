#[derive(Debug)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgba {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn a(&self) -> u8 {
        self.a
    }
}

impl From<(u8, u8, u8)> for Rgba {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b, 255)
    }
}

impl From<(u8, u8, u8, u8)> for Rgba {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::new(r, g, b, a)
    }
}

mvalue::generate_serde_via_bytes_for!(
    Rgba,
    "Rgba",
    mvalue::ser_rgba::RGBA_MVALUE,
    rgba_serde_impl,
    |v: &Self| { [v.r, v.g, v.b, v.a] },
    |v: Vec<u8>| {
        let Some([r, g, b, a]) = mvalue::bytes_num::from_byte_buf::<u8, 1, 4>(&v) else {
            return Err(E::custom("Failed to deserialize Rgba"));
        };
        Ok(Rgba::new(r, g, b, a))
    }
);
