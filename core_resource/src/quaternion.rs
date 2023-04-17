use crate::helpers::IntoF32;

#[derive(Debug, Default)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub fn new(x: impl IntoF32, y: impl IntoF32, z: impl IntoF32, w: impl IntoF32) -> Self {
        Self {
            x: x.into_f32(),
            y: y.into_f32(),
            z: z.into_f32(),
            w: w.into_f32(),
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

impl<T: IntoF32> From<T> for Quaternion {
    fn from(value: T) -> Self {
        Self::new(value, value, value, value)
    }
}

impl<X, Y, Z, W> From<(X, Y, Z, W)> for Quaternion
where
    X: IntoF32,
    Y: IntoF32,
    Z: IntoF32,
    W: IntoF32,
{
    fn from((x, y, z, w): (X, Y, Z, W)) -> Self {
        Self::new(x, y, z, w)
    }
}
