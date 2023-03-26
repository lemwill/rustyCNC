use crate::vector::Vector;
use std::fmt;

#[derive(Copy, Clone)]
pub struct LinearMovement {
    pub start_position: Vector,
    pub target_position: Vector,
    pub start_feedrate: f32,
    pub end_feedrate: f32,
    pub target_feedrate: f32,
}

impl LinearMovement {
    pub(crate) fn new(target_position: Vector, target_feedrate: f32) -> LinearMovement {
        LinearMovement {
            start_position: Vector::default(),
            target_position: target_position,
            start_feedrate: target_feedrate,
            end_feedrate: 0.0,
            target_feedrate: target_feedrate,
        }
    }

    pub(crate) fn calculate_acceleration_decceleration(&mut self, max_acceleration: f32) {
        let distance = self.target_position.distance_to(Vector::default());
        let acceleration_distance = self.target_feedrate * self.target_feedrate / (2.0 * max_acceleration);
        if distance < 2.0 * acceleration_distance {
            let acceleration_time = f32::sqrt(distance / max_acceleration);
            self.start_feedrate = max_acceleration * acceleration_time;
            self.end_feedrate = self.start_feedrate;
        } else {
            self.start_feedrate = self.target_feedrate;
            self.end_feedrate = self.target_feedrate;
        }
    }
}

impl fmt::Display for LinearMovement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Target Pos: {}, Start FR: {}, Target FR: {}, End FR: {})",
            self.target_position, self.start_feedrate, self.target_feedrate, self.end_feedrate
        )
    }
}

impl Default for LinearMovement {
    fn default() -> Self {
        LinearMovement {
            target_position: Vector::default(),
            start_feedrate: 0.0,
            end_feedrate: 0.0,
            target_feedrate: 0.0,
        }
    }
}
