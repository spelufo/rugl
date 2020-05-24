macro_rules! rectangle_impl {
    ($Rectangle:ident, $Vector2:ty, $s:ty) => {
        #[derive(Copy, Clone, Debug, Default, PartialEq)]
        pub struct $Rectangle {
            pub min: $Vector2,
            pub max: $Vector2,
        }

        impl $Rectangle {
            pub const ZERO: $Rectangle = $Rectangle { min: <$Vector2>::ZERO, max: <$Vector2>::ZERO };

            pub fn new(min: $Vector2, width: $s, height: $s) -> $Rectangle {
                $Rectangle {
                    min,
                    max: <$Vector2>::new(min.x + width, min.y + height),
                }
            }

            pub fn width(&self) -> $s {
                self.max.x - self.min.x
            }

            pub fn height(&self) -> $s {
                self.max.y - self.min.y
            }

            pub fn size(&self) -> $Vector2 {
                self.max - self.min
            }
        }
    };
}

rectangle_impl!(Rectangle, Vector2, f32);
rectangle_impl!(RectangleI, Vector2I, i32);
