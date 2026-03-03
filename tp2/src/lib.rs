mod commands;
mod engine;
mod errors;
mod parser;
mod scenario;
mod state;

use crate::engine::{CommandOutcome, enter_scene, render_scene};
use crate::errors::{AppError, GameError, ParseError};
use crate::parser::parse_command;
use crate::scenario::Scenario;
use crate::state::GameState;
use std::io::{self, Write};

pub fn run_game(story_path: &str) -> Result<(), AppError> {
    let scenario = Scenario::load_from_file(story_path)?;
    scenario.validate().map_err(AppError::Validation)?;

    let mut state = GameState::new(scenario.start_scene.clone(), scenario.initial_hp);

    enter_scene(&scenario, &mut state).map_err(|e| AppError::Io(e.to_string()))?;
    render_scene(&scenario, &state);

    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout()
            .flush()
            .map_err(|e| AppError::Io(e.to_string()))?;

        let mut line = String::new();
        if stdin
            .read_line(&mut line)
            .map_err(|e| AppError::Io(e.to_string()))?
            == 0
        {
            break;
        }

        let command = match parse_command(&line) {
            Ok(cmd) => cmd,
            Err(err) => {
                print_parse_error(err);
                continue;
            }
        };

        match command.execute(&scenario, &mut state) {
            Ok(CommandOutcome::Continue) => {}
            Ok(CommandOutcome::Exit) => break,
            Ok(CommandOutcome::SceneEnded(ending)) => {
                println!("End: {}", ending.as_label());
                break;
            }
            Err(GameError::GameOver) => {
                render_scene(&scenario, &state);
                println!("Game Over");
                break;
            }
            Err(err) => {
                println!("Error: {err}");
            }
        }
    }

    Ok(())
}

fn print_parse_error(err: ParseError) {
    match err {
        ParseError::Empty => {}
        ParseError::UnknownCommand(cmd) => println!("Unknown command: {cmd}"),
        ParseError::MissingArgument(cmd) => println!("Missing argument for: {cmd}"),
        ParseError::InvalidNumber(value) => println!("Invalid number: {value}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{ChooseCommand, GameCommand};
    use crate::engine::{CommandOutcome, format_scene};
    use crate::errors::GameError;
    use crate::scenario::Scenario;

    fn load_story() -> Scenario {
        let yaml = include_str!("../story.yaml");
        let scenario: Scenario = serde_yaml::from_str(yaml).expect("parse yaml");
        scenario.validate().expect("valid story");
        scenario
    }

    #[test]
    fn path_to_victory() {
        let scenario = load_story();
        let mut state = GameState::new(scenario.start_scene.clone(), scenario.initial_hp);
        enter_scene(&scenario, &mut state).expect("enter start");

        let steps = [1, 1, 2, 1, 1, 3];
        let mut outcome = CommandOutcome::Continue;
        for step in steps {
            let cmd = ChooseCommand { index: step };
            outcome = cmd.execute(&scenario, &mut state).expect("choose");
        }

        match outcome {
            CommandOutcome::SceneEnded(ending) => {
                assert_eq!(ending.as_label(), "Victory");
            }
            _ => panic!("expected ending"),
        }
    }

    #[test]
    fn invalid_choice() {
        let scenario = load_story();
        let mut state = GameState::new(scenario.start_scene.clone(), scenario.initial_hp);
        enter_scene(&scenario, &mut state).expect("enter start");

        let cmd = ChooseCommand { index: 99 };
        let err = cmd.execute(&scenario, &mut state).unwrap_err();
        assert_eq!(err, GameError::InvalidChoice);
    }

    #[test]
    fn missing_item() {
        let scenario = load_story();
        let mut state = GameState::new("hall".to_string(), scenario.initial_hp);
        enter_scene(&scenario, &mut state).expect("enter hall");

        let cmd = ChooseCommand { index: 3 };
        let err = cmd.execute(&scenario, &mut state).unwrap_err();
        assert_eq!(err, GameError::MissingItem("badge".to_string()));
    }

    #[test]
    fn hp_reaches_zero() {
        let scenario = load_story();
        let mut state = GameState::new("collapse".to_string(), 1);
        let err = enter_scene(&scenario, &mut state).unwrap_err();
        assert_eq!(err, GameError::GameOver);
    }

    #[test]
    fn invalid_scenario_validation() {
        let yaml = r#"
start_scene: missing
initial_hp: 10
scenes:
  - id: start
    title: Start
    text: text
    choices: []
"#;
        let scenario: Scenario = serde_yaml::from_str(yaml).expect("parse yaml");
        let err = scenario.validate().unwrap_err();
        assert!(matches!(
            err,
            crate::errors::ValidationError::StartSceneMissing(_)
        ));
    }

    #[test]
    fn validation_duplicate_scene_ids() {
        let yaml = r#"
start_scene: start
initial_hp: 10
scenes:
  - id: start
    title: A
    text: a
    choices: []
  - id: start
    title: B
    text: b
    choices: []
"#;
        let scenario: Scenario = serde_yaml::from_str(yaml).expect("parse yaml");
        let err = scenario.validate().unwrap_err();
        assert!(matches!(
            err,
            crate::errors::ValidationError::DuplicateSceneId(_)
        ));
    }

    #[test]
    fn validation_missing_choice_target() {
        let yaml = r#"
start_scene: start
initial_hp: 10
scenes:
  - id: start
    title: A
    text: a
    choices:
      - label: Go
        next: missing
"#;
        let scenario: Scenario = serde_yaml::from_str(yaml).expect("parse yaml");
        let err = scenario.validate().unwrap_err();
        assert!(matches!(
            err,
            crate::errors::ValidationError::MissingScene(_)
        ));
    }

    #[test]
    fn look_renders_scene_text_and_choices() {
        let scenario = load_story();
        let state = GameState::new("entrance".to_string(), scenario.initial_hp);
        let output = format_scene(&scenario, &state).expect("format scene");
        assert!(output.contains("Porte Principale"));
        assert!(output.contains("La pluie frappe les vitres"));
        assert!(output.contains("1. Entrer dans le hall"));
        assert!(output.contains("2. Renoncer et partir dans la rue"));
    }
}
