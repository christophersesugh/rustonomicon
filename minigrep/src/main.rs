use minigrep::search;
use std::{env, error, fs, process};
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = search(&config.query, &contents, config.ignore_case);

    for line in result {
        println!("{line}")
    }
    Ok(())
}

struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let ignore_case_arg = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get project name"),
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get file path"),
        };

        let ignore_case = args.next().unwrap_or_default();

        let paths_len = ignore_case_arg.len();
        let _project_path = &ignore_case_arg[13..paths_len];

        let ignore_case_args = vec![String::from("--ignore-case"), String::from("-ic")];

        let ignore_case =
            ignore_case_args.contains(&ignore_case) || env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
