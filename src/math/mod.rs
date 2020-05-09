use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub use std::f32::consts::*;

pub const TAU: f32 = 2. * PI;

include!("vector_space.rs");
include!("color.rs");
include!("vector.rs");
include!("matrix.rs");
include!("quaternion.rs");
