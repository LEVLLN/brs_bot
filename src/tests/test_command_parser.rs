#[cfg(test)]
mod tests {
    use Command::*;

    use crate::common::command_parser::{
        is_bot_call, parse_command, Command, CommandContainer, ControlItem,
    };
    use crate::common::error::ProcessError;
    use crate::common::lexer::{tokenize, Token};

    #[test]
    fn test_is_bot_call() {
        [
            (Token::Word("хлеб"), true),
            (Token::Word("Хлеб"), true),
            (Token::Word("ХЛЕБ"), true),
            (Token::Word("Хлебушек"), true),
            (Token::Word("хлебушек"), true),
            (Token::Word("bread_bot"), true),
            (Token::Word("bread"), true),
            (Token::Word("BREAD_BOT"), true),
            (Token::Word("хлебушкек"), false),
            (Token::Newline, false),
            (Token::Punctuation("-"), false),
        ]
        .iter()
        .for_each(|(input, output)| {
            assert_eq!(is_bot_call(input), *output);
        })
    }

    #[test]
    fn test_skip_command() {
        [
            ("", Err(ProcessError::Next)),
            ("some_wrong_text", Err(ProcessError::Next)),
            ("хлеб", Err(ProcessError::Next)),
            ("Хлеб", Err(ProcessError::Next)),
        ]
        .iter()
        .for_each(|(input, output)| assert_eq!(parse_command(&tokenize(input), false), *output));
    }

    #[test]
    fn test_parse_add() {
        [
            (
                "хлеб добавь",
                Err(ProcessError::Feedback {
                    message: "Необходимо указать объект для редактирования",
                }),
            ),
            (
                "хлеб добавь бред",
                Err(ProcessError::Feedback {
                    message: "Необходимо указать значения",
                }),
            ),
            (
                "хлеб добавь подстроку утка",
                Err(ProcessError::Feedback {
                    message: "Указан недопустимый объект для редактирования",
                }),
            ),
            (
                "хлеб добавь неподстроку ?",
                Err(ProcessError::Feedback {
                    message: "Необходимо указать объект для редактирования",
                }),
            ),
            (
                "хлеб добавь бред утка",
                Ok(CommandContainer {
                    command: &Add,
                    command_aliases: &[Token::Word("добавь")],
                    control_item: Some(&ControlItem::MorphWord),
                    split_values: true,
                    rest: &[Token::Word("утка")],
                }),
            ),
            (
                "хлеб добавь БРЕД утка",
                Ok(CommandContainer {
                    command: &Add,
                    command_aliases: &[Token::Word("добавь")],
                    control_item: Some(&ControlItem::MorphWord),
                    split_values: true,
                    rest: &[Token::Word("утка")],
                }),
            ),
        ]
        .iter()
        .for_each(|(input, output)| assert_eq!(parse_command(&tokenize(input), false), *output));
    }

    #[test]
    fn test_parse_help() {
        [
            (
                "хлеб хелп",
                Ok(CommandContainer {
                    command: &Help,
                    command_aliases: &[Token::Word("хелп")],
                    control_item: None,
                    split_values: false,
                    rest: &[],
                }),
            ),
            (
                "хлеб хелп бред",
                Ok(CommandContainer {
                    command: &Help,
                    command_aliases: &[Token::Word("хелп")],
                    control_item: None,
                    split_values: false,
                    rest: &[Token::Word("бред")],
                }),
            ),
            (
                "хлеб хелп бред бред",
                Ok(CommandContainer {
                    command: &Help,
                    command_aliases: &[Token::Word("хелп")],
                    control_item: None,
                    split_values: false,
                    rest: &[Token::Word("бред"), Token::Word("бред")],
                }),
            ),
        ]
        .iter()
        .for_each(|(input, output)| assert_eq!(parse_command(&tokenize(input), false), *output));
    }

    #[test]
    fn test_parse_show_with_reply() {
        [
            (
                "хлеб покажи",
                Ok(CommandContainer {
                    command: &Show,
                    command_aliases: &[Token::Word("покажи")],
                    control_item: Some(&ControlItem::Substring),
                    split_values: false,
                    rest: &[],
                }),
                true,
            ),
            (
                "хлеб покажи",
                Err(ProcessError::Feedback {
                    message: "Необходимо выбрать сообщение в ответ",
                }),
                false,
            ),
            (
                "хлеб покажи подстроку",
                Ok(CommandContainer {
                    command: &Show,
                    command_aliases: &[Token::Word("покажи")],
                    control_item: Some(&ControlItem::Substring),
                    split_values: false,
                    rest: &[],
                }),
                true,
            ),
            (
                "хлеб покажи подстроку",
                Err(ProcessError::Feedback {
                    message: "Необходимо выбрать сообщение в ответ",
                }),
                false,
            ),
            (
                "хлеб покажи подстроки",
                Ok(CommandContainer {
                    command: &Show,
                    command_aliases: &[Token::Word("покажи")],
                    control_item: Some(&ControlItem::Substring),
                    split_values: false,
                    rest: &[],
                }),
                true,
            ),
            (
                "хлеб покажи подстроки",
                Err(ProcessError::Feedback {
                    message: "Необходимо выбрать сообщение в ответ",
                }),
                false,
            ),
            (
                "хлеб покажи подстроку бред",
                Ok(CommandContainer {
                    command: &Show,
                    command_aliases: &[Token::Word("покажи")],
                    control_item: Some(&ControlItem::Substring),
                    split_values: false,
                    rest: &[Token::Word("бред")],
                }),
                true,
            ),
            (
                "хлеб покажи подстроку бред",
                Err(ProcessError::Feedback {
                    message: "Необходимо выбрать сообщение в ответ",
                }),
                false,
            ),
            (
                "хлеб покажи обед",
                Ok(CommandContainer {
                    command: &Show,
                    command_aliases: &[Token::Word("покажи")],
                    control_item: Some(&ControlItem::Substring),
                    split_values: false,
                    rest: &[Token::Word("обед")],
                }),
                true,
            ),
            (
                "хлеб покажи бред",
                Ok(CommandContainer {
                    command: &Show,
                    command_aliases: &[Token::Word("покажи")],
                    control_item: Some(&ControlItem::MorphWord),
                    split_values: false,
                    rest: &[],
                }),
                true,
            ),
            (
                "хлеб покажи ключи обед",
                Ok(CommandContainer {
                    command: &Show,
                    command_aliases: &[Token::Word("покажи")],
                    control_item: Some(&ControlItem::KeyWord),
                    split_values: false,
                    rest: &[Token::Word("обед")],
                }),
                true,
            ),
        ]
        .iter()
        .for_each(|(input, output, has_reply)| {
            assert_eq!(parse_command(&tokenize(input), *has_reply), *output)
        });
    }

