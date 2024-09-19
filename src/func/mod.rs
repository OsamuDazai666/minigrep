use crate::args::Args;
use std::error::Error;
use std::fs;
use std::process::exit;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&args.file_path);

    let file_content = match file_content {
        Err(_) => {
            println!("File: {} Not Found...", args.file_path);
            exit(1);
        }
        Ok(content) => content,
    };

    let results = search(&args.query, &file_content);

    for result in results {
        println!("{}", result);
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut to_return = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            to_return.push(line);
        }
    }

    to_return
}
