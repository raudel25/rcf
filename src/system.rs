use std::io::Write;
use std::process::{Command, Stdio};

pub fn compile(command: &str) -> Result<Result<(), String>, String> {
    let commands: Vec<_> = command.split_whitespace().into_iter().collect();

    let child = match Command::new(commands[0])
        .args(&commands[1..commands.len()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            return Err(e.to_string());
        }
    };

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

    Ok(Ok(()))
}

pub fn run(command: &str, content: &[u8]) -> Result<Result<String, String>, String> {
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
