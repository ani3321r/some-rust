use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
  //reading a file
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive{
    search(&config.query, &contents)
  } else{
    search_case_insensitive(&config.query, &contents)
  };

  for line in results{
    println!("{}", line);
  }

  Ok(())
}

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
} // to give other files access, we have to make it public

impl Config{
  pub fn new(args: &[String]) -> Result<Config, &str>{// convention for constructor func(calling new)
      if args.len() < 3{
          return Err("not enough args");
      }
      let query =  args[1].clone();
      let filename = args[2].clone();

      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
  
      Ok(Config {query, filename, case_sensitive})
  }
}

// to active the text insensitive option the following command is used
// export CASE_INSENSITIVE=true

// to revert back
// unset CASE_INSENSITIVE

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
  let mut results = Vec::new();

  for line in contents.lines(){
    if line.contains(query){
      results.push(line);
    }
  }

  results
}

pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str,
) -> Vec<&'a str>{
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines(){
    if line.to_lowercase().contains(&query){
      results.push(line);
    }
  }

  results
}

//test driven development
#[cfg(test)]
mod tests{
  use super::*;

  #[test]
  fn case_sensitive(){
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive(){
    let query = "rUSt";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."],
    search_case_insensitive(query, contents));
  }
}