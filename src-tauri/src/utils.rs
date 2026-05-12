use std::env;

use tiktoken_rs::o200k_base;

pub fn get_token_count(input: impl AsRef<str>) -> usize {
    let bpe = o200k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(input.as_ref());
    tokens.len()
}

pub fn openai_api_key() -> Result<String, String> {
    env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY environment variable is not set".to_string())
}
