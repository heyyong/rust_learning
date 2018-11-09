use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let argvs: Vec<String> = env::args().collect();
    let config = Config::new(&argvs).unwrap();
    run (config);
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
