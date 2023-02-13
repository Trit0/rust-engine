const PI: f64 = 3.14159265358979323846;

#[derive(Debug)]
pub struct Vector3F {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::ops::Add for Vector3F {
    type Output = Vector3F;

    fn add(self, v: Self) -> Self::Output {
        return Vector3F {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        };
    }
}

impl std::ops::AddAssign for Vector3F {
    fn add_assign(&mut self, v: Self) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl std::ops::Sub for Vector3F {
    type Output = Vector3F;

    fn sub(self, v: Self) -> Self::Output {
        return Vector3F {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z
        }
    }
}

impl std::ops::SubAssign for Vector3F {
    fn sub_assign(&mut self, v: Self) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }
}

// Scalar product
impl std::ops::Mul<Vector3F> for Vector3F {
    type Output = f64;

    fn mul(self, v: Self) -> Self::Output {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }
}

impl std::ops::Mul<f64> for Vector3F {
    type Output = Vector3F;

    fn mul(self, s: f64) -> Self::Output {
        return Vector3F {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s
        };
    }
}

impl std::ops::Div<f64> for Vector3F {
    type Output = Vector3F;

    fn div(self, s: f64) -> Self::Output {
        return Vector3F {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s
        };
    }
}

// Vectorial product
impl std::ops::Rem for Vector3F {
    type Output = Vector3F;

    fn rem(self, v: Self) -> Self::Output {
        return Vector3F {
            x: (self.y * v.z) - (v.y * self.z),
            y: (self.z * v.x) - (v.z * self.x),
            z: (self.x * v.y) - (v.x * self.y)
        };
    }
}

impl PartialEq for Vector3F {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
}

impl Eq for Vector3F {}

impl Vector3F {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3F {
        return Vector3F {
            x: (x),
            y: (y),
            z: (z),
        }
    }

    pub fn get_norm(self) -> f64 {
        return ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
    }

    pub fn get_angle(self, v: Vector3F) -> f64 {
        let a = self * v;
        let n = self.get_norm() * v.get_norm();
        let i = (a * PI / 180.0) / (n * PI / 180.0);
        let c = if i as u32 == 1 { 0.0 } else {
            if i as i32 == -1 { PI } else { i.acos() }
        };
        return c * 180.0 / PI;
    }

    pub fn normalize(&mut self) {
        let norm = self.get_norm();
        if norm != 0.0 {
            self.x /= norm;
            self.y /= norm;
            self.z /= norm;
        }
    }

    pub fn get_normalized(v: Vector3F) -> Vector3F {
        let norm = v.get_norm();

        if norm == 0.0 {
            return v;
        }

        return Vector3F {
            x: v.x / norm,
            y: v.y / norm,
            z: v.z / norm
        };
    }

    pub fn same_direction(self, v: Vector3F) -> bool {
        return Vector3F::get_normalized(self) == Vector3F::get_normalized(v);
    }

    pub fn lerp(self, v2: Vector3F, t: f64) -> Vector3F {
        let clamped_t = t.clamp(0.0, 1.0);

        return (self * -clamped_t) + (v2 * clamped_t);
    }

    pub fn zero() -> Vector3F {
        return Vector3F { x: 0.0, y: 0.0, z: 0.0 };
    }

    pub fn identity() -> Vector3F {
        return Vector3F { x: 1.0, y: 1.0, z: 1.0 };
    }

    pub fn left() -> Vector3F {
        return Vector3F { x: -1.0, y: 0.0, z: 0.0 };
    }

    pub fn right() -> Vector3F {
        return Vector3F { x: 1.0, y: 0.0, z: 0.0 };
    }

    pub fn down() -> Vector3F {
        return Vector3F { x: 0.0, y: -1.0, z: 0.0 };
    }

    pub fn up() -> Vector3F {
        return Vector3F { x: 0.0, y: 1.0, z: 0.0 };
    }

    pub fn forward() -> Vector3F {
        return Vector3F { x: 0.0, y: 0.0, z: -1.0 };
    }

    pub fn backward() -> Vector3F {
        return Vector3F { x: 0.0, y: 0.0, z: 1.0 };
    }
}

impl std::fmt::Display for Vector3F {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
    }
}

impl Clone for Vector3F {
    fn clone(&self) -> Self {
        return Vector3F { x: (self.x), y: (self.y), z: (self.z) };
    }
}

impl Copy for Vector3F {}
