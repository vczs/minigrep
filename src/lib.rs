use std::{env, error::Error, fs};

pub struct Config{
    query: String,
    file_path: String,
    ignore_case: bool
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("args is wrong!")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        let config = Config { query: query, file_path: file_path, ignore_case: ignore_case };

        Ok(config)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case{
        search_case_insensitive(&config.query, &content)
    }else{
        search_case_sensitive(&config.query, &content)
    };

    for line in result {
        println!("{line}")
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines(){
        if line.contains(query){
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    let query = query.to_lowercase();
    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_search_case_sensitive(){
        let query = "name";
        let content = "
How dreary to be somebody!
How public, like a frog
To tell your name the livelong day";
        assert_eq!(vec!["To tell your name the livelong day"], super::search_case_sensitive(query, content))
    }

    #[test]
    fn test_search_case_insensitive(){
        let query = "nAmE";
        let content = "
How dreary to be somebody!
How public, like a frog
To tell your name the livelong day";
        assert_eq!(vec!["To tell your name the livelong day"], super::search_case_insensitive(query, content))
    }
}