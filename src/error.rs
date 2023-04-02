use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct CommandError {
    message: String,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CommandError {}

impl CommandError {
    pub fn new(message: String) -> CommandError {
        CommandError { message }
    }
}
