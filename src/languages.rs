use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::path::PathBuf;

pub enum Languages {
    C,
    Cpp,
    Python,
}

pub fn compiler_extension<'a>(language: Languages) -> (&'a str, &'a str) {
    match language {
        Languages::C => ("gcc -o main", ".c"),
        Languages::Cpp => ("gcc -o main", ".cpp"),
        Languages::Python => ("python3", ".py"),
    }
}

pub fn language_by_name(language: &str) -> Result<Languages, String> {
    match language {
        "python" => Ok(Languages::Python),
        "cpp" => Ok(Languages::Cpp),
        "c" => Ok(Languages::C),
        _ => Err(String::from("Not found language")),
    }
}

pub fn check_source_file(extension: &str, path: &PathBuf) -> Result<String, String> {
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

pub fn create_config(path: &PathBuf, language: &str) -> std::io::Result<()> {
    if !fs::metadata(&path).is_ok() {
        fs::create_dir_all(&path)?;
    }

    let file_name = PathBuf::from("config.json");
    let mut config = File::create(path.join(file_name))?;

    let mut aux = String::from("{ ");
    aux.push_str(format!("\"language\":\"{}\"", language).as_str());
    aux.push_str(" }");

    config.write_all(aux.as_bytes())?;

    Ok(())
}

pub fn create_source(path: &PathBuf, extension: &str) -> std::io::Result<()> {
    if !fs::metadata(&path).is_ok() {
        fs::create_dir_all(&path)?;
    }

    let file_name = PathBuf::from(format!("main.{}", extension));
    File::create(path.join(file_name))?;

    Ok(())
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