    #[test]
    fn test_parse_who() {
        [
            (
                "хлеб кто булочка?",
                Ok(CommandContainer {
                    command: &Who,
                    command_aliases: &[Token::Word("кто")],
                    control_item: None,
                    split_values: false,
                    rest: &[Token::Word("булочка"), Token::Punctuation("?")],
                }),
            ),
            (
                "хлеб КТО булочка?",
                Ok(CommandContainer {
                    command: &Who,
                    command_aliases: &[Token::Word("КТО")],
                    control_item: None,
                    split_values: false,
                    rest: &[Token::Word("булочка"), Token::Punctuation("?")],
                }),
            ),
            (
                "ХЛЕБ кто булочка?",
                Ok(CommandContainer {
                    command: &Who,
                    command_aliases: &[Token::Word("кто")],
                    control_item: None,
                    split_values: false,
                    rest: &[Token::Word("булочка"), Token::Punctuation("?")],
                }),
            ),
            (
                "хлеб who булочка?",
                Ok(CommandContainer {
                    command: &Who,
                    command_aliases: &[Token::Word("who")],
                    control_item: None,
                    split_values: false,
                    rest: &[Token::Word("булочка"), Token::Punctuation("?")],
                }),
            ),
            (
                "хлеб кто?",
                Ok(CommandContainer {
                    command: &Who,
                    command_aliases: &[Token::Word("кто")],
                    control_item: None,
                    split_values: false,
                    rest: &[Token::Punctuation("?")],
                }),
            ),
            (
                "хлеб кто",
                Ok(CommandContainer {
                    command: &Who,
                    command_aliases: &[Token::Word("кто")],
                    control_item: None,
                    split_values: false,
                    rest: &[],
                }),
            ),
        ]
        .iter()
        .for_each(|(input, output)| assert_eq!(parse_command(&tokenize(input), false), *output))
    }

    #[test]
    fn test_parse_answer_change() {
        [
            (
                "хлеб процент срабатывания бреда",
                Ok(CommandContainer {
                    command: &AnswerChance,
                    command_aliases: &[Token::Word("процент"), Token::Word("срабатывания")],
                    control_item: Some(&ControlItem::MorphWord),
                    split_values: false,
                    rest: &[],
                }),
            ),
            (
                "хлеб процент",
                Ok(CommandContainer {
                    command: &AnswerChance,
                    command_aliases: &[Token::Word("процент")],
                    control_item: Some(&ControlItem::Substring),
                    split_values: false,
                    rest: &[],
                }),
            ),
            (
                "хлеб процент 20",
                Ok(CommandContainer {
                    command: &AnswerChance,
                    command_aliases: &[Token::Word("процент")],
                    control_item: Some(&ControlItem::Substring),
                    split_values: false,
                    rest: &[Token::Word("20")],
                }),
            ),
        ]
        .iter()
        .for_each(|(input, output)| assert_eq!(parse_command(&tokenize(input), false), *output))
    }

    #[test]
    fn test_parse_check() {
        [
            (
                "хлеб проверь",
                Err(ProcessError::Feedback {
                    message: "Необходимо указать значения",
                }),
            ),
            (
                "хлеб проверь нога",
                Ok(CommandContainer {
                    command: &Check,
                    command_aliases: &[Token::Word("проверь")],
                    control_item: Some(&ControlItem::Substring),
                    split_values: false,
                    rest: &[Token::Word("нога")],
                }),
            ),
            (
                "хлеб проверь триггер нога",
                Ok(CommandContainer {
                    command: &Check,
                    command_aliases: &[Token::Word("проверь")],
                    control_item: Some(&ControlItem::Trigger),
                    split_values: false,
                    rest: &[Token::Word("нога")],
                }),
            ),
        ]
        .iter()
        .for_each(|(input, output)| assert_eq!(parse_command(&tokenize(input), false), *output))
    }
}
