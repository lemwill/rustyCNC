
use std::fmt;
use std::ops::Div;
use std::ops::Sub;
use std::ops::Add;
use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Coordinates {
    pub fn length(&self) -> f32 {
        let length = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        return length;
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


impl Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, other: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    
}

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, other: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Div<f32> for Coordinates {
    type Output = Coordinates;

    fn div(self, other: f32) -> Coordinates {
        Coordinates {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}


impl Mul<f32> for Coordinates {
    type Output = Coordinates;

    fn mul(self, other: f32) -> Coordinates {
        Coordinates {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}