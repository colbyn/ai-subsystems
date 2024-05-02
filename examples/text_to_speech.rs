use ai_subsystems::audio_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::fs::read_to_string("secrets/open-ai.key").unwrap();
    let api_url = audio_api::client::URL::openai_v1_audio_speech();
    let client_configuration = audio_api::client::ClientConfigurationBuilder::default()
        .with_api_key(api_key)
        .with_api_url(api_url)
        .build()
        .unwrap();
    let request = audio_api::request::RequestBuilder::default()
        .with_model(audio_api::request::Model::tts_1())
        .with_input("The quick brown fox jumped over the lazy dog.")
        .with_voice(audio_api::request::Voice::alloy())
        .with_response_format(audio_api::request::ResponseFormat::mp3())
        .with_speed(1.0)
        .build()
        .unwrap();
    let response = request.execute(&client_configuration).await.unwrap();
    std::fs::write("text-to-speech-example.mp4", &response).unwrap();
    Ok(())
}
