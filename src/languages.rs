use super::system::{compile, run};
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    pub name: String,
    pub compiler: String,
    pub extension: String,
    pub source: Vec<String>,
    pub executable: bool,
}

impl Language {
    pub fn run(&self, content: &[u8], path: &PathBuf) -> Result<Result<String, String>, String> {
        let file = check_source_file(self.extension.as_str(), path)?;

        if self.executable {
            let command = format!("{} -o main {}", self.compiler, file);

            let c = compile(command.as_str())?;

            match c {
                Ok(_) => {
                    let command_run = "./main";

                    run(command_run, content)
                }
                Err(e) => Ok(Err(e)),
            }
        } else {
            let command = format!("{} {}", self.compiler, file);
            run(command.as_str(), content)
        }
    }

    fn source_code(&self) -> String {
        let mut s = String::new();

        for i in &self.source {
            s.push_str(i.as_str());
            s.push('\n');
        }

        s
    }

    pub fn create_config(&self, path: &PathBuf) -> std::io::Result<()> {
        if !fs::metadata(&path).is_ok() {
            fs::create_dir_all(&path)?;
        }

        let file_name = PathBuf::from("config.json");
        let mut config = File::create(path.join(file_name))?;

        let mut aux = String::from("{ ");
        aux.push_str(format!("\"language\":\"{}\"", self.name).as_str());
        aux.push_str(" }");

        config.write_all(aux.as_bytes())?;

        Ok(())
    }

    pub fn create_source(&self, path: &PathBuf) -> std::io::Result<()> {
        if !fs::metadata(&path).is_ok() {
            fs::create_dir_all(&path)?;
        }

        let file_name = PathBuf::from(format!("main{}", self.extension));
        let mut source = File::create(path.join(file_name))?;

        source.write_all(self.source_code().as_bytes())?;

        Ok(())
    }
}

fn check_source_file(extension: &str, path: &PathBuf) -> Result<String, String> {
    let files = search(extension, path);

    if files.len() != 1 {
        return Err(String::from(
            "The source file do not have the correct format",
        ));
    }

    Ok(files[0].clone())
}

pub fn get_language(path: &PathBuf) -> Result<String, String> {
    let file_name = PathBuf::from("config.json");
    let mut config = match File::open(path.join(file_name)) {
        Ok(c) => c,
        Err(_) => {
            return Err(String::from("Not found language"));
        }
    };

    let mut content = String::new();
    match config.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let config: Value = match serde_json::from_str(&content) {
        Ok(s) => s,
        Err(_) => {
            return Err(String::from("Not found language"));
        }
    };

    match config["language"].as_str() {
        Some(s) => Ok(String::from(s)),
        None => Err(String::from("Not found language")),
    }
}

pub fn search(extension: &str, path: &PathBuf) -> Vec<String> {
    let files: Vec<_> = fs::read_dir(path)
        .unwrap()
        .filter_map(|entry_res| {
            let entry = entry_res.unwrap();
            let file_name_buf = entry.file_name();
            let file_name = file_name_buf.to_str().unwrap();
            if !file_name.starts_with(".") && file_name.ends_with(extension) {
                Some(entry.path())
            } else {
                None
            }
        })
        .collect();

    files
        .into_iter()
        .map(|p| String::from(p.to_str().unwrap()))
        .collect()
}
