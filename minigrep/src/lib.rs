use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Ok(Config { query, filename, case_sensitive })
  }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
  let mut f = File::open(config.filename)?;
  let mut content = String::new();
  f.read_to_string(&mut content)?;
  let results = if config.case_sensitive {
    search(&config.query, &content)
  } else {
    search_insensitive(&config.query, &content)
  };
  for line in results {
    println!("{}", line);
  } 
  Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>{
  let mut ret: Vec<&'a str> = vec![];

  for line in contents.lines() {
    if line.contains(query) {
      ret.push(line);
    }
  }
  ret
}

pub fn search_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>{
  let mut ret: Vec<&'a str> = vec![];
  let lower_query = query.to_lowercase();
  for line in contents.lines() {
    if line.to_lowercase().contains(&lower_query) {
      ret.push(line);
    }
  }
  ret
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
Pick three.";
    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    );
    
  }

  #[test]
  fn search_sensitive_test() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    );
  }

  #[test]
  fn search_insensitive_test() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_insensitive(query, contents)
    );
  }
}