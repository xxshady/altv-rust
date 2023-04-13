#[derive(Debug, Default)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub fn new(x: impl Into<f32>, y: impl Into<f32>, z: impl Into<f32>, w: impl Into<f32>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn w(&self) -> f32 {
        self.w
    }
}

impl From<i32> for Quaternion {
    fn from(value: i32) -> Self {
        Self::new(value as f32, value as f32, value as f32, value as f32)
    }
}
