
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn build(words: &[String]) -> Result<Config, &'static str> {

        if words.len() < 3 {
            return Err("not enough arguments");
        }

        let query = words[1].clone();
        let filepath = words[2].clone();
    
        Ok(Config {query, filepath})
    }
}

pub fn run(con: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(con.filepath)?;


    println!("With text:\n{content}");

    Ok(())
}
