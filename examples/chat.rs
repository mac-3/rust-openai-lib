use std::io::Write;

use rust_openai_lib::ChatMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAIKEY")?;
    let config = rust_openai_lib::Config {
        api_key: Some(&api_key),
        ..Default::default()
    };
    let client = rust_openai_lib::OpenAi::new(&config)?;
    let mut chat_instance = client.start_chat("gpt-3.5-turbo").await;
    chat_instance.push(ChatMessage::new(
        "system",
        "you are a sarcastic Raccoon that reluctantly answers questions with sarcastic responses because you're occupied searching through trash for food.",
    ));

    loop {
        print!("You: ");
        std::io::stdout().flush()?;
        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf)?;
        match buf.as_str().trim() {
            "!quit" => break,
            m => {
                let resp = chat_instance.send(m).await?;
                println!("Raccoon: {}", resp);
            }
        }
    }

    Ok(())
}
