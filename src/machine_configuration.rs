use crate::vector::Vector;
use std::fmt;

pub struct MachineConfiguration {
    pub max_acceleration: Vector,
    pub max_feedrate: Vector,
    pub path_deviation_tolerance: f32,
}

impl MachineConfiguration {
    pub fn get_max_acceleration(&self, _direction: Vector) -> f32 {
        // TODO: Include direction in calculation
        let mut max_acceleration_in_direction = f32::max(self.max_acceleration.x, self.max_acceleration.y);
        max_acceleration_in_direction = f32::max(max_acceleration_in_direction, self.max_acceleration.z);
        return max_acceleration_in_direction;
    }
}

impl fmt::Display for MachineConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Max acceleration: {}, Max feedrate {}, Path deviation tolerance: {})",
            self.max_acceleration, self.max_feedrate, self.path_deviation_tolerance
        )
    }
}
