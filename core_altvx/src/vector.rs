#[derive(Debug, Default)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub fn new(x: impl Into<f32>, y: impl Into<f32>, z: impl Into<f32>) -> Self {
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

pub trait IntoVector3 {
    fn into_vector3(self) -> Vector3;
}

impl IntoVector3 for Vector3 {
    fn into_vector3(self) -> Vector3 {
        self
    }
}

impl IntoVector3 for i32 {
    fn into_vector3(self) -> Vector3 {
        Vector3::new(self as f32, self as f32, self as f32)
    }
}

impl IntoVector3 for f32 {
    fn into_vector3(self) -> Vector3 {
        Vector3::new(self, self, self)
    }
}

impl IntoVector3 for (f32, f32, f32) {
    fn into_vector3(self) -> Vector3 {
        Vector3::new(self.0, self.1, self.2)
    }
}

impl IntoVector3 for (i32, i32, i32) {
    fn into_vector3(self) -> Vector3 {
        Vector3::new(self.0 as f32, self.1 as f32, self.2 as f32)
    }
}

#[derive(Debug)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub fn new(x: impl Into<f32>, y: impl Into<f32>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

impl From<f32> for Vector2 {
    fn from(value: f32) -> Self {
        Self::new(value, value)
    }
}

impl From<i32> for Vector2 {
    fn from(value: i32) -> Self {
        Self::new(value as f32, value as f32)
    }
}

pub trait IntoVector2 {
    fn into_vector2(self) -> Vector2;
}

impl IntoVector2 for Vector2 {
    fn into_vector2(self) -> Vector2 {
        self
    }
}

impl IntoVector2 for i32 {
    fn into_vector2(self) -> Vector2 {
        Vector2::new(self as f32, self as f32)
    }
}

impl IntoVector2 for f32 {
    fn into_vector2(self) -> Vector2 {
        Vector2::new(self, self)
    }
}

impl IntoVector2 for (f32, f32) {
    fn into_vector2(self) -> Vector2 {
        Vector2::new(self.0, self.1)
    }
}

impl IntoVector2 for (i32, i32) {
    fn into_vector2(self) -> Vector2 {
        Vector2::new(self.0 as f32, self.1 as f32)
    }
}
