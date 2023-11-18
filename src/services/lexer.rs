use unicase::UniCase;

use Token::Text;

#[derive(Copy, Clone, Debug)]
pub enum Token<'a> {
    Newline,
    Text(&'a str),
}

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Text(a), &Text(b)) => UniCase::new(a) == UniCase::new(b),
            (&Token::Newline, &Token::Newline) => true,
            _ => false,
        }
    }
}

pub fn tokenize(text: &str) -> Vec<Token> {
    use Token::*;
    let mut token_list = vec![];
    text.split_inclusive('\n')
        .flat_map(|line| line.split([' ', '\r']))
        .filter(|words| !words.is_empty())
        .for_each(|word| match word {
            "\n" => token_list.push(Newline),
            x if x.ends_with('\n') => {
                token_list.push(Text(&word[..word.len() - 1]));
                token_list.push(Newline);
            }
            _ => token_list.push(Text(word)),
        });
    token_list
}

#[cfg(test)]
mod tests {
    use crate::services::lexer::{tokenize, Token::*};

    #[test]
    fn test_text_equals() {
        assert_eq!(Text("строкА"), Text("Строка"));
    }

    #[test]
    fn test_simple_str() {
        assert_eq!(tokenize("some_str"), [Text("some_str")]);
    }

    #[test]
    fn test_two_str() {
        assert_eq!(
            tokenize("some_str some_another_str"),
            [Text("some_str"), Text("some_another_str")]
        );
    }

    #[test]
    fn test_carriage_str() {
        assert_eq!(
            tokenize("some_str\rsome_another_str"),
            [Text("some_str"), Text("some_another_str")]
        )
    }

    #[test]
    fn test_simple_new_line() {
        assert_eq!(
            tokenize("some_str \n some_another_str"),
            [Text("some_str"), Newline, Text("some_another_str")]
        );
    }

    #[test]
    fn test_ends_new_line() {
        assert_eq!(
            tokenize("some_str\n \n some_another_str"),
            [Text("some_str"), Newline, Newline, Text("some_another_str")]
        );
    }

    #[test]
    fn test_starts_new_line() {
        assert_eq!(
            tokenize("\nsome_str\n \n some_another_str"),
            [
                Newline,
                Text("some_str"),
                Newline,
                Newline,
                Text("some_another_str")
            ]
        );
    }

    #[test]
    fn test_empty_str() {
        assert_eq!(tokenize(""), [])
    }
}