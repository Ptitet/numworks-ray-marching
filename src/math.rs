use core::ops::{Add, AddAssign, Div, Mul, Sub};

/// 2-dimensional vector (floats)
#[derive(Default, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Vec2::new(x, y)
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Vec2::new(x, y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Vec2::new(x, y)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        Vec2::new(x, y)
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    /// get the norm (or the length) of the vector
    pub fn magnitude(self) -> f32 {
        sqrt(self.x * self.x + self.y * self.y)
    }

    /// normalize the vector
    pub fn normalize(&mut self) {
        let inv_sqrt_norm = inv_sqrt(self.x * self.x + self.y * self.y);
        self.x = self.x * inv_sqrt_norm;
        self.y = self.y * inv_sqrt_norm;
    }

    /// return a normalized copy of the vector
    pub fn normalized(self) -> Self {
        let inv_sqrt_norm = inv_sqrt(self.magnitude());
        Vec2 {
            x: self.x * inv_sqrt_norm,
            y: self.y * inv_sqrt_norm,
        }
    }

    /// return the dot product of the vectors
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// return a new vector linearly interpolated to a target vector
    pub fn lerp(self, target: Self, t: f32) -> Self {
        self + (target - self) * t
    }
}

/// 3-dimensional vector (floats)
#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Vec3::new(x, y, z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Vec3::new(x, y, z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Vec3::new(x, y, z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        Vec3::new(x, y, z)
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    /// get the norm (or the length) of the vector
    pub fn magnitude(self) -> f32 {
        sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    /// normalize the vector
    pub fn normalize(&mut self) {
        let inv_sqrt_norm = inv_sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        self.x = self.x * inv_sqrt_norm;
        self.y = self.y * inv_sqrt_norm;
        self.z = self.z * inv_sqrt_norm;
    }

    /// return a normalized copy of the vector
    pub fn normalized(self) -> Self {
        let inv_sqrt_norm = inv_sqrt(self.magnitude());
        Vec3 {
            x: self.x * inv_sqrt_norm,
            y: self.y * inv_sqrt_norm,
            z: self.z * inv_sqrt_norm,
        }
    }

    /// return the dot product of the vectors
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// return a new vector linearly interpolated to a target vector
    pub fn lerp(self, target: Self, t: f32) -> Self {
        self + (target - self) * t
    }
}

/// Quake's Fast Inverse Square Root
pub fn inv_sqrt(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);
    y * (1.5 - 0.5 * x * y * y)
}

/// sqrt derived from fast inv sqrt
pub fn sqrt(x: f32) -> f32 {
    1. / inv_sqrt(x)
}

/// trigonometry module
pub mod trigo {
    use core::f32;

    /// 5th degree polynomial interpolation of the first quarter (of 2pi) of the cosine function
    fn quarter_cos(x: f32) -> f32 {
        1.00002 - 0.000447247 * x - 0.497081 * x * x - 0.00767104 * x * x * x
            + 0.0512404 * x * x * x * x
            - 0.00575391 * x * x * x * x * x
    }

    /// cosine function (approx)
    pub fn cos(x: f32) -> f32 {
        // transformations
        let mut x = ((x % f32::consts::TAU) + f32::consts::TAU) % f32::consts::TAU; // bc rust modulo can be neg ):<
        if x > f32::consts::PI {
            x = f32::consts::TAU - x;
        }
        let mut multiplier = 1.;
        if x > f32::consts::FRAC_PI_2 {
            multiplier = -1.;
            x = f32::consts::PI - x;
        }
        //sample
        quarter_cos(x) * multiplier
    }

    /// sine function (approx)
    pub fn sin(x: f32) -> f32 {
        cos(x - f32::consts::FRAC_PI_2)
    }

    /// tangent function (approx)
    pub fn tan(x: f32) -> f32 {
        sin(x) / cos(x)
    }

    /// inverse tangent (approx)
    pub fn inv_tan(x: f32) -> f32 {
        cos(x) / sin(x)
    }

    // TODO : other approx (taylor ?)
    // https://www.desmos.com/calculator/ymtgipxmdg
    /// atan function (please don't kill me)
    pub fn atan(x: f32) -> f32 {
        if x >= 0. {
            -1. / (0.636 * x + f32::consts::FRAC_2_PI) + f32::consts::FRAC_PI_2
        } else {
            -1. / (0.636 * x - f32::consts::FRAC_2_PI) - f32::consts::FRAC_PI_2
        }
    }
}
