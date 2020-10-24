use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let query = config.query;
    
    for line in search(&query, &contents) {
        println!("{}", line);
    }
    
    Ok(())
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

pub struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query: &'a str = &args[1];
        let filename: &'a str = &args[2];
        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}