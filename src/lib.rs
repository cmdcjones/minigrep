use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, (&str, usize)> {
        if args.len() < 3 {
            return Err(("not enough arguments - arguments passed:", args.len() - 1));
        }
        let query = args[1].to_owned();
        let file_path = args[2].to_owned();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
