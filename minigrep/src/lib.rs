#[cfg(test)]
mod conditional_imports {
    pub mod fs;
}

#[cfg(not(test))]
mod conditional_imports {
    pub use std::fs;
}

use conditional_imports::*;
use std::error::Error;

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let query = config.query;
    let content = fs::read_to_string(config.filename)?;
    let mut result: Vec<String> = Vec::new();
    for line in search(&query, &content) {
        result.push(String::from(line));
    }
    Ok(result)
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let filename = args.pop().unwrap();
        let query = args.pop().unwrap();
        Ok(Config { query, filename })
    }
}

/**************************************
 **************************************
 ************* UNIT TESTS *************
 **************************************
 **************************************/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        // First param when running a console application is ignored
        let waste = String::from("");
        let query = String::from("forest");
        let fake_file = String::from("fake file");
        let parameters: Vec<String> = vec![waste, query, fake_file];
        let config = Config::new(parameters).unwrap();
        let result = run(config).unwrap();
        assert_eq!(vec!["In the forest"], result);
    }

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let content = "\
            Rust:\n\
            safe, fast, productive.\n\
            Pick three.\n\
            Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn creating_a_new_config_object_requires_3_parameters() {
        let parameters: Vec<String> = vec![String::from(""), String::from("foo"), String::from("bar")];
        let config = Config::new(parameters).unwrap();
        assert_eq!("foo", config.query);
        assert_eq!("bar", config.filename);
    }
}
