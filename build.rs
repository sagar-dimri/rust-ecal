// use std::env;

// fn main() {
//     // This prints the OUT_DIR to help you verify where the code is being generated
//     println!("cargo:rerun-if-changed=proto_messages/message.proto");
//     println!("cargo:out-dir={}", env::var("OUT_DIR").unwrap_or_else(|_| "undefined".to_string()));
    
//     prost_build::compile_protos(&["proto_messages/message.proto"], &["proto_messages"]).unwrap();
// }

use std::env;

fn main() {
    // Print the OUT_DIR for debugging purposes (e.g., target/debug/build/.../out)
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-changed=proto_messages/message.proto");
    println!("cargo:out-dir={}", out_dir);

    // Compile the protobuf file
    prost_build::compile_protos(&["proto_messages/message.proto"], &["proto_messages"]).unwrap();
}
