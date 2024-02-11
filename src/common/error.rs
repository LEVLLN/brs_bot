#[derive(Debug, PartialEq)]
pub enum ProcessError<'a> {
    // Force stop process message
    Stop,
    // Skip current process go to next process
    Next,
    // Force stop process message for send feedback to user
    Feedback{message: &'a str},
}
