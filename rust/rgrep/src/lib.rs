#[cfg(test)]
mod tests;

use std::{error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(query: &str, file_path: &str, ignore_case: bool) -> Config {
        return Config {
            query: query.to_owned(),
            file_path: file_path.to_owned(),
            ignore_case: ignore_case,
        };
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(&cfg.file_path)?;
    for result in search(&cfg.query, &content, cfg.ignore_case) {
        println!("{}", result);
    }
    Ok(())
}

fn search<'a>(query: &str, text: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let query = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_owned()
    };
    let mut results = vec![];
    for line in text.lines() {
        let l = if ignore_case {
            line.to_lowercase()
        } else {
            line.to_owned()
        };
        if l.contains(&query) {
            results.push(line);
        }
    }
    return results;
}
