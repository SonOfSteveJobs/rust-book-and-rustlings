use std::{error::Error, fs, path::Path, thread};

pub struct Config {
    pub query: String,
    pub filenames: Vec<String>,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String], case_sensitive: bool) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        if query.is_empty() {
            return Err("Query is empty");
        }

        let filenames: Vec<String> = args[2..].to_vec();
        if filenames.is_empty() {
            return Err("Please provide at least one filename");
        }

        for filename in &filenames {
            let extension = Path::new(filename)
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            match extension {
                "txt" => (),
                _ => return Err("Only .txt files are supported"),
            }
        }

        Ok(Config {
            query,
            filenames,
            case_sensitive,
        })
    }
}

fn parse_file(filename: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filename)
}

fn process_file(filename: &str, query: &str, case_sensitive: bool) -> Result<usize, String> {
    let contents =
        parse_file(filename).map_err(|e| format!("Error reading file {}: {}", filename, e))?;

    let results = if case_sensitive {
        search(query, &contents)
    } else {
        search_case_insensitive(query, &contents)
    };

    println!(
        "File: {} - Found {} lines for query \"{}\":",
        filename,
        results.len(),
        query
    );

    for line in &results {
        println!("{}", line);
    }
    println!();

    Ok(results.len())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut total_matches = 0;

    thread::scope(|s| -> Result<(), String> {
        let mut handles = Vec::new();

        // Запускаем поток для каждого файла
        for filename in &config.filenames {
            let query = config.query.clone();
            let case_sensitive = config.case_sensitive;

            let handle = s.spawn(move || process_file(filename, &query, case_sensitive));
            handles.push(handle);
        }

        // Ждём завершения всех потоков и собираем результаты
        for handle in handles {
            match handle.join() {
                Ok(Ok(count)) => total_matches += count,
                Ok(Err(e)) => return Err(e),
                Err(_) => return Err("Thread panicked".to_string()),
            }
        }

        Ok(())
    })
    .map_err(|e| -> Box<dyn Error> { e.into() })?;

    println!("=== Summary ===");
    println!("Total files processed: {}", config.filenames.len());
    println!("Total matches found: {}", total_matches);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
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
safe, fast, productive.";
        assert_eq!(vec!["safe, fast, productive."], search(query, &contents));
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
