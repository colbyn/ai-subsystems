use ai_subsystems::text_api;
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::fs::read_to_string("secrets/open-ai.key").unwrap();
    let api_url = text_api::client::URL::OPEN_AI_CHAT_COMPLETIONS;
    let globals = text_api::xml_dsl::object!({});
    let prompt = text_api::xml_dsl::Prompt::open_with("assets/basic.prompt.liquid", "prompt-3", &globals).unwrap();
    let request = prompt.request
        // .with_model("gpt-4-turbo-2024-04-09")
        // .with_response_format(ResponseFormat::JSON_OBJECT)
        // .with_model(OpenAiModels::gpt_3_5_turbo_0125)
        .with_n(4);
        // .with_stream(true);
        // .with_stream(true)
        // .with_model("gpt-3.5-turbo");
    let results = text_api::client::ApiCallBuilder::default()
        .with_request_body(request)
        .with_api_key(api_key)
        .with_api_url(api_url)
        .with_logger(text_api::client::StdOutLogger::default())
        // .with_logger(text_api::client::FileLogger::new("logs/output.txt"))
        // .build_streaming_api_call()
        .build_batch_api_call()
        // .build_batch_api_call()
        .unwrap()
        .execute()
        .await;
    println!("{}", "DONE".cyan());
    match results {
        Ok(response) => {
            println!("{response:#?}");
            println!("{}", "-".repeat(80));
            for choise in response.choices.iter() {
                if let Some(content) = choise.message.content.as_ref() {
                    println!("{content}");
                    println!("{}", "-".repeat(80));
                }
            }
        }
        Err(x) => {
            println!("{}", "ERROR".red());
            println!("{x:?}");
        }
    }
    Ok(())
}
