use std::{env, fs};

/// This will change the file multiple times per build.
/// This build.rs file will work in tandem with the commit
/// and push hooks to change the secret being injected here.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the .env file to build the scripts environment
    dotenvy::dotenv().expect("Failed to load the .env file");

    // Retrieve the specific secret
    let secret_val = env::var("DATABASE_PW").expect("Unable to find or parse the .env file");

    let dest_path = String::from("./settings/base.yaml");

    // Define the output path in the build directory
    let _out_dir = env::var("OUT_DIR").expect("The OUT_DIR is unknown");

    // Create the yaml content
    let yaml_content = format!(
        " uri: \"mongodb+srv://djhunter67:{secret_val}@devcluster.jbdh4mk.mongodb.net/?appName=devCluster\""
    );

    // Capture the contents of the yaml file
    // let _ = File::read_to_string(
    //     &mut File::open(&dest_path).expect("File not found"),
    //     &mut prev_contents,
    // )
    // .expect("Unable to convert the contents of the file");

    let prev_contents = fs::read_to_string(&dest_path)?;

    let mut new_content: Vec<&str> = Vec::with_capacity(prev_contents.len());
    let mut uri_count: u8 = 0;

    for line in prev_contents.lines() {
        if line.contains("uri:") {
            uri_count += 1;
            if line.contains("<password>") && !uri_count.gt(&1) {
                new_content.push(&yaml_content);
                continue;
            }
        }
        new_content.push(line);
    }

    let new_content: String = new_content.join("\n");

    // Write to the base.yaml file
    fs::write(&dest_path, new_content)?;

    // Rerun the build if the .env file changes
    println!("cargo:rerun-if-changed={dest_path}");

    Ok(())
}
