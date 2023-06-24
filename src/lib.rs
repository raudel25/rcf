mod test_cases;

use test_cases::get_test_cases;

pub fn clone_problem(contest_id: i32, problem_index: String) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(get_test_cases(contest_id, problem_index))?;

    Ok(())
}
