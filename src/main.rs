/// chatwise a cli command to hold a conversation with OpenAI's ChatGPT.
/// Author: Bob Peters <contact@rust-trends.com>
///
/// This file is part of ChatWise which is released under GNU GPLv3.
/// See file LICENSE or go to https://www.gnu.org/licenses/gpl-3.0.md for full license details.
mod api_structs;

use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use std::{
    env,
    error::Error,
    fs::File,
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
    time::Duration,
};

use api_structs::{Chatwise, Message, ResponseCompletion, Role};

#[derive(Parser)]
#[clap(
    author = "Bob Peters <contact@rust-trends.com>",
    about = "chatwise a cli command to have a conversation with OpenAI's ChatGPT.",
    version
)]
struct Args {
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    /// Name of the output file, will be written in Markdown format
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut _output_file: Option<File> = None;

    // Get the API key from the environment
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(val) => val,
        Err(e) => {
            println!("Environment variable `OPENAI_API_KEY` is not set, {e}");
            return Err(e.into());
        }
    };

    if args.verbose {
        println!("Running in verbose mode");
        println!("API key is {api_key}");
    }

    if args.output.is_some() {
        let path = args.output.unwrap();
        _output_file = Some(
            OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(path)
                .unwrap(),
        );
    }

    let headers = create_headers(&api_key);
    let url = "https://api.openai.com/v1/chat/completions";

    let mut chatwise = Chatwise {
        model: "gpt-3.5-turbo".to_string(),
        stream: false,
        messages: vec![],
    };

    chatwise.messages.push(Message {
        role: Role::System,
        content: "You are a helpful assistant.".to_string(),
    });

    let stdin = io::stdin(); // We get `Stdin` here.
    let pb = ProgressBar::new_spinner();

    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["-", "\\", "|", "/"])
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );

    loop {
        print!("{} ", ">".green());
        std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;

        chatwise.messages.push(Message {
            role: Role::User,
            content: buffer.clone(),
        });

        // Enbable the progress bar and send the request
        pb.enable_steady_tick(Duration::from_millis(100));
        let client = Client::new();

        let mut request = client
            .post(url)
            .json(&chatwise)
            .headers(headers.clone())
            .send()
            .await?;

        // Read the response body
        let mut body = vec![];
        while let Some(chunk) = request.chunk().await? {
            body.extend(chunk);
        }

        // Disable the progress bar
        pb.disable_steady_tick();

        let response_string = String::from_utf8_lossy(&body).to_string();
        let response_json: ResponseCompletion = serde_json::from_str(&response_string)?;
        let text = response_json.choices[0].message.content.clone();

        println!("chatwise:\n{}\n", text.blue());

        if let Some(file) = _output_file.as_mut() {
            // Write the user input and chatwise response to the file
            if let Err(e) = write!(file, "***user>*** {}\n\n***chatwise>*** ", buffer.trim()) {
                eprintln!("Couldn't write to file: {}", e);
            }

            // Write the chatwise response to the file
            if let Err(e) = write!(file, "{}\n\n", text.trim()) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
}

/// Create the headers for the request with the API key
/// returns a HeaderMap
fn create_headers(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers
}
