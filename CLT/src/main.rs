use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;

async fn gemini_prompt(
    question: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let n = api_key.trim();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}",
        api_key,
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
        // Parse the JSON and extract the text part
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

async fn chatgpt_prompt(
    question: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.openai.com/v1/completions"; // Example endpoint, you may need to update
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
        // Parse the JSON and extract the text part
        let json_response: Value = serde_json::from_str(&response_text)?;
        println!("{:?}", json_response.pointer("/error"));
        if (json_response.get("error").is_some()) {
            println!("Out of request for the api");
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
async fn github_copilot(question:&str){

}
fn main() -> Result<(), Box<dyn Error>> {
    // Displaying options
    println!("1. Gemini 2.0 Experimental");
    println!("2. ChatGPT 4.0/4.0 mini");
    println!("3.Github Copliot");
    println!("Press Enter to select all");

    // Reading user option
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    println!("Select option: {}", option.trim());
    io::stdout().flush();
    // Reading the question from user
    let mut question = String::new();
    print!("Question: ");
    io::stdout().flush().expect("Error while cleaning the text");
    io::stdin()
        .read_line(&mut question)
        .expect("Error while reading question");

    // Reading API keys from file
    let api_file = File::open("/opt/keys/chatbots_api_keys.txt")?;
    let reader = BufReader::new(api_file);
    let mut api_keys = Vec::new();

    // Assuming each line contains a single API key for each chatbot
    for line in reader.lines() {
        api_keys.push(line?.trim().to_string());
    }

    if api_keys.len() < 2 {
        eprintln!("Error: Not enough API keys in the file.");
        return Err("Missing API keys".into());
    }
    // Select the appropriate API key for each option
    let gemini_api_key = &api_keys[0];
    let chatgpt_api_key = &api_keys[1];

    // Initialize a runtime to execute async functions within the main sync context
    let runtime = tokio::runtime::Runtime::new()?;

    // Handling option selection and calling the respective chatbot prompt function
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
        "3" =>{
            println!("Suggest/Explain");
            let mut copilot=String::new();
            io::stdin().read_line(&mut copilot).expect("Failed to read copilot options");
            let copilot_response=runtime.block_on(github_copilot(question))
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
