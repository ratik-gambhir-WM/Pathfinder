use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize)]
struct EmbeddingRequest<'a> {
    model: &'a str,
    input: Vec<&'a str>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
    model: String,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    index: usize,
    embedding: Vec<f32>,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    total_tokens: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable is not set");

    let chunks = vec![
        "The architecture is designed around a modular service model where each service owns a clearly defined business capability. This approach improves maintainability by reducing coupling between teams and application components. Services communicate through well-defined APIs for synchronous workflows and publish events for asynchronous updates, allowing the platform to scale and evolve without requiring tightly coordinated releases.",

        "Data management follows a service-owned persistence pattern, where each domain service is responsible for its own schema, storage technology, and access rules. Customer profile data is stored in a relational database to support consistency and transactional integrity, while interaction history is stored in a document database to accommodate flexible event payloads. Aggregated data is periodically streamed into an analytics warehouse for reporting and business intelligence.",

        "Security controls are applied across identity, network, application, and data layers. Users authenticate through an enterprise identity provider, while APIs are protected using token-based authorization and role-based access policies. Sensitive information is encrypted in transit and at rest, secrets are stored in a managed vault, and audit logs are retained for operational monitoring, compliance reviews, and incident investigation.",
    ];
    let request_body = EmbeddingRequest {
        model: "text-embedding-3-small",
        input: chunks,
    };

    let client = Client::new();

    let response = client
        .post("https://api.openai.com/v1/embeddings")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await?
        .error_for_status()?
        .json::<EmbeddingResponse>()
        .await?;

    for item in response.data {
        println!(
            "Embedding {} has {} dimensions",
            item.index,
            item.embedding.len()
        );
        println!("Embedding {} values: {:?}", item.index, item.embedding);
    }

    println!("Model: {}", response.model);
    println!("Prompt tokens: {}", response.usage.prompt_tokens);
    println!("Total tokens: {}", response.usage.total_tokens);

    Ok(())
}
