use std::{error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config{query, file_path})
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    println!("searching for {}", cfg.query);
    println!("from file {}", cfg.file_path);

    let contents = fs::read_to_string(cfg.file_path)?;
    println!("With text:\n{contents}");

    Ok(())
}