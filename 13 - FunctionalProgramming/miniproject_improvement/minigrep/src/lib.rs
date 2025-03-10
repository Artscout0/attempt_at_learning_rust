use std::fs;
use std::error::Error;
use std::env;


/// Posible configuration.
pub struct Config {
    query: String,
    file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Creates a config from the command line arguments. 
    /// 
    /// In other terms, it gets the what you're looking for, where you're looking for it, and wheter it needs to be case sensitive.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let config = Config::build(&args).unwrap_or_else(|err| 

        // Must clone, as String is immutable
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case})
    }
}

/// Reads data from the file_path specified in config, and searches for the query.
pub fn run(config: Config)  -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;

    // this is beautiful. I've looked at this for 5 hours now.
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

// For each line in the contents, check if it contains the exact query
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // The change is here:  
    
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// For each line in the contents, check if it contains the query, ignoring the case
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // The change is here:

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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