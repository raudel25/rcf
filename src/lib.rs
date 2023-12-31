use std::path::PathBuf;

pub mod config;
mod contest;
pub mod languages;
mod system;
mod test_cases;
mod tester;

use config::language_by_name;
use contest::get_problems;
use languages::{get_language, Language};
use test_cases::{get_test_cases, TestCase};
use tester::run_tests;

extern crate colored;
use colored::*;

pub fn error(e: String) {
    eprintln!("{} {}", "rsh:".red(), e);
}

pub fn clone_problem(contest_id: i32, problem_index: &str, language: &Language, path: &PathBuf) {
    match get_test_cases(contest_id, problem_index) {
        Ok(test_cases) => create_test_cases(test_cases, path),
        Err(e) => error(e),
    };

    match language.create_config(path) {
        Ok(_) => (),
        Err(e) => error(e.to_string()),
    };

    match language.create_source(path) {
        Ok(_) => (),
        Err(e) => error(e.to_string()),
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
        Err(e) => error(e.to_string()),
    };
}

pub fn test(path: &PathBuf) {
    let language = match get_language(path) {
        Ok(s) => match language_by_name(s) {
            Ok(l) => l,
            Err(e) => {
                error(e);
                return;
            }
        },
        Err(e) => {
            error(e);
            return;
        }
    };

    match run_tests(language, path) {
        Ok(_) => (),
        Err(e) => error(e),
    };
}

fn create_test_cases(test_cases: Vec<TestCase>, path: &PathBuf) {
    for test_case in test_cases {
        match test_case.create(path) {
            Ok(_) => (),
            Err(e) => error(e.to_string()),
        }
    }
}
