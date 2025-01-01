
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(words: &[String]) -> Result<Config, &'static str> {

        if words.len() < 3 {
            return Err("not enough arguments");
        }

        let query = words[1].clone();
        let filepath = words[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config {query, filepath})
    }
}

pub fn run(con: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(con.filepath)?;

    let results = if con.ignore_case {
        search_case_insensitive(&con.query, &content)
    } else {
        search(&con.query, &content)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
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

