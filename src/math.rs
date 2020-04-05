use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub coords: [f32; 2],
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub coords: [f32; 3],
}

pub fn vec2(x: f32, y: f32) -> Vector2 {
    Vector2 { coords: [x, y] }
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3 { coords: [x, y, z] }
}

macro_rules! sum {
    ($e:expr) => {
        $e
    };
    ($e1:expr, $($r:expr),+) => {
        $e1 + sum!($($r),+)
    }
}

macro_rules! vector_space_f32_coords {
    ($v:ty, ($($i:literal => $c:ident),+)) => {
        impl $v {
            pub fn norm_squared(self) -> f32 {
                sum!($(self.coords[$i] * self.coords[$i]),+)
            }

            pub fn norm(self) -> f32 {
                self.norm_squared().sqrt()
            }

            pub fn normalize(self) -> $v {
                self / self.norm()
            }

            $(pub fn $c(self) -> f32 {
                self.coords[$i]
            })+
        }

        impl Add for $v {
            type Output = $v;

            fn add(self, other: $v) -> $v {
                Self {
                    coords: [$(self.coords[$i] + other.coords[$i]),+]
                }
            }
        }

        impl Sub for $v {
            type Output = $v;

            fn sub(self, other: $v) -> $v {
                Self {
                    coords: [$(self.coords[$i] - other.coords[$i]),+]
                }
            }
        }

        impl Neg for $v {
            type Output = $v;

            fn neg(self) -> $v {
                Self {
                    coords: [$(-self.coords[$i]),+]
                }
            }
        }

        impl Mul<$v> for f32 {
            type Output = $v;

            fn mul(self, rhs: $v) -> $v {
                Self::Output {
                    coords: [$(self * rhs.coords[$i]),+ ]
                }
            }
        }

        impl Div<f32> for $v {
            type Output = $v;

            fn div(self, denom: f32) -> $v {
                Self {
                    coords: [$(self.coords[$i] / denom),+ ]
                }
            }
        }
    }
}

vector_space_f32_coords!(Vector2, (0 => x, 1 => y));
vector_space_f32_coords!(Vector3, (0 => x, 1 => y, 2 => z));

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Frame3 {
    pub i: Vector3,
    pub j: Vector3,
    pub k: Vector3,
}
