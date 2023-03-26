use crate::machine_configuration::MachineConfiguration;
use crate::movement::LinearMovement;
use crate::vector::Vector;
use std::collections::VecDeque;

pub struct MotionPlanner {
    //current_position: Vector,
    target_queue: VecDeque<LinearMovement>,
    machine_configuration: MachineConfiguration,
}

impl MotionPlanner {
    pub fn new(machine_configuration: MachineConfiguration) -> MotionPlanner {
        MotionPlanner {
            //current_position: Vector { x: 0.0, y: 0.0, z: 0.0 },
            target_queue: VecDeque::new(),
            machine_configuration: machine_configuration,
        }
    }

    // Explained here: https://onehossshay.wordpress.com/2011/09/24/improving_grbl_cornering_algorithm/
    fn calculate_radius(&mut self, angle: f32) -> f32 {
        let radius = self.machine_configuration.path_deviation_tolerance * f32::sin(angle / 2.0) / (1.0 - f32::sin(angle / 2.0));
        return radius;
    }

    fn get_max_cornering_velocity(&mut self, move_a: Vector, move_b: Vector) -> f32 {
        let angle = std::f32::consts::PI - move_a.angle_with(move_b);

        let radius = self.calculate_radius(angle);

        let mut max_acceleration_xy = self.machine_configuration.max_acceleration.x;

        // TODO: Fix ================================================================
        if self.machine_configuration.max_acceleration.y < max_acceleration_xy {
            max_acceleration_xy = self.machine_configuration.max_acceleration.y;
        }

        let max_cornering_velocity = f32::sqrt(max_acceleration_xy * radius);
        return max_cornering_velocity;
    }

    pub fn get_next(&mut self, index: usize) -> Option<LinearMovement> {
        // Set the next target position while taking into account the special case of the last move
        let mut next_move = LinearMovement {
            target_position: (Vector { x: 0.0, y: 0.0, z: 0.0 }),
            start_feedrate: (0.0),
            target_feedrate: (0.0),
            end_feedrate: (0.0),
        };
        if index + 1 >= self.target_queue.len() {
            next_move.target_position = Vector { x: 0.0, y: 0.0, z: 0.0 };
            next_move.target_feedrate = 0.0;
            next_move.start_feedrate = 0.0;
        } else {
            if let Some(item_next) = self.target_queue.get(index + 1) {
                next_move = *item_next;
            } else {
                println!("Error, no elements left in target_queue for index+1");
                return None;
            }
        }
        return Some(next_move);
    }

    pub fn traverse(&mut self, index: usize) {
        // Set previous position while taking into account the special case of the first move
        let previous_position: Vector;
        if index == 0 {
            previous_position = Vector { x: 0.0, y: 0.0, z: 0.0 };
        } else if let Some(item_previous) = self.target_queue.get(index - 1) {
            previous_position = item_previous.target_position;
        } else {
            return;
        }

        // Get the current move
        let current_move: LinearMovement;
        if let Some(item_current) = self.target_queue.get(index) {
            current_move = *item_current;
        } else {
            return;
        }

        // Set the next target position while taking into account the special case of the last move
        let next_move: LinearMovement;
        if let Some(item_current) = self.get_next(index) {
            next_move = item_current;
        } else {
            return;
        }

        // Calculate the vectors for the current and next move
        let current_move_vector = current_move.target_position - previous_position;
        let next_move_vector = next_move.target_position - current_move.target_position;

        // Calculate the maximum velocity at the corner
        let max_corner_velocity = self.get_max_cornering_velocity(current_move_vector, next_move_vector);

        // Verify the maximum velocity at the end of the current move
        // v2 = u2 + 2ad
        // v2 is the final speed, u2 the initial speed
        let v_initial = current_move.start_feedrate;
        let distance = current_move_vector.length();
        let acceleration = self.machine_configuration.get_max_acceleration(current_move.target_position);
        let max_end_feedrate = f32::sqrt(f32::powi(v_initial, 2) + acceleration * distance);

        // Calculate the maximum velocity at the end of the current move
        let mut max_junction_velocity = current_move.target_feedrate;
        max_junction_velocity = f32::min(max_junction_velocity, next_move.start_feedrate);
        max_junction_velocity = f32::min(max_junction_velocity, next_move.target_feedrate);
        max_junction_velocity = f32::min(max_junction_velocity, max_corner_velocity);
        max_junction_velocity = f32::min(max_junction_velocity, max_end_feedrate);

        // Calculate the maximum velocity at the start of the next move
        // v2 = u2 + 2ad
        // v2 is the final speed, u2 the innitial speed and
        let v_initial = max_junction_velocity;
        let distance = current_move_vector.length();
        let acceleration = self.machine_configuration.get_max_acceleration(current_move.target_position);
        let max_start_feedrate = f32::sqrt(f32::powi(v_initial, 2) + acceleration * distance);

        // Print debug information
        print!(
            "[{}] Corner feedrate {:5.3} (end: {:5.3}, target: {:5.3}, max {:5.3}, next target {:5.3}, cornering {:5.3}). ",
            index,
            max_junction_velocity,
            next_move.start_feedrate,
            current_move.target_feedrate,
            max_end_feedrate,
            next_move.target_feedrate,
            max_corner_velocity
        );

        // Lookahead buffer
        if max_start_feedrate < current_move.start_feedrate {
            let current_move_mut = self.target_queue.get_mut(index).unwrap();
            current_move_mut.start_feedrate = max_start_feedrate;
            println!(
                "Reduced start feedrate from {:5.3} to {:5.3}. Recalculating. ",
                current_move.start_feedrate, max_start_feedrate
            );
            self.traverse(index - 1);
            return;
        }

        // Store end velocity
        if index + 1 < self.target_queue.len() {
            let next_move_mut = self.target_queue.get_mut(index + 1).unwrap();
            next_move_mut.start_feedrate = max_junction_velocity;
            let current_move_mut = self.target_queue.get_mut(index).unwrap();
            current_move_mut.end_feedrate = max_junction_velocity;

            println!(
                "Changed next start feedrate from {:5.3} to {:5.3}. Stepping to next.",
                next_move.start_feedrate, max_junction_velocity
            );
            self.traverse(index + 1);
        } else {
            println!("");
            println!("Finished motion planning.");
            println!("---------------------------------");
            return;
        }
    }

    pub fn run(&mut self) {
        match self.target_queue.get_mut(0) {
            Some(first_move) => {
                first_move.start_feedrate = 0.0;
            }
            None => {
                println!("Error, no elements in target_queue");
                return;
            }
        }
        println!("Starting motion planning.");
        self.traverse(0);
    }

    pub fn calculate_feedrate_profile(&mut self) {
        println!("Calculating motion profile.");
        // Calculate the feedrate profile for each move
        for (index, item) in self.target_queue.iter().enumerate() {
            // Calculate the feedrate profile for the current move
            println!("[{}] item: {}", index, item);
        }
        println!("Done calculating motion profile.");
        println!("---------------------------------");
    }

    pub fn add_target(&mut self, target: LinearMovement) {
        self.target_queue.push_back(target);
    }
    /*
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
        let time_step_vector = vector_to_target * step_fraction;

        // Verify if we are close enough to the target
        if distance_to_next_segment < step_distance {
            return (current_position, true);
        }

        let new_current_position = current_position + time_step_vector;

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
    }*/
}
