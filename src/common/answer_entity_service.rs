use crate::common::lexer::{tokens_to_string, Token};
use sqlx::PgPool;

async fn search_tokens<'a>(pool: &PgPool, tokens: &'a [Token<'a>]) {
    let mut keys: Vec<String> = vec![];
    if tokens.len() > 1 {
        keys.push(tokens_to_string(tokens, false));
    }
    tokens.iter().for_each(|x| if let Token::Word(word) = x {keys.push(word.to_string())});
       
    println!("{:?}", keys)
}
