use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    // I'm not sure why we put a static liftetime here tbh, maybe for nested errors?
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // Clone here for very minor performance cost but simpler code. What is this, python?!
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query){
            matches.push(line);
        }
    }
    matches
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matches = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
            matches.push(line);
        }
    }
    matches
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
RUST:
safe, fast, productive.
Pick three.
Duct tape lol.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
RUST:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["RUST:", "Trust me."], search_case_insensitive(query, contents));
    }
}
