use std::cmp::{max, min};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

extern crate colored;
use colored::*;

use crate::languages::Language;

use super::languages::search;

pub fn run_tests(lang: Language, path: &PathBuf) -> Result<(), String> {
    let inputs = search(".in", path);
    let outputs = search(".out", path);

    for (input, output) in inputs.into_iter().zip(outputs.into_iter()) {
        match run_test(&lang, input, output, path) {
            Ok(_) => (),
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}

pub fn run_test(
    lang: &Language,
    input: String,
    output: String,
    path: &PathBuf,
) -> Result<(), String> {
    let mut input = File::open(input).unwrap();
    let mut content = Vec::new();

    match input.read_to_end(&mut content) {
        Ok(_) => (),
        Err(e) => {
            return Err(e.to_string());
        }
    }

    let content: &[u8] = content.as_slice();

    let r = lang.run(content, path)?;

    match r {
        Ok(out) => {
            check_out(output, out)?;
        }
        Err(e) => {
            println!("{}: {}", "Error:\n".red(), e);
        }
    };

    Ok(())
}

fn compare(output: &Vec<&str>, out: &Vec<&str>) -> bool {
    if output.len() != out.len() {
        return false;
    }

    for (s1, s2) in output.into_iter().zip(out.into_iter()) {
        if s1 != s2 {
            return false;
        }
    }

    return true;
}

fn check_out(output: String, out: String) -> Result<(), String> {
    let mut output = File::open(output).unwrap();

    let mut content = String::new();
    match output.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => {
            return Err(e.to_string());
        }
    }

    let output: Vec<_> = content.split("\n").collect();
    let out: Vec<_> = out.split("\n").collect();

    let output: Vec<_> = output.into_iter().map(|s| s.trim_end()).collect();
    let out: Vec<_> = out.into_iter().map(|s| s.trim_end()).collect();

    if compare(&output, &out) {
        println!("{}", "OK".green());
    } else {
        println!("{}", "Failed:\n".red());
        analyze_failed(&output, &out);
    }

    Ok(())
}

fn analyze_failed(output: &Vec<&str>, out: &Vec<&str>) {
    let mut output: Vec<_> = output.clone();
    let mut out: Vec<_> = out.clone();

    let dif = max(output.len(), out.len()) - min(output.len(), out.len());
    output.extend(vec![""; dif]);
    out.extend(vec![""; dif]);

    for (s1, s2) in output.into_iter().zip(out.into_iter()) {
        if s1 == s2 {
            println!("{}", s1);
        } else {
            println!("{}", s1.green());
            println!("{}", s2.red());
        }
    }
}
