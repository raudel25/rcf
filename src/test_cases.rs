use std::fs::File;
use std::io::prelude::*;

extern crate reqwest;
extern crate scraper;

use reqwest::Error;
use scraper::{ElementRef, Html, Selector};

pub struct Case {
    name: String,
    input: String,
    output: String,
}

impl Case {
    pub fn new(name: String, input: String, output: String) -> Case {
        Case {
            name,
            input,
            output,
        }
    }

    pub fn create(&self, path: String) -> std::io::Result<()> {
        let mut f_in = File::create(format!("{}/{}.in", path, self.name))?;
        f_in.write_all(self.input.as_bytes())?;

        let mut f_out = File::create(format!("{}/{}.out", path, self.name))?;
        f_out.write_all(self.output.as_bytes())?;

        Ok(())
    }
}

fn get_lines(e: ElementRef) -> Vec<String> {
    let div_selector = Selector::parse("div").unwrap();

    e.select(&div_selector).map(|e| e.inner_html()).collect()
}

fn vec_to_lines(lines: Vec<String>) -> String {
    let mut aux = String::new();

    for s in lines {
        aux.push_str(s.as_str());
        aux.push('\n');
    }

    aux
}

fn extract_example_test_cases(html_content: &str) -> Vec<Case> {
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
            cases.push(Case::new(
                format!("case{}", i),
                vec_to_lines(input),
                vec_to_lines(output),
            ));
        }
    } else {
        println!("No example test cases found.");
    }

    cases
}

pub async fn get_test_cases(contest_id: i32, problem_index: String) -> Result<(), Error> {
    let url = format!(
        "https://codeforces.com/contest/{}/problem/{}",
        contest_id, problem_index
    );
    let html_response = reqwest::get(&url).await?.text().await?;

    extract_example_test_cases(&html_response);

    Ok(())
}
