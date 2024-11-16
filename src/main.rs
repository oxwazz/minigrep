use std::error::Error;
use std::fs;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
    println!("Searching for {}", &config.query);
    println!("In file {}", &config.file_path);

    if let Err(e) = Config::run(config) {
        eprintln!("Ada error cuy: {e}");
        process::exit(1)
    }
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Argumen kurang cuy");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
    fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;

        let contents = match config.ignore_case {
            true => search_incase_sensitive(&config.query, &contents),
            false => search(&config.query, &contents),
            _ => vec![]
        };
        println!("hasil pencarian:\n{contents:?}");
        Ok(())
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for content in contents.lines() {
        if content.contains(query) {
            result.push(content)
        }
    }

    result
}

fn search_incase_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for content in contents.lines() {
        if content.to_lowercase().contains(&query.to_lowercase()) {
            result.push(content)
        }
    }

    result
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn positive_case() {
        let query = "tab";
        let contents = "\
Oxwazz:
mantabbanget, gg, wp.
Terima kasih.";
        assert_eq!(search(&query, &contents), ["mantabbanget, gg, wp."])
    }

    #[test]
    fn incase_sensitive() {
        let query = "tABbAnG";
        let contents = "\
Oxwazz:
mantabbanget, gg, wp.
Terima kasih.";
        assert_eq!(search_incase_sensitive(&query, &contents), ["mantabbanget, gg, wp."])
    }
}


