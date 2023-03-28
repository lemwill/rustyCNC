use crate::vector::Vector;
use std::fmt;

#[derive(Copy, Clone)]
pub struct LinearMovement {
    pub end_position: Vector,
    pub start_feedrate: f32,
    pub end_feedrate: f32,
    pub target_feedrate: f32,
}

impl LinearMovement {
    pub(crate) fn new(end_position: Vector, target_feedrate: f32) -> LinearMovement {
        LinearMovement {
            end_position: end_position,
            start_feedrate: target_feedrate,
            end_feedrate: 0.0,
            target_feedrate: target_feedrate,
        }
    }

    pub(crate) fn calculate_acceleration_decceleration(&mut self, start_position: Vector, acceleration: f32) -> Vector {
        let move_vector = self.end_position - start_position;

        // Calculate distance to accelerate to target feedrate
        let mut acceleration_distance = (self.target_feedrate.powi(2) - self.start_feedrate.powi(2)) / (2.0 * acceleration);

        // Calculate distance to deccelerate to end feedrate
        let mut decceleration_distance = (self.target_feedrate.powi(2) - self.end_feedrate.powi(2)) / (2.0 * acceleration);

        // Calculate distance to accelerate and deccelerate
        let acceleration_decceleration_distance = acceleration_distance + decceleration_distance;

        // Verify that this is not longer than the move
        if acceleration_decceleration_distance > move_vector.length() {
            // Calculate the new target feedrate, knowing the previous feedrate cannot be reached

            // v^2 = v_0^2 + 2ad
            let feedrate_delta_distance = f32::abs(self.end_feedrate.powi(2) - self.start_feedrate.powi(2)) / (2.0 * acceleration);

            let remaining_distance = f32::abs(move_vector.length() - feedrate_delta_distance) / 2.0;

            if self.end_feedrate > self.start_feedrate {
                acceleration_distance = remaining_distance + feedrate_delta_distance;
                decceleration_distance = remaining_distance;
            } else {
                acceleration_distance = remaining_distance;
                decceleration_distance = remaining_distance + feedrate_delta_distance;
            }

            let reduced_target_feedrate = (2.0 * acceleration * acceleration_distance + self.start_feedrate.powi(2)).sqrt();

            self.target_feedrate = reduced_target_feedrate;

            //print!(
            //    "Cannot reach full speed, reducing from {:6.3} to {:6.3}. ",
            //    self.target_feedrate, reduced_target_feedrate
            //);
        }

        let constant_acceleration_distance = move_vector.length() - acceleration_distance - decceleration_distance;

        println!(
            "Distance {:6.3}, acc dist {:6.3}, const dist {:6.3}, decc dist {:6.3}",
            move_vector.length(),
            acceleration_distance,
            constant_acceleration_distance,
            decceleration_distance
        );

        return self.end_position;
    }
}

impl fmt::Display for LinearMovement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Target Pos: {:6.3}, Start FR: {:6.3}, Target FR: {:6.3}, End FR: {:6.3})",
            self.end_position, self.start_feedrate, self.target_feedrate, self.end_feedrate
        )
    }
}

impl Default for LinearMovement {
    fn default() -> Self {
        LinearMovement {
            end_position: Vector::default(),
            start_feedrate: 0.0,
            end_feedrate: 0.0,
            target_feedrate: 0.0,
        }
    }
}
