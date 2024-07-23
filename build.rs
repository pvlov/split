use core::panic;
use std::{env, error::Error, fs, process::Command};

fn generate_openapi() -> Result<(), Box<dyn Error>> {
    let input_dir = env::var("OPENAPI_INPUT_DIR")?;
    let output_dir = env::var("OPENAPI_OUTPUT_DIR")?;

    fs::create_dir_all(output_dir.clone()).expect("Failed to create output directory");

    for entry in fs::read_dir(input_dir.clone()).expect("Failed to read input directory") {
        let path = entry?.path();
        let path_ext = path.extension().and_then(|ext| ext.to_str());

        if path_ext == Some("yaml") || path_ext == Some("yml") {
            let output = Command::new("openapi-generator")
                .arg("generate")
                .arg("-i") // input file
                .arg(path)
                .arg("-g")
                .arg("rust")
                .arg("--global-property")
                .arg("models")
                .arg("-o")
                .arg(output_dir.clone())
                .output()
                .expect("Failed to run openapi-generator");

            if !output.status.success() {
                panic!("Failed to generate models");
            }
        }
    }
    println!("cargo:rerun-if-changed={}", input_dir.clone());

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().or(dotenv::from_filename("template.env"))?;
    generate_openapi()?;

    Ok(())
}
