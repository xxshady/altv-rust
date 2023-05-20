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

mvalue::generate_serde_via_bytes_for!(
    Vector3,
    "Vector3",
    mvalue::ser_vector3::VECTOR3_MVALUE,
    vector3_serde_impl,
    |v: &Self| { [v.x, v.y, v.z] },
    |v: Vec<u8>| {
        let [x, y, z] = mvalue::bytes_num::from_byte_buf::<f32, 4, 3>(&v);
        Vector3::new(x, y, z)
    }
);

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

mvalue::generate_serde_via_bytes_for!(
    Vector2,
    "Vector2",
    mvalue::ser_vector2::VECTOR2_MVALUE,
    vector2_serde_impl,
    |v: &Self| { [v.x, v.y] },
    |v: Vec<u8>| {
        let [x, y] = mvalue::bytes_num::from_byte_buf::<f32, 4, 2>(&v);
        Vector2::new(x, y)
    }
);
