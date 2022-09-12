use std::collections::VecDeque;


mod coordinates;
use coordinates::Coordinates;

mod segment;
use segment::Segment;



fn main() {
    let spindle_coordinates = Coordinates { x: 1.0, y: 2.0, z: 3.0 };

    let x1 = Coordinates { x: 1.0, y: 2.0, z: 3.0 };
    let x2 = Coordinates { x: 2.0, y: 3.0, z: 2.0 };

    let segment = Segment::new(x1, x2, 100.0);

    println!("segment: {}", segment);
    println!("segment length: {}", segment.length());

    let coordinates_diff = &x2 - &x1;
    println!("Coordinates diff: {}", coordinates_diff);

    let mut deque: VecDeque<Coordinates> = VecDeque::new();
    deque.push_back(spindle_coordinates);
    deque.push_back(Coordinates { x: 4.0, y: 5.0, z: 6.0 });

    while deque.len() > 0 {
        match deque.pop_front() {
            Some(coordinates) => println!("Coordinates: {}", coordinates),
            None => println!("No coordinates"),
        }
    }
} 