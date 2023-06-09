use std::env;
use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

// arg contents' lifetime is connected to that of the return val
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // the lines method returns an iterator
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results

    // using an iter, you can omit the intermediate vec and the above code looks like this:
    // contents
    //     .lines()
    //     .filter(|line| line.contains(query))
    //     .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //dyn is short for dynamic
    let contents = fs::read_to_string(config.file_path)?;

    // println!("With text:\n{contents}");

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    // this is an idiomatic way to indicate that this func is called for its side effects only; it doesn't return a value you need
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

/* The first step: extract the parsing logic from the main to a func */
// fn parse_config(args: &[String]) -> Config {
//     // clone makes it straightforward(managing the lifetimes of the refs isn't necessary) but takes a bit more time to process at runtime so performance gets worse
//     // it's a trade-off and which one to choose depends on things like the scale of your project
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

impl Config {
    pub fn build(
        /* args: &[String] */ mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // the fist arg returned from env::args is the name of the program, so go one step further in advance
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // if args.len() < 3 {
        //     // panic!("not enough arguments");
        //     return Err("not enough arguments");
        // }
        // let query = args[1].clone();
        // let file_path = args[2].clone();
        // let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
