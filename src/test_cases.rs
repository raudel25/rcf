use std::fs;
use std::fs::File;
use std::io::prelude::*;

extern crate reqwest;
extern crate scraper;

use reqwest::Error;
use scraper::{ElementRef, Html, Selector};

pub struct TestCase {
    name: String,
    input: String,
    output: String,
}

impl TestCase {
    pub fn new(name: String, input: String, output: String) -> TestCase {
        TestCase {
            name,
            input,
            output,
        }
    }

    pub fn create(&self, path: &str) -> std::io::Result<()> {
        if !fs::metadata(&path).is_ok() {
            fs::create_dir_all(&path)?;
        }

        let mut f_in = File::create(format!("{}/{}.in", path, self.name))?;
        f_in.write_all(self.input.as_bytes())?;

        let mut f_out = File::create(format!("{}/{}.out", path, self.name))?;
        f_out.write_all(self.output.as_bytes())?;

        Ok(())
    }
}

fn get_lines(e: ElementRef) -> String {
    let div_selector = Selector::parse("div").unwrap();

    let lines = e
        .select(&div_selector)
        .map(|e| e.inner_html())
        .collect::<Vec<_>>();

    if lines.len() != 0 {
        vec_to_lines(lines)
    } else {
        e.inner_html()
    }
}

fn vec_to_lines(lines: Vec<String>) -> String {
    let mut aux = String::new();

    for s in lines {
        aux.push_str(s.as_str());
        aux.push('\n');
    }

    aux
}

fn extract_example_test_cases(html_content: &str) -> Result<Vec<TestCase>, String> {
    let document = Html::parse_document(html_content);
    let example_selector = Selector::parse(".sample-test").unwrap();
    let input_selector = Selector::parse(".input pre").unwrap();
    let output_selector = Selector::parse(".output pre").unwrap();

    let mut cases = Vec::new();

    if let Some(sample_test) = document.select(&example_selector).next() {
        let inputs = sample_test
            .select(&input_selector)
            .map(|e| get_lines(e))
            .collect::<Vec<_>>();
        let outputs = sample_test
            .select(&output_selector)
            .map(|e| get_lines(e))
            .collect::<Vec<_>>();

        for (i, (input, output)) in inputs.into_iter().zip(outputs.into_iter()).enumerate() {
            cases.push(TestCase::new(format!("case{}", i + 1), input, output));
        }

        Ok(cases)
    } else {
        Err(String::from("No example test cases found"))
    }
}

async fn response(contest_id: i32, problem_index: &str) -> Result<String, Error> {
    let url = format!(
        "https://codeforces.com/contest/{}/problem/{}",
        contest_id, problem_index
    );
    let html_response = reqwest::get(&url).await?.text().await?;

    Ok(html_response)
}

pub fn get_test_cases(contest_id: i32, problem_index: &str) -> Result<Vec<TestCase>, String> {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    match runtime.block_on(response(contest_id, problem_index)) {
        Ok(html_response) => extract_example_test_cases(&html_response),
        Err(e) => Err(e.to_string()),
    }
}
