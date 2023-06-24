mod contest;
mod test_cases;

use contest::get_problems;
use test_cases::{get_test_cases, TestCase};

pub fn clone_problem(contest_id: i32, problem_index: &str, path: &str) {
    match get_test_cases(contest_id, problem_index) {
        Ok(test_cases) => create_test_cases(test_cases, path),
        Err(e) => eprintln!("{}", e),
    }
}

pub fn clone_contest(contest_id: i32, path: &str) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    match runtime.block_on(get_problems(contest_id)) {
        Ok(problems) => {
            for index in problems {
                let path = format!("{}/{}", path, index);
                clone_problem(contest_id, &index, &path);
            }
        }
        Err(e) => eprintln!("{}", e.to_string()),
    };
}

fn create_test_cases(test_cases: Vec<TestCase>, path: &str) {
    for test_case in test_cases {
        match test_case.create(path) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e.to_string()),
        }
    }
}
