#[derive(Debug)]
pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl RGBA {
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

impl From<(u8, u8, u8)> for RGBA {
    fn from(value: (u8, u8, u8)) -> Self {
        Self::new(value.0, value.1, value.2, 255)
    }
}

impl From<(u8, u8, u8, u8)> for RGBA {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}
