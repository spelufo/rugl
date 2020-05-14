macro_rules! sum {
    ($e:expr) => {
        $e
    };
    ($e1:expr, $($r:expr),+) => {
        $e1 + sum!($($r),+)
    }
}

macro_rules! vector_space {
    ($v:ident $s:ty {$($c:ident: $ct:ty),+}) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        #[repr(C)]
        pub struct $v {
            $(pub $c: $ct),+
        }

        impl $v {
            pub fn dot(v: Self, w: Self) -> f32 {
                sum!($(v.$c as f32 * w.$c as f32),+)
            }

            pub fn norm_squared(self) -> f32 {
                Self::dot(self, self)
            }

            pub fn norm(self) -> f32 {
                self.norm_squared().sqrt()
            }
        }

        impl Add for $v {
            type Output = $v;

            fn add(self, other: $v) -> $v {
                Self {
                    $($c: self.$c + other.$c),+
                }
            }
        }

        impl AddAssign for $v {
            fn add_assign(&mut self, other: $v) {
                $(self.$c += other.$c;)+
            }
        }

        impl Sub for $v {
            type Output = $v;

            fn sub(self, other: $v) -> $v {
                Self {
                    $($c: self.$c - other.$c),+
                }
            }
        }

        impl SubAssign for $v {
            fn sub_assign(&mut self, other: $v) {
                $(self.$c -= other.$c;)+
            }
        }

        impl Neg for $v {
            type Output = $v;

            fn neg(self) -> $v {
                Self {
                    $($c: -self.$c),+
                }
            }
        }

        impl Mul<$v> for $s {
            type Output = $v;

            fn mul(self, rhs: $v) -> $v {
                Self::Output {
                    $($c: self * rhs.$c),+
                }
            }
        }

        impl MulAssign<$s> for $v {
            fn mul_assign(&mut self, rhs: $s) {
                $(self.$c *= rhs;)+
            }
        }

        impl Div<$s> for $v {
            type Output = $v;

            fn div(self, denom: $s) -> $v {
                Self {
                    $($c: self.$c / denom),+
                }
            }
        }

        impl DivAssign<$s> for $v {
            fn div_assign(&mut self, rhs: $s) {
                $(self.$c /= rhs;)+
            }
        }
    }
}
