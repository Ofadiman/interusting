use std::error::Error;
use std::fs;

pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        return Ok(Config {
            query: &args[1],
            file_path: &args[2],
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
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
