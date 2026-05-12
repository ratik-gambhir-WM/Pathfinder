use crate::clients::helix::HelixClient;

pub async fn add_user(
    client: &HelixClient,
    name: Option<&String>,
    email: Option<&String>,
    api_key: Option<&String>,
) -> Result<(), String> {
    client.add_user(name, email, api_key).await
}

pub async fn get_user(client: &HelixClient, email: Option<&String>) -> Result<(), String> {
    client.get_user(email).await
}
