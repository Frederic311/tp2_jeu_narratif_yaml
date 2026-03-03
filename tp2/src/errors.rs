use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    UnknownCommand(String),
    MissingArgument(String),
    InvalidNumber(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameError {
    InvalidChoice,
    MissingItem(String),
    UnknownScene(String),
    GameOver,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InvalidChoice => write!(f, "Invalid choice"),
            GameError::MissingItem(item) => write!(f, "Missing required item: {item}"),
            GameError::UnknownScene(id) => write!(f, "Unknown scene: {id}"),
            GameError::GameOver => write!(f, "Game over"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValidationError {
    StartSceneMissing(String),
    DuplicateSceneId(String),
    MissingScene(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::StartSceneMissing(id) => {
                write!(f, "start_scene does not exist: {id}")
            }
            ValidationError::DuplicateSceneId(id) => write!(f, "duplicate scene id: {id}"),
            ValidationError::MissingScene(id) => write!(f, "missing scene for choice: {id}"),
        }
    }
}
