use crate::vector::Vector;
use std::fmt;

#[derive(Copy, Clone)]
pub struct ArcMovement {
    pub clockwise: bool,
    pub start_position: Vector,
    pub end_position: Vector,
    pub center_offset: Vector,
    pub start_feedrate: f32,
    pub target_feedrate: f32,
    pub end_feedrate: f32,
}

impl ArcMovement {
    pub(crate) fn new(start_position: Vector, end_position: Vector, center_offset: Vector, clockwise: bool, target_feedrate: f32) -> ArcMovement {
        ArcMovement {
            clockwise: clockwise,
            start_position: start_position,
            end_position: end_position,
            center_offset: center_offset,
            start_feedrate: target_feedrate,
            end_feedrate: 0.0,
            target_feedrate: target_feedrate,
        }
    }
    // Calculate the center of the arc
    pub(crate) fn center(&self, start_position: Vector) -> Vector {
        let center = start_position + self.center_offset;

        return center;
    }

    // Calculate angle of arc
    pub(crate) fn angle_rad(&self) -> f32 {
        let center_to_end_position_vector = self.end_position - self.center(self.start_position);
        let angle = self.center_offset.angle_with(center_to_end_position_vector);

        return angle;
    }

    pub(crate) fn start_direction(&self) -> Vector {
        let start_direction = self.center_offset.rotate_90_degrees(self.clockwise);

        return start_direction;
    }

    pub(crate) fn radius(&self) -> f32 {
        let radius = self.center_offset.length();

        return radius;
    }

    // calculate length of arc
    pub(crate) fn length(&self) -> f32 {
        let angle = self.angle_rad();
        let length = self.radius() * angle;

        return length;
    }
}

impl fmt::Display for ArcMovement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ArcMovement {{ start_position: {}, end_position: {}, center_offset: {}, start_feedrate: {}, target_feedrate: {}, end_feedrate: {} }}",
            self.start_position, self.end_position, self.center_offset, self.start_feedrate, self.target_feedrate, self.end_feedrate
        )
    }
}
