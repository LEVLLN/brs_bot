use crate::core::command::parse_command;
use crate::core::lexer::{tokenize, Token};
use crate::telegram::request::RequestPayload;

pub fn tokens_from_request(request: &RequestPayload) -> Option<Vec<Token>> {
    request.any_message().direct().ext.raw_text().map(tokenize)
}

pub fn handle_command<'a>(request: &RequestPayload) -> Option<&'a str> {
    let tokens = &tokens_from_request(request)?;
    let command_property = parse_command(tokens);
    println!("{:?}", command_property);
    Some("test")
}
