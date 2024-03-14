use unicase::UniCase;

use Token::{Newline, Punctuation, Symbol, Word};

#[derive(Copy, Clone, Debug)]
pub enum Token<'a> {
    Newline,
    Word(&'a str),
    Punctuation(&'a str),
    Symbol(&'a str),
}

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Word(a), &Word(b)) => UniCase::new(a) == UniCase::new(b),
            (&Punctuation(a), &Punctuation(b)) => a == b,
            (&Symbol(a), &Symbol(b)) => a == b,
            (&Newline, &Newline) => true,
            _ => false,
        }
    }
}

pub fn tokens_to_string<'a>(tokens: &'a [Token<'a>], remove_question_mark: bool) -> String {
    tokens
        .iter()
        .enumerate()
        .fold(String::new(), |result, (index, token)| match token {
            Newline => result + "\n",
            Word(word) if result.is_empty() => result + word,
            Word(word) => result + " " + word,
            Punctuation(punct)
                if remove_question_mark && punct.eq(&"?") && tokens.len() - 1 == index =>
            {
                result
            }
            Punctuation(punct) => result + punct,
            Symbol(symbol) if result.is_empty() => result + symbol,
            Symbol(symbol) => result + " " + symbol,
        })
}

pub fn normalize_text(text: String) -> String{
    text.to_ascii_lowercase().replace('ё', "е")
}

pub fn tokenize(text: &str) -> Vec<Token> {
    use Token::*;
    let mut token_list = vec![];
    text.split_inclusive('\n')
        .flat_map(|line| line.split([' ', '\r']))
        .filter(|words| !words.is_empty())
        .for_each(|word| match word {
            "\n" => token_list.push(Newline),
            x if x.chars().all(|x| !x.is_alphanumeric()) => {
                token_list.push(Symbol(&word[word.len() - 1..]));
            }
            x if x.ends_with('\n') => {
                token_list.push(Word(&word[..word.len() - 1]));
                token_list.push(Newline);
            }
            x if !x.chars().last().unwrap().is_alphanumeric() => {
                token_list.push(Word(&word[..word.len() - 1]));
                token_list.push(Punctuation(&word[word.len() - 1..]));
            }
            _ => token_list.push(Word(word)),
        });
    token_list
}

#[cfg(test)]
mod tests {
    use crate::common::lexer::{tokenize, Token::*};

    #[test]
    fn test_text_equals() {
        [
            (Word("строкА"), Word("Строка")),
            (Word("строка"), Word("СТРОКА")),
            (Word("строка"), Word("строка")),
            (Word("string"), Word("STRING")),
            (Word("strinG"), Word("String")),
            (Word("string"), Word("string")),
        ]
        .iter()
        .for_each(|(left_eq, right_eq)| assert_eq!(left_eq, right_eq));
    }

    #[test]
    fn test_tokenize() {
        [
            ("some_str", vec![Word("some_str")]),
            ("some_str?", vec![Word("some_str"), Punctuation("?")]),
            ("?some_str", vec![Word("?some_str")]),
            ("some - str", vec![Word("some"), Symbol("-"), Word("str")]),
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
        ]
        .iter()
        .for_each(|(input, output)| assert_eq!(tokenize(input), *output));
    }
}
