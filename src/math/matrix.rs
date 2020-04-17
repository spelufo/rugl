
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
    pub fn as_ptr(&self) -> *const f32 {
        self.coords[0].as_ptr()
    }

    pub fn id() -> Matrix4 {
        mat4(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        )
    }

    pub fn translate(v: Vector3) -> Matrix4 {
        mat4(
            1., 0., 0., v.x,
            0., 1., 0., v.y,
            0., 0., 1., v.z,
            0., 0., 0., 1.,
        )
    }

    pub fn perspective(fov_x: f32, aspect: f32, near: f32, far: f32) -> Matrix4 {
        let n = near;
        let f = far;
        let r = (fov_x/2.).tan()*n;
        let t = r / aspect;
        mat4(
            n/r, 0., 0., 0.,
            0., n/t, 0., 0.,
            0., 0., -(f + n)/(f - n), -2.*f*n/(f-n),
            0., 0., -1., 0.,
        )
    }

    pub fn rotate_z(a: f32) -> Matrix4 {
        mat4(
            a.cos(), -a.sin(), 0., 0.,
            a.sin(), a.cos(), 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        )
    }

    pub fn rotate_y(a: f32) -> Matrix4 {
        mat4(
            a.cos(), 0., -a.sin(), 0.,
            0., 1., 0., 0.,
            a.sin(), 0., a.cos(), 0.,
            0., 0., 0., 1.,
        )
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut res = Matrix4{coords: [[0.; 4]; 4]};
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