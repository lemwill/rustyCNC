use std::{collections::VecDeque};

mod coordinates;
use coordinates::Coordinates;

mod segment;
use segment::Segment;


fn step(current_position: Coordinates, target: Coordinates, feedrate: f32, max_feedrate: Coordinates) -> Coordinates {

    // Step to feedrate
    // If it exceeds an axis feedrate, scale all feedrates
    let time = 1.0;
    let distance = target - current_position; 
    let calculated_feedrate = distance / time;
    let calculated_feedrate_ratio = feedrate / calculated_feedrate.length();
    let new_current_position = current_position +  calculated_feedrate * calculated_feedrate_ratio;
    return new_current_position;
}

fn main() {

    let mut current_position = Coordinates { x: 0.0, y: 0.0, z: 0.0 };

    let displacement = Coordinates { x: 1.0, y: 2.0, z: 3.0 };

    let feedrate = Coordinates { x: 1.0, y: 1.0, z: 1.0 };

    for x in 0..10 {
        current_position = step(current_position, displacement, 1.0, feedrate);
        println!("New current position: {}", current_position);
    }



    /*let x2 = Coordinates { x: 2.0, y: 3.0, z: 2.0 };



    let segment = Segment::new(x1, x2, 100.0);

    println!("segment: {}", segment);
    println!("segment length: {}", segment.length());

    let coordinates_diff = &x2 - &x1;
    println!("Coordinates diff: {}", coordinates_diff);

    let mut deque: VecDeque<Coordinates> = VecDeque::new();
    deque.push_back(Coordinates { x: 4.0, y: 5.0, z: 6.0 });

    while deque.len() > 0 {
        match deque.pop_front() {
            Some(coordinates) => println!("Coordinates: {}", coordinates),
            None => println!("No coordinates"),
        }
    }*/
} 