use std::{env, fs};
use std::path::Path;
use shellexpand::env_with_context_no_errors;

pub fn load_config_file(){
    let _ = dotenvy::from_filename(".env");
    let tmpl = fs::read_to_string("processor.yaml.tmpl").expect("Failed to read config template");

    let rendered = env_with_context_no_errors(&tmpl, |key|{
        Some(env::var(key).unwrap())
    });

    fs::write(Path::new("config.yaml"), rendered.as_ref()).expect("Failed to write config file");

    println!("Config file written to config.yaml");
}