#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline]
    pub fn random() -> Vec3 {
        vec3(super::random(), super::random(), super::random())
    }

    #[inline]
    pub fn random_range(min: f32, max: f32) -> Vec3 {
        vec3(
            super::random_range(min, max),
            super::random_range(min, max),
            super::random_range(min, max),
        )
    }

    #[inline]
    pub fn dot(&self, v: &Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    #[inline]
    pub fn cross(&self, v: &Vec3) -> Vec3 {
        vec3(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    #[inline]
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min(-self.dot(n), 1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt() * n;

        r_out_perp + r_out_parallel
    }

    #[inline]
    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[inline(always)]
pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

#[inline(always)]
pub const fn point3(x: f32, y: f32, z: f32) -> Vec3 {
    Point3 { x, y, z }
}

#[inline(always)]
pub const fn color(r: f32, g: f32, b: f32) -> Color {
    Color { x: r, y: g, z: b }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Vec3 {
        &self + rhs
    }
}

impl std::ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Vec3 {
        vec3(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f32) -> Vec3 {
        self * (1.0 / rhs)
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs
    }
}

// NOTE: Hadamard Product (not cross or dot!)
impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        vec3(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl std::ops::Mul<f32> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        vec3(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        &self * rhs
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        &rhs * self
    }
}

impl std::ops::Mul<&Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::MulAssign for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        vec3(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Vec3 {
        &self - rhs
    }
}

impl std::ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Vec3 {
        vec3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = vec3(1.0, 1.0, 1.0);
        let b = vec3(2.0, 2.0, 2.0);

        let r = a + b;

        assert_eq!(r, vec3(3.0, 3.0, 3.0));
    }

    #[test]
    fn add_assign() {
        let mut a = vec3(1.0, 1.0, 1.0);
        a += vec3(2.0, 2.0, 2.0);

        assert_eq!(a, vec3(3.0, 3.0, 3.0));
    }

    #[test]
    fn div_by_float() {
        let a = vec3(2.0, 2.0, 2.0);
        let r = a / 2.0;

        assert_eq!(r, vec3(1.0, 1.0, 1.0));
    }

    #[test]
    fn div_assign_by_float() {
        let mut a = vec3(2.0, 2.0, 2.0);

        a /= 2.0;

        assert_eq!(a, vec3(1.0, 1.0, 1.0));
    }

    #[test]
    fn hadamard_product() {
        let a = vec3(2.0, 3.0, 4.0);
        let b = vec3(3.0, 4.0, 5.0);

        let r = a * b;

        assert_eq!(r, vec3(6.0, 12.0, 20.0));
    }

    #[test]
    fn scalar_multiplication() {
        let a = vec3(1.0, 2.0, 3.0);

        assert_eq!(a * 2.0, vec3(2.0, 4.0, 6.0));
        assert_eq!(2.0 * a, vec3(2.0, 4.0, 6.0));
    }

    #[test]
    fn multiply_assign() {
        let mut a = vec3(1.0, 2.0, 3.0);

        a *= 2.0;

        assert_eq!(a, vec3(2.0, 4.0, 6.0));
    }

    #[test]
    fn negate() {
        let a = vec3(1.0, 2.0, 3.0);

        assert_eq!(-a, vec3(-1.0, -2.0, -3.0));
    }

    #[test]
    fn subtraction() {
        let a = vec3(1.0, 2.0, 3.0);
        let b = vec3(2.0, 3.0, 4.0);

        let s = a - b;
        let r = a + (-b); // Definition of vector subtraction

        assert_eq!(s, r);
    }

    #[test]
    fn near_zero() {
        assert!(vec3(0.0, 0.0, 0.0).near_zero());
    }

    #[test]
    fn scales_to_unit() {
        let a = vec3(3.0, 4.0, 5.0);
        let u = a.unit();

        let length = (u.x * u.x + u.y * u.y + u.z * u.z).sqrt();

        assert_eq!(1.0, length);
    }

    #[test]
    fn dot_product() {
        let a = vec3(1.0, 2.0, 3.0);
        let b = vec3(3.0, 2.0, 1.0);

        // Definition of a dot product: axbx + ayby + azbz
        let def = a.x * b.x + a.y * b.y + a.z * b.z;

        assert_eq!(a.dot(&b), def);
    }

    #[test]
    fn cross_product() {
        let b = vec3(1.0, 2.0, 3.0);
        let c = vec3(3.0, 2.0, 1.0);

        // Definition of a cross product (https://en.wikipedia.org/wiki/Cross_product#Mnemonic)
        // a = b x c
        // ax = bycz - bzcy
        // ay = bzcx - bxcz
        // az = bxcy - bycx

        let a = vec3(
            b.y * c.z - b.z * c.y,
            b.z * c.x - b.x * c.z,
            b.x * c.y - b.y * c.x,
        );

        assert_eq!(b.cross(&c), a);
    }
}
