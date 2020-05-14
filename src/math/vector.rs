macro_rules! vector2_impl {
    ($Vector2:ident, $s:ty) => {
        vector_space!($Vector2 $s {
            x: $s,
            y: $s
        });

        impl $Vector2 {
            pub const ZERO: $Vector2 = $Vector2 { x: 0 as $s, y: 0 as $s };
            pub const X: $Vector2 = $Vector2 { x: 1 as $s, y: 0 as $s };
            pub const Y: $Vector2 = $Vector2 { x: 0 as $s, y: 1 as $s };

            pub fn new(x: $s, y: $s) -> $Vector2 {
                $Vector2 { x, y }
            }
        }
    }
}

vector2_impl!(Vector2, f32);
vector2_impl!(Vector2I, i32);

impl Vector2 {
    pub fn normalized(self) -> Vector2 {
        self / self.norm()
    }
}


macro_rules! vector3_impl {
    ($Vector3:ident, $s:ty) => {
        vector_space!($Vector3 $s {
            x: $s,
            y: $s,
            z: $s
        });

        impl $Vector3 {
            pub const ZERO: $Vector3 = $Vector3 { x: 0 as $s, y: 0 as $s, z: 0 as $s };
            pub const X: $Vector3 = $Vector3 { x: 1 as $s, y: 0 as $s, z: 0 as $s };
            pub const Y: $Vector3 = $Vector3 { x: 0 as $s, y: 1 as $s, z: 0 as $s };
            pub const Z: $Vector3 = $Vector3 { x: 0 as $s, y: 0 as $s, z: 1 as $s };

            pub fn new(x: $s, y: $s, z: $s) -> $Vector3 {
                $Vector3 { x, y, z }
            }

            pub fn cross(v: $Vector3, w: $Vector3) -> $Vector3 {
                $Vector3 {
                    x: v.y * w.z - v.z * w.y,
                    y: v.z * w.x - v.x * w.z,
                    z: v.x * w.y - v.y * w.x,
                }
            }
        }
    }
}

vector3_impl!(Vector3, f32);
vector3_impl!(Vector3I, i32);

impl Vector3 {
    pub fn normalized(self) -> Vector3 {
        self / self.norm()
    }
}
