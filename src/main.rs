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

    match args[1].as_str() {
        "contest" => contest(&args),
        "problem" => problem(&args),
        "test" => test_e(&args),
        _ => eprintln!("Command not found"),
    }
}

fn get_language(arg: &str) -> Result<Language, String> {
    if arg == "" {
        let l = language_default()?;
        Ok(l)
    } else {
        let l = language_by_name(arg.to_string())?;
        Ok(l)
    }
}

fn contest(args: &Vec<String>) {
    if args.len() < 3 {
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

    let path = if args.len() >= 4 {
        PathBuf::from(args[3].as_str())
    } else {
        PathBuf::from(".")
    };

    let language = if args.len() >= 5 {
        get_language(args[4].as_str())
    } else {
        get_language("")
    };

    let language = match language {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    clone_contest(contest_id, &language, &path);
}

fn problem(args: &Vec<String>) {
    if args.len() < 4 {
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

    let path = if args.len() >= 5 {
        PathBuf::from(args[4].as_str())
    } else {
        PathBuf::from(".")
    };

    let language = if args.len() >= 6 {
        get_language(args[5].as_str())
    } else {
        get_language("")
    };

    let language = match language {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    clone_problem(contest_id, &args[3], &language, &path);
}

fn test_e(args: &Vec<String>) {
    if args.len() < 2 {
        eprintln!("Wrong parameters");
        return;
    }

    let path = if args.len() >= 3 {
        PathBuf::from(args[2].as_str())
    } else {
        PathBuf::from(".")
    };

    test(&path);
}
