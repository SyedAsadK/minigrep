use colored::*;
use std::{
    error::{Error},
    fs,
};

pub enum ConfigType {
    File(ConfigFile),
    String(ConfigString),
}

pub struct ConfigFile {
    to_find: String,
    file_path: String,
}
pub struct ConfigString {
    to_find: String,
    in_str: String,
}
impl ConfigType {
    pub fn get_to_find(&self) -> &str {
        match self {
            ConfigType::File(config) => &config.to_find,
            ConfigType::String(config) => &config.to_find,
        }
    }
    pub fn get_content(&self) -> Result<String, Box<dyn Error>> {
        match self {
            ConfigType::File(config) => Ok(fs::read_to_string(&config.file_path)?),
            ConfigType::String(config) => Ok(config.in_str.clone()),
        }
    }
    pub fn get_source_name(&self) -> &str {
        match self {
            ConfigType::File(config) => std::path::Path::new(&config.file_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown"),
            ConfigType::String(_) => "input string",
        }
    }
}

impl ConfigFile {
    pub fn new(args: &[String]) -> Result<ConfigFile, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments!");
        }
        let to_find = args[2].clone();
        let file_path = args[3].clone();
        Ok(ConfigFile { to_find, file_path })
    }
}

impl ConfigString {
    pub fn new(args: &[String]) -> Result<ConfigString, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let to_find = args[1].clone();
        let in_str = args[2].clone();
        Ok(ConfigString { to_find, in_str })
    }
}

pub fn run(config: ConfigType) -> Result<(), Box<dyn Error>> {
    let source_name = config.get_source_name();
    println!(
        "Searching for '{}' which appear in lines of '{}' ...",
        config.get_to_find(),
        source_name
    );

    let content = config.get_content()?;

    match config {
        ConfigType::File(_) => {
            let mut cline = 1;
            for line in search(config.get_to_find(), &content) {
                println!("{line}");
                cline += 1;
            }
            if cline == 1 {
                println!(
                    "Sorry the query '{}' was not found in '{}'",
                    config.get_to_find(),
                    source_name
                );
            }
        }
        ConfigType::String(_) => {
            if content.contains(config.get_to_find()) {
                let highlighted = content.replace(
                    config.get_to_find(),
                    &config.get_to_find().bold().underline().to_string(),
                );
                println!("{highlighted}");
            } else {
                println!(
                    "Sorry the query '{}' was not found in '{}'",
                    config.get_to_find(),
                    source_name
                );
            }
        }
    }

    Ok(())
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
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut res = Vec::new();
    let mut cline = 1;
    for line in contents.lines() {
        if line.contains(query) {
            let mut highlighted = String::from("(");
            highlighted.push_str(&cline.to_string()[..]);
            // highlighted.push(',');
            // highlighted.push_str(&line.find(query.chars().nth(0).unwrap()).unwrap().to_string()[..]);
            highlighted.push(')');
            // highlighted.push(')');
            highlighted.push_str(" : ");
            highlighted.push_str(&line.replace(query, &query.bold().underline().to_string()));
            res.push(highlighted);
        }
        cline += 1;
    }
    res
}
