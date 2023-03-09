use std::fmt::Display;

#[derive(Debug)]
pub enum CError {
    IncorrectNumArgs { expected: usize, found: usize },
    OpNotFound,
}

impl Display for CError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CError::IncorrectNumArgs { expected, found } => {
                write!(f, "Expected {expected} arguments, found {found}.")
            }
            CError::OpNotFound => write!(f, "Operation with this name not found."),
        }
    }
}
