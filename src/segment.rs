
use std::fmt;

use crate::coordinates;
use coordinates::Coordinates;


pub struct Segment {
    start: Coordinates,
    stop: Coordinates,
    feed_rate: f32,
}


impl Segment {

    pub fn new(start: Coordinates, stop: Coordinates, feed_rate: f32) -> Segment {
        Segment {
            start,
            stop,
            feed_rate,
        }
    }

    pub fn length(&self) -> f32 {
        let coordinates_diff = self.stop - self.start;
        let length = (coordinates_diff.x.powi(2) + coordinates_diff.y.powi(2) + coordinates_diff.z.powi(2)).sqrt();
        return length;
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Start: {}, Stop: {}, Feedrate {})", self.start, self.stop, self.feed_rate)
    }
}

