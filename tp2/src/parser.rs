use crate::commands::{
    ChooseCommand, GameCommand, HelpCommand, InventoryCommand, LookCommand, QuitCommand,
    StatusCommand,
};
use crate::errors::ParseError;

pub fn parse_command(line: &str) -> Result<Box<dyn GameCommand>, ParseError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(ParseError::Empty);
    }

    let mut parts = trimmed.split_whitespace();
    let cmd = parts.next().unwrap_or_default().to_lowercase();

    match cmd.as_str() {
        "look" => Ok(Box::new(LookCommand)),
        "inventory" => Ok(Box::new(InventoryCommand)),
        "status" => Ok(Box::new(StatusCommand)),
        "help" => Ok(Box::new(HelpCommand)),
        "quit" => Ok(Box::new(QuitCommand)),
        "choose" => {
            let value = parts.next().ok_or(ParseError::MissingArgument(cmd))?;
            let index = value
                .parse::<usize>()
                .map_err(|_| ParseError::InvalidNumber(value.to_string()))?;
            Ok(Box::new(ChooseCommand { index }))
        }
        _ => Err(ParseError::UnknownCommand(cmd)),
    }
}
