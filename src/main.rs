use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use reqwest::blocking::{Client, Request};
use reqwest::StatusCode;

fn read_file(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn create_client(proxy: Option<&String>) -> Client {
    match proxy {
        Some(p) => {
            let client = reqwest::blocking::Client::builder()
                .proxy(reqwest::Proxy::http(p).unwrap())
                .build()
                .unwrap();
            client
        },
        None => reqwest::blocking::Client::new(),
    }
}

fn brute(url: &str, word: &str, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let uri = format!("{}/{}", url, word);
    let response = client.get(&uri).send()?;
    let status = response.status();

    if status < StatusCode::BAD_REQUEST {
        println!(" | INFO: {}: [{}]", word, status);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: {} --url --wordlist --threads [--proxy]", "./rubuster");
    } else {
        let lines = read_file(&args[2]).unwrap();
        println!("=================================");
        println!("      RuBuster V1");
        println!("=================================");
        println!("URL: {}", &args[1]);
        println!("Wordlist: {}", &args[2]);
        println!("Threads: {}", args[3]);
        
        let proxy = args.get(5);
        let client = create_client(proxy);

        let total_requests = lines.len();
        let requests_made = Arc::new(Mutex::new(0));

        let mut handles = vec![];

        for chunk in lines.chunks(total_requests / args[3].parse::<usize>().unwrap()) {
            let url = args[1].clone();
            let chunk = chunk.to_vec();
            let requests_made = Arc::clone(&requests_made);
            let client = client.clone();

            let handle = thread::spawn(move || {
                for line in chunk {
                    if let Err(err) = brute(&url, &line, &client) {
                        eprintln!("ERROR: {}: [{}]", line, err);
                    }
                    let mut requests_made = requests_made.lock().unwrap();
                    *requests_made += 1;
                    let progress = (*requests_made as f32 / total_requests as f32) * 100.0;
                    print!("\rProgress: {:.2}% | Lines Bruted: {} | Lines Left: {}   ", progress, *requests_made, total_requests - *requests_made);
                    io::stdout().flush().expect("Failed to flush");
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("\n=================================");
        println!("Brute force completed.");
    }
}
