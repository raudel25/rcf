mod contest;
mod languages;
mod test_cases;
mod tester;

use contest::get_problems;
use languages::{
    check_source_file, compiler_extension, create_config, create_source, get_language,
    language_by_name,
};
use test_cases::{get_test_cases, TestCase};
use tester::run_tests;

pub fn clone_problem(contest_id: i32, problem_index: &str, language: &str, path: &str) {
    match get_test_cases(contest_id, problem_index) {
        Ok(test_cases) => create_test_cases(test_cases, path),
        Err(e) => eprintln!("{}", e),
    };

    let lang = match language_by_name(language) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    match create_config(path, language) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    };

    match create_source(path, compiler_extension(lang).1) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    };
}

pub fn clone_contest(contest_id: i32, language: &str, path: &str) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    match runtime.block_on(get_problems(contest_id)) {
        Ok(problems) => {
            for index in problems {
                let path = format!("{}/{}", path, index.to_lowercase());
                clone_problem(contest_id, &index, language, &path);
            }
        }
        Err(e) => eprintln!("{}", e.to_string()),
    };
}

pub fn test(path: &str) {
    let language = match get_language(path) {
        Ok(s) => match language_by_name(&s) {
            Ok(s) => s,
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

    let (compiler, extension) = compiler_extension(language);

    let file = check_source_file(extension, path);

    match file {
        Ok(file) => match run_tests(&format!("{} {}", compiler, file), path) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{}", e),
    }
}

fn create_test_cases(test_cases: Vec<TestCase>, path: &str) {
    for test_case in test_cases {
        match test_case.create(path) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e.to_string()),
        }
    }
}
