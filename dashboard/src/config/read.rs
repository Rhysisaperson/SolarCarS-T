use std::{fs::{self, File}, io::Write};

use serde_json::Error;

use super::structs::Config;

const DEFAULT_CONFIG: &[u8] = include_bytes!("config.json");

pub fn read_config() -> Option<Config> {
    let mut contents = fs::read_to_string("./config.json");

    if contents.is_err() {
        let mut file = File::create("./config.json").unwrap();
        file.write(DEFAULT_CONFIG).unwrap();
        contents = fs::read_to_string("./config.json");
    }

    let contents = contents.unwrap();

    let json_output: Result<Config, Error> = serde_json::from_str(&contents);

    return Some(json_output.unwrap());
    // if json_output.is_ok() {
        
    // }


    //return None;
}