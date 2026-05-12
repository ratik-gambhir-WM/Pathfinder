use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct InsertUserInput {
    pub name: String,
    pub email: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct GetUserByEmailInput {
    pub email: String,
}
