use crate::vector::Vector;
use std::{collections::VecDeque};

pub struct MotionPlanner {
    current_position: Vector,
    target_queue: VecDeque<Vector>,

}

impl MotionPlanner {
    pub fn new() -> MotionPlanner {
        MotionPlanner {
            current_position: Vector { x: 0.0, y: 0.0, z: 0.0 },
            target_queue: VecDeque::new()
        }
    }

    pub fn run(&mut self) {
        let feedrate_config = Vector { x: 0.0, y: 0.0, z: 0.0 };
        while self.target_queue.len() > 0 {
            match self.target_queue.pop_front() {
                Some(displacement) =>  
                    self.current_position = self.step_to_next_target(self.current_position, displacement, 100.0, feedrate_config),
                None => println!("Empty queue"),
            }
        }
    }

    pub fn add_target(&mut self, target: Vector) {
        self.target_queue.push_back(target);
    }

    pub fn single_step(&self, current_position: Vector, target: Vector, feedrate: f32, max_feedrate: Vector) -> (Vector, bool) {

        // Step to feedrate
        // If it exceeds an axis feedrate, scale all feedrates
    
        // Distance is fixed
        // Feedrate is fixed
        // We need to calculate the duration of the step
        // Distance/Velocity (m/s) = Time duration (s)
    
        let step_distance = 0.1;
    
        // Calculate the vector to the target
        let vector_to_target = target - current_position; 
        
        // Calculate the distance to the next segment
        let distance_to_next_segment = vector_to_target.length();
        
        // Calculate the ratio between the step distance and the distance to the next segment
        let step_fraction = step_distance / distance_to_next_segment;
    
        // Calculate the vector for this timestep
        let time_step_vector  = vector_to_target * step_fraction;
    
        // Verify if we are close enough to the target
        if distance_to_next_segment < step_distance {
            return (current_position, true);
        }
    
        let new_current_position = current_position +  time_step_vector;
    
        return (new_current_position, false);
    
    }
    
    
    pub fn step_to_next_target(&self, current_position: Vector, target: Vector, feedrate: f32, max_feedrate: Vector) -> Vector {
    
        let mut new_current_position = current_position;
        let mut is_target_reached = false;
    
        while !is_target_reached {
            let (new_position, target_reached) = self.single_step(new_current_position, target, feedrate, max_feedrate);
            new_current_position = new_position;
            is_target_reached = target_reached;
            println!("Current position: {}", new_current_position);
        }
    
        return new_current_position;
    }
}
