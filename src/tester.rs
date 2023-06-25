use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

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
            let check = check_out(output, out)?;

            if check {
                println!("OK");
            } else {
                println!("Failed");
            }
        }
        Err(e) => {
            println!("Error {}", e);
        }
    };

    Ok(())
}

fn check_out(output: String, out: String) -> Result<bool, String> {
    let mut output = File::open(output).unwrap();

    let mut content = String::new();
    match output.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => {
            return Err(e.to_string());
        }
    }

    Ok(content == out)
}
