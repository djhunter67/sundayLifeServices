use std::{env, fs, path::Path};

fn main() {
    // Load the .env file to build the scripts environment
    dotenvy::dotenvy().expect("Failed to load the .env file");

    // Retrieve the specific secret
    let secret_val = env::var("DATABASE_PW").expect("Unable to find or parse the .env file");

    let dest_path = String::from("./static/base.yaml");

    // Define the output path in the build directory
    let out_dir = env::var("OUT_DIR").unwrap();

    // Write the secret to the yaml file
    fs::write(&dest_path, secret_val).expect("Failure to write the secret to the file");

    // Rerun the build if the .env file changes
    println!("cargo:rerun-if-changed=.env");
}
