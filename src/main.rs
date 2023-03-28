mod linear_movement;
use linear_movement::LinearMovement;

mod arc_movement;
use arc_movement::ArcMovement;

mod machine_configuration;
use machine_configuration::MachineConfiguration;

mod vector;
use vector::Vector;

mod motion_planner;
use motion_planner::MotionPlanner;

mod motor_step_generator;

mod utilities;

fn main() {
    let machine_configuration = MachineConfiguration {
        max_acceleration: Vector {
            x: (10.0),
            y: (10.0),
            z: (10.0),
        },
        max_feedrate: Vector {
            x: (10.0),
            y: (10.0),
            z: (10.0),
        },
        path_deviation_tolerance: 0.1,
    };

    // Allow for arcs
    // Plot the feedrate
    // Integrate other types of commands
    // Integrate arcs
    // Interpolate to steps
    // Remove recursion
    // Select the correct acceleration when calculating the corner velocity
    // Add machine limits

    let mut motion_planner = MotionPlanner::new(machine_configuration);

    //let a = LinearMovement::new(Vector { x: 10.0, y: 10.0, z: 0.0 }, 4.0);

    // Straight line from (0,0,0)
    let b: LinearMovement = LinearMovement::new(Vector { x: 12.0, y: 12.0, z: 0.0 }, 10.0);

    // 90 degrees
    let c: LinearMovement = LinearMovement::new(Vector { x: 4.0, y: 4.0, z: 0.0 }, 10.0);

    // 180 degrees
    let d: LinearMovement = LinearMovement::new(Vector { x: 0.0, y: 0.0, z: 0.0 }, 10.0);

    let e: ArcMovement = ArcMovement::new(
        Vector { x: 0.0, y: 0.0, z: 0.0 },
        Vector { x: 0.0, y: 0.0, z: 0.0 },
        Vector { x: 1.0, y: 0.0, z: 0.0 },
        true,
        10.0,
    );

    println!("Start direction: {}", e.start_direction());

    //motion_planner.add_target(a);
    motion_planner.add_target(b);
    motion_planner.add_target(c);
    motion_planner.add_target(d);

    motion_planner.run();

    motion_planner.calculate_feedrate_profile();

    //let mut motor_step_generator = MotorStepGenerator::new();
}

/*

#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, Ticker};
use log::*;



fn foo(test: &mut embassy_time::Instant, count:&mut i64, jitter: &mut f32) {
    let y = embassy_time::Instant::now() - *test ;
    let z = (y.as_ticks() as i64);
    //info!("instant => {:?}", z-100000);
    *test = embassy_time::Instant::now();

    let error = f32::powf(z as f32 - 100000 as f32,2.0);
    if (z > 0) {
        *jitter = *jitter +  error;
        *count +=  1;
    }
    //info!("tick");
}

#[embassy_executor::task]
async fn run() {
    info!("Starting execution");
    let mut ticker = Ticker::every(Duration::from_millis(100));

    let mut test:embassy_time::Instant = embassy_time::Instant::now();
    let mut count:i64 = 0;
    let mut jitter:f32 = 0.0;

    loop {
        foo(&mut test, &mut count, &mut jitter);
        ticker.next().await;
        info!("jitter => {:?}", f32::sqrt(jitter / count as f32))

    }
}


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();

    info!("Starting  main");
    spawner.spawn(run()).unwrap();


}*/
/*

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));



use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use protobuf::{EnumOrUnknown, Message};



use example::{Person};

fn main() -> std::io::Result<()> {

    let mut out_msg = Person::new();
    out_msg.name = "John Smith".to_string();
    out_msg.age = 25;
    println!("Message request:\nout_msg {:#?}", out_msg);
    let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap();


    // Bind to TCP port
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    // Accept a connection
    let (mut socket, _) = listener.accept()?;

    // Receive a message
    let mut size_bytes = [0u8; 4];
    socket.read_exact(&mut size_bytes)?;
    let size = u32::from_be_bytes(size_bytes);
    let mut data = vec![0u8; size as usize];
    socket.read_exact(&mut data)?;
    let received_message = Person::parse_from_bytes(&out_bytes).unwrap();
    println!("Received message: {:?}", received_message);

    // Send a message
    let mut message = Person::new();
    message.name = "John Smith".to_string();
    message.age = 25;
    let message_bytes = message.write_to_bytes().unwrap();
    let message_size = message_bytes.len() as u32;
    let size_bytes = message_size.to_be_bytes();
    socket.write_all(&size_bytes)?;
    socket.write_all(&message_bytes)?;

    // Receive another message
    let mut size_bytes = [0u8; 4];
    socket.read_exact(&mut size_bytes)?;
    let size = u32::from_be_bytes(size_bytes);
    let mut data = vec![0u8; size as usize];
    socket.read_exact(&mut data)?;
    let received_message2 = Person::parse_from_bytes(&out_bytes).unwrap();
    println!("Received message: {:?}", received_message2);

    // Send another message
    let mut message2 = Person::new();
    message2.name = "John Smith".to_string();
    message2.age = 25;
    let message2_bytes = message.write_to_bytes().unwrap();
    let message2_size = message2_bytes.len() as u32;
    let size2_bytes = message2_size.to_be_bytes();
    socket.write_all(&size2_bytes)?;
    socket.write_all(&message2_bytes)?;

    // Close the socket
    socket.shutdown(std::net::Shutdown::Both)?;
    Ok(())
}
*/
/*
use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::Message;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut websocket = accept_async(stream).await.unwrap();
            println!("New WebSocket connection: {}", addr);

            loop {
                let message = match websocket.read_message().await {
                    Ok(msg) => msg,
                    Err(_) => break,
                };

                match message {
                    Message::Text(msg) => {
                        println!("Received message: {}", msg);
                        websocket.write_message(Message::text(msg)).await.unwrap();
                        println!("Sent message: {}", msg);
                    }
                    _ => {}
                }
            }
        });
    }
}*/
