use unicase::UniCase;
use Token::{Newline, Punctuation, Word};

#[derive(Copy, Clone, Debug)]
pub enum Token<'a> {
    Newline,
    Word(&'a str),
    Punctuation(&'a str),
}

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Word(a), &Word(b)) => UniCase::new(a) == UniCase::new(b),
            (&Punctuation(a), &Punctuation(b)) => a == b,
            (&Newline, &Newline) => true,
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
            "," | "|" | "." | ":" | "!" | "?" | "-" | "(" | ")" | "[" | "]" | ";" | "'" => {
                token_list.push(Punctuation(&word[word.len() - 1..]));
            }
            x if x.ends_with('\n') => {
                token_list.push(Word(&word[..word.len() - 1]));
                token_list.push(Newline);
            }
            _ => token_list.push(Word(word)),
        });
    token_list
}

#[cfg(test)]
mod tests {
    use crate::core::lexer::{tokenize, Token::*};

    #[test]
    fn test_text_equals() {
        for (left_eq, right_eq) in [
            (Word("строкА"), Word("Строка")),
            (Word("строка"), Word("СТРОКА")),
            (Word("строка"), Word("строка")),
            (Word("string"), Word("STRING")),
            (Word("strinG"), Word("String")),
            (Word("string"), Word("string")),
        ] {
            assert_eq!(left_eq, right_eq);
        }
    }

    #[test]
    fn test_tokenize() {
        for (input, output) in [
            ("some_str", vec![Word("some_str")]),
            (
                "some - str",
                vec![Word("some"), Punctuation("-"), Word("str")],
            ),
            (
                "some_str some_another_str",
                vec![Word("some_str"), Word("some_another_str")],
            ),
            (
                "some_str\rsome_another_str",
                vec![Word("some_str"), Word("some_another_str")],
            ),
            (
                "some_str \n some_another_str",
                vec![Word("some_str"), Newline, Word("some_another_str")],
            ),
            (
                "some_str\n \n some_another_str",
                vec![Word("some_str"), Newline, Newline, Word("some_another_str")],
            ),
            (
                "\nsome_str\n \n some_another_str",
                vec![
                    Newline,
                    Word("some_str"),
                    Newline,
                    Newline,
                    Word("some_another_str"),
                ],
            ),
            ("", vec![]),
        ] {
            assert_eq!(tokenize(input), output);
        }
    }
}
