use std::fmt;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn length(&self) -> f32 {
        let length = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        return length;
    }

    pub fn dot(&self, other: Vector) -> f32 {
        let dot = self.x * other.x + self.y * other.y + self.z * other.z;
        return dot;
    }

    pub fn angle_with(&self, other: Vector) -> f32 {
        let mut angle = self.dot(other) / (self.length() * other.length());
        // Fix rounding error
        if angle < -1.0 {
            angle = -1.0;
        } else if angle > 1.0 {
            angle = 1.0;
        }
        return f32::acos(angle);
    }
}

impl Default for Vector {
    fn default() -> Self {
        Vector { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, other: f32) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
