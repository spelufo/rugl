use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub coords: [f32; 2],
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub coords: [f32; 3],
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color3 {
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

macro_rules! zero {
    ($e:expr) => {0.0}
}

macro_rules! vector_space {
    ($v:ty, $s:ty, ($($i:literal => $c:ident),+)) => {
        impl $v {
            pub fn zero() -> $v {
                Self {coords: [$(zero!($i)),+]}
            }

            pub fn dot(a: $v, b: $v) -> $s {
                sum!($(a.coords[$i] * b.coords[$i]),+)
            }

            pub fn norm_squared(self) -> $s {
                Self::dot(self, self)
            }

            pub fn norm(self) -> $s {
                self.norm_squared().sqrt()
            }

            pub fn normalize(self) -> $v {
                self / self.norm()
            }

            $(pub fn $c(self) -> $s {
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

        impl Mul<$v> for $s {
            type Output = $v;

            fn mul(self, rhs: $v) -> $v {
                Self::Output {
                    coords: [$(self * rhs.coords[$i]),+ ]
                }
            }
        }

        impl Div<$s> for $v {
            type Output = $v;

            fn div(self, denom: $s) -> $v {
                Self {
                    coords: [$(self.coords[$i] / denom),+ ]
                }
            }
        }

        impl Mul<$v> for $v {
            type Output = $v;

            fn mul(self, other: $v) -> $v {
                Self {
                    coords: [$(self.coords[$i] * other.coords[$i]),+ ]
                }
            }
        }

        impl Div<$v> for $v {
            type Output = $v;

            fn div(self, other: $v) -> $v {
                Self {
                    coords: [$(self.coords[$i] / other.coords[$i]),+ ]
                }
            }
        }
    }
}

vector_space!(Vector2, f32, (0 => x, 1 => y));
vector_space!(Vector3, f32, (0 => x, 1 => y, 2 => z));
vector_space!(Color3, f32, (0 => r, 1 => g, 2 => b));




#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix4 {
    pub coords: [[f32; 4]; 4]
}

pub fn mat4(xx: f32, xy: f32, xz: f32, xw: f32,
            yx: f32, yy: f32, yz: f32, yw: f32,
            zx: f32, zy: f32, zz: f32, zw: f32,
            wx: f32, wy: f32, wz: f32, ww: f32) -> Matrix4 {
    Matrix4{
        coords: [
            [xx, yx, zx, wx],
            [xy, yy, zy, wy],
            [xz, yz, zz, wz],
            [xw, yw, zw, ww],
        ]
    }
}

impl Matrix4 {
    pub fn id() -> Matrix4 {
        mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Matrix4 {
        mat4(
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn perspective(fov_x: f32, aspect: f32, near: f32, far: f32) -> Matrix4 {
        let n = near;
        let f = far;
        let r = (fov_x/2.0).tan()*n;
        let t = r / aspect;
        mat4(
            n/r, 0.0, 0.0, 0.0,
            0.0, n/t, 0.0, 0.0,
            0.0, 0.0, -(f + n)/(f - n), -2.0*f*n/(f-n),
            0.0, 0.0, -1.0, 0.0,
        )
    }

    pub fn rotate_z(a: f32) -> Matrix4 {
        mat4(
            a.cos(), -a.sin(), 0.0, 0.0,
            a.sin(), a.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn rotate_y(a: f32) -> Matrix4 {
        mat4(
            a.cos(), 0.0, -a.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            a.sin(), 0.0, a.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut res = Matrix4{coords: [[0.0; 4]; 4]};
        for c in 0..4 {
            for r in 0..4 {
                for i in 0..4 {
                    res.coords[c][r] += self.coords[i][r] * other.coords[c][i]
                }
            }
        }
        res
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Frame3 {
    pub i: Vector3,
    pub j: Vector3,
    pub k: Vector3,
}
