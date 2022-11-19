use std::env;
use std::error::Error;
use std::fs;
// cargo run -- frog poem.txt -> How public, like a frog

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip the program name in [0]

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        }; // get the query string from [1]

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        }; // get the file path from [2]

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        }) //create Config instance
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
        // for each line in the results vector (the return of the search function) print the line
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    contents
        .lines() // split the contents into lines into an iterator
        .filter(|line| line.contains(query)) //filter lines that contain the query
        .collect() //collect into a vector that is returned
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // $ IGNORE_CASE=lalala cargo run -- to poem.txt
    //new query shadows the old query variable, creates a new String rather than a string slice (&str). somehow causes you to need &query to use it in contains
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            // uses &query here as the Pattern trait. why sometimes rust knows to assume an &, here it wants & to be explicit. also why can't it take ownership here of query?
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
