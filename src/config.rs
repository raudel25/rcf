use std::io::prelude::*;
use std::io::Read;
use std::{fs::File, path::PathBuf};

use super::languages::Language;

extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub default: String,
    pub languages: Vec<Language>,
}

pub fn language_default() -> Result<Language, String> {
    let config = get_config();

    for l in config.languages {
        if l.name == config.default {
            return Ok(l);
        }
    }

    Err(String::from("Not fount language"))
}

pub fn language_by_name(language: String) -> Result<Language, String> {
    let config = get_config();

    for l in config.languages {
        if l.name == language {
            return Ok(l);
        }
    }

    Err(String::from("Not fount language"))
}

fn default_config() -> Config {
    let languages = vec![Language {
        name: String::from("cpp"),
        compiler: String::from("g++"),
        extension: String::from(".cpp"),
        source: Vec::new(),
        executable: true,
    }];

    let config = Config {
        default: String::from("cpp"),
        languages,
    };

    let config_json = serde_json::to_string(&config).unwrap();

    let file_name = PathBuf::from("rcf.json");
    let path = match get_home_user() {
        Ok(p) => PathBuf::from(p),
        Err(e) => {
            eprintln!("{}", e);
            return config;
        }
    };

    let mut file = match File::create(path.join(file_name)) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            return config;
        }
    };

    match file.write_all(config_json.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            return config;
        }
    };

    config
}

pub fn get_config() -> Config {
    let path_config = match get_home_user() {
        Ok(p) => PathBuf::from(p),
        Err(_) => {
            return get_config();
        }
    };

    let file_name = PathBuf::from("rcf.json");

    let mut file = match File::open(path_config.join(file_name)) {
        Ok(f) => f,
        Err(_) => {
            return default_config();
        }
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            return default_config();
        }
    };

    let config: Config = match serde_json::from_str(&content) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return default_config();
        }
    };

    config
}

fn get_home_user() -> Result<String, String> {
    use std::env;

    let home_dir = match env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
        Ok(path) => path,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    Ok(home_dir)
}
