use tiktoken_rs::{o200k_base, Rank};


pub fn get_token_count(input: Option<&str>) -> usize {
    let bpe = o200k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(
        input.unwrap()
    );
     tokens.len()
}

pub fn gen_tokens(content: &str) -> Vec<Rank> {
    let bpe = o200k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(
        content
    );
    tokens
}