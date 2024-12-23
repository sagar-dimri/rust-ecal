extern crate ecal;
extern crate prost;

use ecal::{Cal, Publisher};
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
    // if let Err(e) = ecal::initialize("rust_sample_publisher", ecal::InitializationFlags::empty()) {
    let _cal = match Cal::new("rust_sample_publisher") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error initializing eCAL: {}", e);
            return; // Exit if initialization fails
        }
    };

    // Initialize the publisher
    let mut publisher: Option<Publisher<MyMessage, Prost<MyMessage>>> = None;
    match Publisher::new("HelloTopic") {
        Ok(p) => publisher = Some(p),
        Err(e) => {
            eprintln!("Unable to create publisher for 'HelloTopic': {}", e);
            return; // Exit if creating the publisher fails
        }
    }

    let mut counter: u64 = 0;
    // Create a new message
    let mut message = MyMessage {
        content: "Hello from Rust Publisher!".to_string(),
        id: counter
    };

    // Periodically send the message
    loop {
        counter += 1;
        message.id = counter;
        if let Some(p) = &publisher {
            // Send the message directly; eCAL handles serialization
            if let Err(e) = p.send(&message) {
                eprintln!("Error sending message: {}", e);
            } else {
                println!("Message sent: {:?}", message);
            }
        }

        // Sleep for 1 second before sending the next message
        thread::sleep(time::Duration::from_secs(1));
    }
}
