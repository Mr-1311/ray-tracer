use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn r(&self) -> f64 {
        self.x
    }
    pub fn g(&self) -> f64 {
        self.y
    }
    pub fn b(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let vec = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(
            vec,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0
            }
        );
    }

    #[test]
    fn length() {
        let vec = Vec3::new(0.0, 3.0, 4.0);

        assert_eq!(vec.length(), 5.0);
        assert_eq!(vec.squared_length(), 25.0);
    }

    #[test]
    fn vector_add() {
        let vec1 = Vec3::new(0.0, 1.0, 2.0);
        let vec2 = Vec3::new(2.0, 1.0, 0.0);

        let add = vec1 + vec2;
        assert_eq!(add, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn vector_sub() {
        let vec1 = Vec3::new(0.0, 1.0, 2.0);
        let vec2 = Vec3::new(2.0, 1.0, 0.0);

        let sub = vec1 - vec2;
        assert_eq!(sub, Vec3::new(-2.0, 0.0, 2.0));
    }

    #[test]
    fn vector_mul() {
        let vec1 = Vec3::new(0.0, 1.0, 2.0);
        let vec2 = Vec3::new(2.0, 1.0, 3.0);

        let mul = vec1 * vec2;
        assert_eq!(mul, Vec3::new(0.0, 1.0, 6.0));

        let mul = vec1 * 2.0;
        assert_eq!(mul, Vec3::new(0.0, 2.0, 4.0));

        let mul = 2.0 * vec1;
        assert_eq!(mul, Vec3::new(0.0, 2.0, 4.0));
    }

    #[test]
    fn vector_div() {
        let vec1 = Vec3::new(0.0, 2.0, 4.0);
        let vec2 = Vec3::new(2.0, 1.0, 8.0);

        let div = vec1 / vec2;
        assert_eq!(div, Vec3::new(0.0, 2.0, 0.5));

        let div = vec1 / 2.0;
        assert_eq!(div, Vec3::new(0.0, 1.0, 2.0));

        let vec1 = Vec3::new(1.0, 2.0, 4.0);
        let div = 2.0 / vec1;
        assert_eq!(div, Vec3::new(2.0, 1.0, 0.5));
    }

    #[test]
    fn vector_neg() {
        let vec1 = Vec3::new(0.0, 2.0, 4.0);

        let neg = -vec1;
        assert_eq!(neg, Vec3::new(0.0, -2.0, -4.0));
    }
}
