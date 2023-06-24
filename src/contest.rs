use reqwest::{get, Error};
use serde_json::Value;

pub async fn get_problems<'a>(contest_id: i32) -> Result<Vec<String>, Error> {
    let url = format!(
        "https://codeforces.com/api/contest.standings?contestId={}&from=1&count=1",
        contest_id
    );

    let response = get(&url).await?.text().await?;
    let response: Value = serde_json::from_str(&response).unwrap();

    let problems = response["result"]["problems"].as_array().unwrap();
    let mut index = Vec::new();

    for problem in problems {
        index.push(String::from(problem["index"].as_str().unwrap()));
    }

    Ok(index)
}
