pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub fn new<T: Into<f32>>(x: T, y: T, z: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
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
}

impl From<f32> for Vector3 {
    fn from(value: f32) -> Self {
        Self::new(value, value, value)
    }
}

impl From<i32> for Vector3 {
    fn from(value: i32) -> Self {
        Self::new(value as f32, value as f32, value as f32)
    }
}
