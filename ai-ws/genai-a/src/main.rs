use genai::chat::{ChatMessage, ChatRequest};
use genai::client::Client;
use genai::utils::{print_chat_stream, PrintChatStreamOptions};

const MODEL_OPENAI: &str = "gpt-3.5-turbo";
const MODEL_ANTHROPIC: &str = "claude-3-haiku-20240307";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let question = "生命的意义是什么？";

    let chat_req = ChatRequest::new(vec![
        ChatMessage::user(question),
    ]);

    let client = Client::default();

    let print_options = PrintChatStreamOptions::from_stream_events(true);

    for model in [MODEL_OPENAI, MODEL_ANTHROPIC] {
        println!("\n===== 模型: {} =====", model);

        println!("\n--- 问题:\n{}", question);

        println!("\n--- 回答: (一次性响应)");
        let chat_res = client.exec_chat(model, chat_req.clone(), None).await?;
        println!("{}", chat_res.content.as_deref().unwrap_or("无答案"));

        println!("\n--- 回答: (流式响应)");
        let chat_res = client.exec_chat_stream(model, chat_req.clone(), None).await?;
        print_chat_stream(chat_res, Some(&print_options)).await?;

        println!();
    }

    Ok(())
}