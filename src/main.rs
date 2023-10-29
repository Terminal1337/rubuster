use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use reqwest::blocking;
use reqwest::StatusCode;

fn read_file(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn brute(url: &str, word: &str) -> Result<(), Box<dyn std::error::Error>> {
    let uri = format!("{}/{}", url, word);
    let response = blocking::get(&uri)?;
    let status = response.status();

    if status < StatusCode::BAD_REQUEST {
        println!(" | INFO: {}: [{}]", word, status);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: {} --url --wordlist --threads", "./rubuster");
    } else {
        let lines = read_file(&args[2]).unwrap();
        println!("=================================");
        println!("      RuBuster V1");
        println!("=================================");
        println!("URL: {}", &args[1]);
        println!("Wordlist: {}", &args[2]);
        println!("Threads: {}\n", args[3]);

        let total_requests = lines.len();
        let mut requests_made = 0;

        for (index, line) in lines.iter().enumerate() {
            if let Err(err) = brute(&args[1], line) {
                eprintln!("ERROR: {}: [{}]", line, err);
            }
            requests_made += 1;
            let progress = (requests_made as f32 / total_requests as f32) * 100.0;
            print!("\rProgress: {:.2}% | Lines Bruted: {} | Lines Left: {}   ", progress, requests_made, total_requests - requests_made);

            io::stdout().flush().expect("Failed to flush");
        }

        println!("\n=================================");
        println!("Brute force completed.");
    }
}
