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
        for (left_eq, right_eq) in [
            (Text("строкА"), Text("Строка")),
            (Text("строка"), Text("СТРОКА")),
            (Text("строка"), Text("строка")),
            (Text("string"), Text("STRING")),
            (Text("strinG"), Text("String")),
            (Text("string"), Text("string")),
        ] {
            assert_eq!(left_eq, right_eq);
        }
    }

    #[test]
    fn test_tokenize() {
        for (input, output) in [
            ("some_str", vec![Text("some_str")]),
            (
                "some_str some_another_str",
                vec![Text("some_str"), Text("some_another_str")],
            ),
            (
                "some_str\rsome_another_str",
                vec![Text("some_str"), Text("some_another_str")],
            ),
            (
                "some_str \n some_another_str",
                vec![Text("some_str"), Newline, Text("some_another_str")],
            ),
            (
                "some_str\n \n some_another_str",
                vec![Text("some_str"), Newline, Newline, Text("some_another_str")],
            ),
            (
                "\nsome_str\n \n some_another_str",
                vec![
                    Newline,
                    Text("some_str"),
                    Newline,
                    Newline,
                    Text("some_another_str"),
                ],
            ),
            ("", vec![]),
        ] {
            assert_eq!(tokenize(input), output);
        }
    }
}
