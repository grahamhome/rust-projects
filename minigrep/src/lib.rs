use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, ()> {
        if args.len() < 3 {
            return Err(());
        }
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| { line.contains(query) }).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "heal";
        let contents = "\
Early to bed and early to rise,
Makes a man healthy, wealthy and wise";
        assert_eq!(vec!["Makes a man healthy, wealthy and wise"], search(query, contents))
    }

    #[test]
    fn case_insensitive_result() {
        let query = "BanAnA";
        let contents = "\
My favorite fruits are:
Apples, bananas and pears.
I don't like tomatoes";
        assert_eq!(vec!["Apples, bananas and pears."], search_case_insensitive(query, contents))
    }
}
