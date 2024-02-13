pub mod text_api;
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), text_api::client::Error> {
    // let api_key = std::fs::read_to_string("secrets/open-ai.key").unwrap();
    // let api_url = text_api::client::URL::OPEN_AI_CHAT_COMPLETIONS;
    let api_key = std::fs::read_to_string("secrets/octo-ai.key").unwrap();
    let api_url = text_api::client::URL::OCTO_AI_CHAT_COMPLETIONS;
    let prompt = text_api::xml_dsl::Prompt::open("assets/basic.prompt.liquid", "question-1").unwrap();
    let request = prompt.request
        .with_model("mixtral-8x7b-instruct-fp16")
        .with_stream(true);
        // .with_stream(true)
        // .with_model("gpt-3.5-turbo");
    let results = text_api::client::ApiCallBuilder::default()
        .with_request_body(request)
        .with_api_key(api_key)
        .with_api_url(api_url)
        .with_stderr_logger()
        .build_streaming_api_call()
        // .build_batch_api_call()
        .unwrap()
        .execute()
        .await;
    println!("{}", "DONE".cyan());
    match results {
        Ok(collection) => {
            println!("{collection:#?}");
            // if let Some(output) = collection.content(0) {
            //     println!("{output}");
            // } else {
            //     println!("None");
            // }
        }
        Err(x) => {
            println!("{}", "ERROR".red());
            println!("{x:?}");
        }
    }
    Ok(())
}
