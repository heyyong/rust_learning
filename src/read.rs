use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let argvs: Vec<String> = env::args().collect();
    let config = Config::new(&argvs).unwrap();

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {

    let mut file = File::open(config.filename)?;
    let mut file_content = String::new();

    file.read_to_string(&mut file_content)?;

    println!("{}", file_content);

    Ok(())
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let query = args[0].clone();
        let filename = args[1].clone();

        Ok(Config { query, filename})
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three";

        assert_eq!(
            vec!["save, fast, productive."],
            search(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
