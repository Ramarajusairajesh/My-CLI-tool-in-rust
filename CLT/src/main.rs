use serde_json::Value;
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;

// Define an enum for Copilot Response
enum CopilotResponse {
    Success(String),
    Error(String),
}

// Implement the Display trait for CopilotResponse
impl Display for CopilotResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CopilotResponse::Success(response) => write!(f, "Success: {}", response),
            CopilotResponse::Error(error) => write!(f, "Error: {}", error),
        }
    }
}

// Function to call Gemini API
async fn gemini_prompt(
    question: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}",
        api_key
    );

    let json = format!(
        r#"{{"contents": [{{"parts": [{{"text": "{}"}}]}}]}}"#,
        question
    );

    let output = Command::new("curl")
        .arg(url)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-X")
        .arg("POST")
        .arg("-d")
        .arg(json)
        .output()?;

    if output.status.success() {
        let response_text = String::from_utf8_lossy(&output.stdout).to_string();
        let json_response: Value = serde_json::from_str(&response_text)?;
        if let Some(text) = json_response
            .pointer("/candidate/0/content/parts/0/text")
            .and_then(|v| v.as_str())
        {
            Ok(text.to_string())
        } else {
            Err("Text not found in response".into())
        }
    } else {
        Err(format!(
            "Error calling Gemini API: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into())
    }
}

// Function to call ChatGPT API
async fn chatgpt_prompt(
    question: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.openai.com/v1/completions"; // Example endpoint
    let json = format!(
        r#"{{"model": "gpt-4", "prompt": "{}", "max_tokens": 150}}"#,
        question
    );

    let output = Command::new("curl")
        .arg(url)
        .arg("-H")
        .arg(format!("Authorization: Bearer {}", api_key))
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-X")
        .arg("POST")
        .arg("-d")
        .arg(json)
        .output()?;

    if output.status.success() {
        let response_text = String::from_utf8_lossy(&output.stdout).to_string();
        let json_response: Value = serde_json::from_str(&response_text)?;
        if json_response.get("error").is_some() {
            println!("Out of requests for the API.");
        }
        if let Some(text) = json_response
            .pointer("/choices/0/text")
            .and_then(|v| v.as_str())
        {
            Ok(text.to_string())
        } else {
            Err("Text not found in response".into())
        }
    } else {
        Err(format!(
            "Error calling ChatGPT API: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into())
    }
}

// Function to call GitHub Copilot
async fn github_copilot(
    question: String,
    options: String,
) -> Result<CopilotResponse, Box<dyn Error>> {
    let output = Command::new("gh")
        .arg("copilot")
        .arg(options)
        .arg(question)
        .output()?;

    if output.status.success() {
        Ok(CopilotResponse::Success(
            String::from_utf8_lossy(&output.stdout).to_string(),
        ))
    } else {
        Ok(CopilotResponse::Error(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

// Main function
fn main() -> Result<(), Box<dyn Error>> {
    println!("1. Gemini 2.0 Experimental");
    println!("2. ChatGPT 4.0/4.0 mini");
    println!("3. GitHub Copilot");
    println!("Press Enter to select all");

    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    println!("Select option: {}", option.trim());
    io::stdout().flush()?;

    let mut question = String::new();
    print!("Question: ");
    io::stdout().flush().expect("Error while cleaning the text");
    io::stdin()
        .read_line(&mut question)
        .expect("Error while reading question");

    let api_file = File::open("/opt/keys/chatbots_api_keys.txt")?;
    let reader = BufReader::new(api_file);
    let mut api_keys = Vec::new();

    for line in reader.lines() {
        api_keys.push(line?.trim().to_string());
    }

    if api_keys.len() < 2 {
        eprintln!("Error: Not enough API keys in the file.");
        return Err("Missing API keys".into());
    }

    let gemini_api_key = &api_keys[0];
    let chatgpt_api_key = &api_keys[1];
    let runtime = tokio::runtime::Runtime::new()?;

    match option.trim() {
        "1" => {
            let gemini_response =
                runtime.block_on(gemini_prompt(question.trim(), gemini_api_key))?;
            println!("Gemini Response: {}", gemini_response);
        }
        "2" => {
            let chatgpt_response =
                runtime.block_on(chatgpt_prompt(question.trim(), chatgpt_api_key))?;
            println!("ChatGPT Response: {}", chatgpt_response);
        }
        "3" => {
            println!("Suggest/Explain");
            let mut copilot = String::new();
            io::stdin()
                .read_line(&mut copilot)
                .expect("Failed to read Copilot options");
            let copilot_response = runtime.block_on(github_copilot(
                question.trim().to_lowercase(),
                copilot.trim().to_lowercase(),
            ))?;
            println!("{}", copilot_response);
        }
        _ => {
            println!("Invalid selection, defaulting to ChatGPT.");
            let chatgpt_response =
                runtime.block_on(chatgpt_prompt(question.trim(), chatgpt_api_key))?;
            println!("ChatGPT Response: {}", chatgpt_response);
        }
    }

    Ok(())
}
