mod test_cases;

use test_cases::{get_test_cases, TestCase};

pub fn clone_problem(contest_id: i32, problem_index: String, path: String) {
    match get_test_cases(contest_id, problem_index) {
        Ok(test_cases) => create_test_cases(test_cases, path),
        Err(e) => eprintln!("{}", e),
    }
}

fn create_test_cases(test_cases: Vec<TestCase>, path: String) {
    for test_case in test_cases {
        match test_case.create(&path) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e.to_string()),
        }
    }
}
