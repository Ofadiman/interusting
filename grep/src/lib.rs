use std::{env, error::Error, fs};

pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        return Ok(Config {
            query: &args[1],
            file_path: &args[2],
            ignore_case,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    results.iter().for_each(|line| println!("{line}"));

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod search {
        use super::*;

        #[test]
        fn no_results() {
            let query = "Hello";
            let contents = "Rust:
safe, fast, productive.
Pick three.";

            assert_eq!(vec![] as Vec<&str>, search(query, contents));
        }

        #[test]
        fn one_result() {
            let query = "duct";
            let contents = "Rust:
safe, fast, productive.
Pick three.";

            assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        }

        #[test]
        fn many_results() {
            let query = "u";
            let contents = "Rust:
safe, fast, productive.
Pick three.";

            assert_eq!(
                vec!["Rust:", "safe, fast, productive."],
                search(query, contents)
            );
        }
    }

    mod search_case_insensitive {
        use super::*;

        #[test]
        fn no_results() {
            let query = "hello";
            let contents = "Rust:
safe, fast, productive.
Pick three.";

            assert_eq!(
                vec![] as Vec<&str>,
                search_case_insensitive(query, contents)
            );
        }

        #[test]
        fn one_result() {
            let query = "SAFE";
            let contents = "Rust:
safe, fast, productive.
Pick three.";

            assert_eq!(
                vec!["safe, fast, productive."],
                search_case_insensitive(query, contents)
            );
        }

        #[test]
        fn many_results() {
            let query = "U";
            let contents = "Rust:
safe, fast, productive.
Pick three.";

            assert_eq!(
                vec!["Rust:", "safe, fast, productive."],
                search_case_insensitive(query, contents)
            );
        }
    }
}
