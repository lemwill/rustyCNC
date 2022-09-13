
mod vector;
use vector::Vector;

mod motion_planner;
use motion_planner::MotionPlanner;


fn main() {

    let mut motion_planner = MotionPlanner::new();

    motion_planner.add_target(Vector { x: 1.0, y: 3.0, z: 4.0 });
    motion_planner.add_target(Vector { x: 0.0, y: 0.0, z: 0.0 });

    motion_planner.run();
} 