use rcf::{clone_contest, clone_problem, test};
use std::path::PathBuf;

fn main() {
    let path = &PathBuf::from("a");
    // clone_problem(1842, "A", "cpp", path);
    test(path);
}
