use ai_subsystems::text_api;
// use ai_subsystems::text_api::request::OpenAiModels;
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::fs::read_to_string("secrets/mistral-ai.key").unwrap();
    let api_url = text_api::client::URL::MISTRAL_AI_CHAT_COMPLETIONS;
    let globals = text_api::xml_dsl::object!({});
    let prompt = text_api::xml_dsl::Prompt::open("assets/basic.prompt.liquid", "question-1", &globals).unwrap();
    let request = prompt.request
        // .with_model("gpt-3.5-turbo-0125")
        .with_model("open-mixtral-8x22b")
        .with_stream(true);
        // .with_stream(true)
        // .with_model("gpt-3.5-turbo");
    let results = text_api::client::ApiCallBuilder::default()
        .with_request_body(request)
        .with_api_key(api_key)
        .with_api_url(api_url)
        .with_logger(text_api::client::StdOutLogger::default())
        // .with_logger(text_api::client::FileLogger::new("logs/output.txt"))
        .build_streaming_api_call()
        // .build_batch_api_call()
        .unwrap()
        .execute()
        .await;
    println!("{}", "DONE".cyan());
    match results {
        Ok(collection) => {
            println!("{collection:#?}");
            if let Some(output) = collection.content(0) {
                println!("{output}");
            }
        }
        Err(x) => {
            println!("{}", "ERROR".red());
            println!("{x:?}");
        }
    }
    Ok(())
}
