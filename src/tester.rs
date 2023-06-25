use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process::{Command, Stdio};

use super::languages::search;

pub fn run_tests(command: &str, path: &str) -> Result<(), String> {
    let inputs = search(".in", path);
    let outputs = search(".out", path);

    for (input, output) in inputs.into_iter().zip(outputs.into_iter()) {
        match run_test(command, input, output) {
            Ok(_) => (),
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}

pub fn run_test(command: &str, input: String, output: String) -> Result<(), String> {
    let mut input = File::open(input).unwrap();
    let mut content = Vec::new();

    match input.read_to_end(&mut content) {
        Ok(_) => (),
        Err(e) => {
            return Err(e.to_string());
        }
    }

    let content: &[u8] = content.as_slice();

    let r = run_command(command, content)?;

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

fn run_command(command: &str, content: &[u8]) -> Result<Result<String, String>, String> {
    let commands: Vec<_> = command.split_whitespace().into_iter().collect();

    let mut child = match Command::new(commands[0])
        .args(&commands[1..commands.len()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    if let Some(stdin) = child.stdin.as_mut() {
        match stdin.write_all(content) {
            Ok(s) => s,
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    if !output.status.success() {
        return match String::from_utf8(output.stderr) {
            Ok(e) => Ok(Err(e)),
            Err(e) => Err(e.to_string()),
        };
    }

    match String::from_utf8(output.stdout) {
        Ok(o) => Ok(Ok(o)),
        Err(e) => Err(e.to_string()),
    }
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
