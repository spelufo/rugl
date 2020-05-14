vector_space!(Quaternion f32 {
    s: f32,
    x: f32,
    y: f32,
    z: f32
});

impl Quaternion {
    pub const ZERO: Quaternion = Quaternion { s: 0., x: 0., y: 0., z: 0. };
    pub const ONE: Quaternion = Quaternion { s: 1., x: 0., y: 0., z: 0. };
    pub const X: Quaternion = Quaternion { s: 0., x: 1., y: 0., z: 0. };
    pub const Y: Quaternion = Quaternion { s: 0., x: 0., y: 1., z: 0. };
    pub const Z: Quaternion = Quaternion { s: 0., x: 0., y: 0., z: 1. };

    pub fn normalized(self) -> Quaternion {
        self / self.norm()
    }

    pub fn rotation(axis: Vector3, angle: f32) -> Quaternion {
        let axis = axis.normalized();
        let cos = (angle/2.).cos();
        let sin = (angle/2.).sin();
        Quaternion {
            s: cos,
            x: sin * axis.x,
            y: sin * axis.y,
            z: sin * axis.z,
        }
    }

    pub fn conj(self) -> Quaternion {
        Quaternion { s: self.s, x: -self.x, y: -self.y, z: -self.z }
    }

    pub fn v(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn rotate(self, v: Vector3) -> Vector3 {
        (self * Quaternion::from(v) * self.conj()).v()
    }

    pub fn rotation_matrix(self) -> Matrix4 {
        let fx = self.frame_x();
        let fy = self.frame_y();
        let fz = self.frame_z();
        mat4(
            fx.x, fy.x, fz.x, 0.,
            fx.y, fy.y, fz.y, 0.,
            fx.z, fy.z, fz.z, 0.,
            0., 0., 0., 1.,
        )
    }

    pub fn frame_x(self) -> Vector3 {
        self.rotate(Vector3::X)
    }

    pub fn frame_y(self) -> Vector3 {
        self.rotate(Vector3::Y)
    }

    pub fn frame_z(self) -> Vector3 {
        self.rotate(Vector3::Z)
    }
}

impl From<Vector3> for Quaternion {
    fn from(v: Vector3) -> Quaternion {
        Quaternion{s: 0., x: v.x, y: v.y, z: v.z}
    }
}

impl From<f32> for Quaternion {
    fn from(s: f32) -> Quaternion {
        Quaternion{s, x: 0., y: 0., z: 0.}
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion {
            s: self.s * other.s - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.s * other.x + other.s * self.x + self.y * other.z - self.z * other.y,
            y: self.s * other.y + other.s * self.y + self.z * other.x - self.x * other.z,
            z: self.s * other.z + other.s * self.z + self.x * other.y - self.y * other.x,
        }
    }
}

impl MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, other: Quaternion) {
        *self = *self * other;
    }
}

impl Div<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn div(self, other: Quaternion) -> Quaternion {
        // Quaternion {
        //     s: self.s * other.s + self.x * other.x + self.y * other.y + self.z * other.z,
        //     x: -self.s * other.x + other.s * self.x - self.y * other.z + self.z * other.y,
        //     y: -self.s * other.y + other.s * self.y - self.z * other.x + self.x * other.z,
        //     z: -self.s * other.z + other.s * self.z - self.x * other.y + self.y * other.x,
        // } / other.norm_squared()
        self * other.conj() / other.norm_squared()
    }
}

impl DivAssign<Quaternion> for Quaternion {
    fn div_assign(&mut self, other: Quaternion) {
        *self = *self / other;
    }
}
