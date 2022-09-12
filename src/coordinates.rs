
use std::fmt;
use std::ops::Sub;
use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


impl Sub for &Coordinates {
    type Output = Coordinates;

    fn sub(self, other: &Coordinates) -> Coordinates {
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