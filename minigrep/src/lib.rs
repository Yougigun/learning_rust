use std::{env, error::Error, fs};

//adding documentation comments to the library's public API
/// Config struct
///  
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip the first argument which is the program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    if !config.ignore_case {
        for lines in search(&config.query, &contents) {
            println!("{lines}")
        }
    } else {
        for lines in search_case_insensitive(&config.query, &contents) {
            println!("{lines}")
        }
    }
    Ok(())
}

//adding documentation comments to the library's public API
/// search function
/// 
/// # Examples
/// 
/// ```
/// use minigrep::search;
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
/// assert_eq!(vec!["safe, fast, productive."], search(query, contents))
/// ```
/// 
/// # Panics
/// 
/// ```
/// use minigrep::search;
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
/// assert_eq!(vec!["safe, fast, productive."], search(query, contents))
/// ```
/// 
/// # Errors
/// 
/// ```
/// use minigrep::search;
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
/// assert_eq!(vec!["safe, fast, productive."], search(query, contents))
/// ```
/// 
/// # Safety
/// 
/// ```
/// use minigrep::search;
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
/// assert_eq!(vec!["safe, fast, productive."], search(query, contents))
/// ```
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect::<Vec<_>>()
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
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
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
