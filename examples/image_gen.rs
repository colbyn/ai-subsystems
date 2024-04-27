use ai_subsystems::images_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::fs::read_to_string("secrets/open-ai.key").unwrap();
    let api_url = images_api::client::URL::openai_v1_images_generations();
    let client_configuration = images_api::client::ClientConfigurationBuilder::default()
        .with_api_key(api_key)
        .with_api_url(api_url)
        .build()
        .unwrap();
    let request = images_api::request::RequestBuilder::default()
        .with_prompt("the european knight.")
        .with_model(images_api::request::Model::dall_e3())
        .with_response_format(images_api::request::ResponseFormat::url())
        .build()
        .unwrap();
    let response = request.execute(&client_configuration).await.unwrap();
    println!("RESULT: {:#?}", response);
    Ok(())
}
