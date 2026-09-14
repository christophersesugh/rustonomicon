pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            if ignore_case {
                return line.to_lowercase().contains(&query.to_lowercase());
            }
            line.contains(query)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
      Rust:
      safe, fast, productive
      Pick three.
      Duct tape.";

        assert_eq!(
            vec!["safe", "fast", "productive"],
            search(query, contents, false)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true));
    }
}
