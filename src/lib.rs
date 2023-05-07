use std::error::Error;
use std::fs;
use std::env;


pub fn run(params: Params) -> Result<(), Box<dyn Error>>{
    let contents  = fs::read_to_string(params.filename)?;
    let results = if config.ignore_case {
        search_case_insensitive(&params.regex, &contents)
    } else {
        search(&params.regex, &contents)
    }
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub struct Params{
    pub regex: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Params {
    pub fn new(args: &[String]) -> Result<Params, &'static str>{
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let regex = args[0].clone();
        let filename = args[1].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Params{regex, filename, ignore_case}) 
    }

    
}

pub fn search<'a> (regex: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(regex) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a> (regex: &str, contents: &'a str) -> Vec<&'a str> {
    let regex = regex.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&regex) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}