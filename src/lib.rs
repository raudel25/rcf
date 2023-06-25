use std::io::Read;
use std::{fs::File, path::PathBuf};

mod contest;
mod languages;
mod system;
mod test_cases;
mod tester;

use contest::get_problems;
use languages::{create_config, create_source, get_language, Config, Language};
use test_cases::{get_test_cases, TestCase};
use tester::run_tests;

pub fn clone_problem(contest_id: i32, problem_index: &str, language: &Language, path: &PathBuf) {
    match get_test_cases(contest_id, problem_index) {
        Ok(test_cases) => create_test_cases(test_cases, path),
        Err(e) => eprintln!("{}", e),
    };

    match create_config(path, &language.name) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    };

    match create_source(path, language) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    };
}

pub fn clone_contest(contest_id: i32, language: &Language, path: &PathBuf) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    match runtime.block_on(get_problems(contest_id)) {
        Ok(problems) => {
            for index in problems {
                let folder = PathBuf::from(index.to_lowercase());
                clone_problem(contest_id, &index, language, &path.join(folder));
            }
        }
        Err(e) => eprintln!("{}", e.to_string()),
    };
}

pub fn test(path: &PathBuf) {
    let language = match get_language(path) {
        Ok(s) => match language_by_name(s) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    match run_tests(language, path) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    };
}

fn create_test_cases(test_cases: Vec<TestCase>, path: &PathBuf) {
    for test_case in test_cases {
        match test_case.create(path) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e.to_string()),
        }
    }
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

    let config = Config { languages };

    config
}

pub fn get_config() -> Config {
    let path_config = PathBuf::from(".");
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
