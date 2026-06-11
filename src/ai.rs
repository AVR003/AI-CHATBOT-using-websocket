use reqwest::Client;
use serde_json::{json, Value};

pub async fn ask_gemini(prompt: &str) -> String {

    let api_key =
        std::env::var("GROQ_API_KEY")
            .expect("GROQ_API_KEY not found");

    let client = Client::new();

    let body = json!({
        "model": "llama-3.3-70b-versatile",
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await;

    match response {

        Ok(res) => {

            let value: Value =
                res.json().await.unwrap();

            println!("Groq Response:");
            println!("{}", value);

            if let Some(error) = value.get("error") {

                return format!(
                    "Groq Error: {}",
                    error["message"]
                        .as_str()
                        .unwrap_or("Unknown error")
                );
            }

            value["choices"][0]
                ["message"]["content"]
                .as_str()
                .unwrap_or("No response")
                .to_string()
        }

        Err(err) => {

            format!(
                "Failed to contact Groq: {}",
                err
            )
        }
    }
}