use rcf::{clone_contest, clone_problem, languages::Language, test};
use std::env;
use std::path::PathBuf;

use rcf::config::{language_by_name, language_default};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Wrong parameters");
        return;
    }

    let language = match get_language() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    match args[1].as_str() {
        "contest" => contest(&args, language),
        "problem" => problem(&args, language),
        "test" => test_e(&args),
        _ => eprintln!("Command not found"),
    }
}

fn get_language() -> Result<Language, String> {
    match env::var("lang") {
        Ok(lang) => {
            let l = language_by_name(lang)?;
            Ok(l)
        }
        Err(_) => {
            let l = language_default()?;
            Ok(l)
        }
    }
}

fn contest(args: &Vec<String>, language: Language) {
    if args.len() != 4 && args.len() != 3 {
        eprintln!("Wrong parameters");
        return;
    }

    let contest_id = match args[2].parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Wrong parameters");
            return;
        }
    };

    let path = if args.len() == 4 {
        PathBuf::from(args[3].as_str())
    } else {
        PathBuf::from(".")
    };

    clone_contest(contest_id, &language, &path);
}

fn problem(args: &Vec<String>, language: Language) {
    if args.len() != 5 && args.len() != 4 {
        eprintln!("Wrong parameters");
        return;
    }

    let contest_id = match args[2].parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Wrong parameters");
            return;
        }
    };

    let path = if args.len() == 5 {
        PathBuf::from(args[4].as_str())
    } else {
        PathBuf::from(".")
    };

    clone_problem(contest_id, &args[3], &language, &path);
}

fn test_e(args: &Vec<String>) {
    if args.len() != 2 && args.len() != 3 {
        eprintln!("Wrong parameters");
        return;
    }

    let path = if args.len() == 3 {
        PathBuf::from(args[2].as_str())
    } else {
        PathBuf::from(".")
    };

    test(&path);
}
