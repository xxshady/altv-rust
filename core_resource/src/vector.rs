use crate::helpers::IntoF32;

#[derive(Debug, Default)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub fn new(x: impl IntoF32, y: impl IntoF32, z: impl IntoF32) -> Self {
        Self {
            x: x.into_f32(),
            y: y.into_f32(),
            z: z.into_f32(),
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

impl<T: IntoF32> From<T> for Vector3 {
    fn from(value: T) -> Self {
        Self::new(value, value, value)
    }
}

impl<X, Y, Z> From<(X, Y, Z)> for Vector3
where
    X: IntoF32,
    Y: IntoF32,
    Z: IntoF32,
{
    fn from((x, y, z): (X, Y, Z)) -> Self {
        Self::new(x, y, z)
    }
}

#[derive(Debug)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub fn new(x: impl IntoF32, y: impl IntoF32) -> Self {
        Self {
            x: x.into_f32(),
            y: y.into_f32(),
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

impl<T: IntoF32> From<T> for Vector2 {
    fn from(value: T) -> Self {
        Self::new(value, value)
    }
}

impl<X, Y> From<(X, Y)> for Vector2
where
    X: IntoF32,
    Y: IntoF32,
{
    fn from((x, y): (X, Y)) -> Self {
        Self::new(x, y)
    }
}
