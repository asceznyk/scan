use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool, 
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        let query = String::from(&args[1]);
        let file_path = String::from(&args[2]);
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            query, 
            file_path, 
            ignore_case, 
        })
    }
}

fn match_case(s: &String, case: &bool) -> String {
    if *case { return s.to_lowercase() }
    else { return s.to_string() };
}

pub fn scan<'a>(
    query: &String, 
    content: &'a String,
    ignore_case: bool,
) -> Vec<&'a str> {
    let mut matches = Vec::new();

    let query = match_case(query, &ignore_case); 
    for line in content.lines() {
        let mod_line = match_case(&line.clone().to_string(), &ignore_case);
        if mod_line.contains(&query) {
            matches.push(line);
        }
    }

    matches
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    for line in scan(&config.query, &content, config.ignore_case) {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = String::from("duct");
        let contents = String::from("\
Rust:
safe, fast, productive.
Pick three.");

        assert_eq!(vec!["safe, fast, productive."], scan(&query, &contents, false));
    }

    #[test]
    fn case_insensitive() {
        let query = String::from("rUsT");
        let contents = String::from("\
Rust:
safe, fast, productive.
Pick three.
Trust me.");

        assert_eq!(
            vec!["Rust:", "Trust me."],
            scan(&query, &contents, true)
        );
    }
}

