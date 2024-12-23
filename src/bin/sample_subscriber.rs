extern crate ecal;
extern crate prost;

use ecal::{Cal, Subscriber};
use std::{thread, time};

mod my_message {
    // Include the generated file from the .proto definition
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

use my_message::MyMessage;
use ecal::format::prost::Prost; // Correct import of Prost from ecal

// Implement ecal::Message for MyMessage
impl ecal::Message for MyMessage {
    fn type_name() -> &'static str {
        "MyMessage" // Return the type name
    }
}

fn main() {
    // Initialize eCAL. This is critical for any eCAL operations.
    let _cal = match Cal::new("rust_sample_subscriber") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error initializing eCAL: {}", e);
            return; // Exit if initialization fails
        }
    };

    // Initialize the subscriber
    let mut subscriber: Option<Subscriber<MyMessage, Prost<MyMessage>>> = None;
    match Subscriber::new("HelloTopic") {
        Ok(s) => subscriber = Some(s),
        Err(e) => {
            eprintln!("Unable to create subscriber for 'HelloTopic': {}", e);
            return; // Exit if creating the subscriber fails
        }
    }

    // Start listening for messages
    loop {
        if let Some(s) = &subscriber {
            // Receive a message
            match s.recv() {
                Ok(received_message) => {
                    // Successfully received a message
                    println!("Received message: {:?}", received_message);
                }
                Err(e) => {
                    // Handle errors in message reception
                    eprintln!("Error receiving message: {}", e);
                }
            }
        }

        // Sleep for a short duration to avoid tight loops
        thread::sleep(time::Duration::from_millis(100));
    }
}
