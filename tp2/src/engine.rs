use crate::errors::GameError;
use crate::scenario::{Ending, Scenario};
use crate::state::GameState;

#[derive(Debug, PartialEq, Eq)]
pub enum CommandOutcome {
    Continue,
    Exit,
    SceneEnded(Ending),
}

pub fn enter_scene(scenario: &Scenario, state: &mut GameState) -> Result<(), GameError> {
    let scene = scenario
        .scene_by_id(&state.current_scene)
        .ok_or_else(|| GameError::UnknownScene(state.current_scene.clone()))?;

    if let Some(delta) = scene.hp_delta {
        state.hp += delta;
    }

    if state.hp <= 0 {
        return Err(GameError::GameOver);
    }

    if let Some(item) = &scene.found_item {
        state.add_item(item.clone());
    }

    Ok(())
}

pub fn format_scene(scenario: &Scenario, state: &GameState) -> Option<String> {
    let scene = scenario.scene_by_id(&state.current_scene)?;
    let mut lines = vec![scene.title.clone(), scene.text.clone()];
    if !scene.choices.is_empty() {
        for (index, choice) in scene.choices.iter().enumerate() {
            lines.push(format!("{}. {}", index + 1, choice.label));
        }
    }
    Some(lines.join("\n"))
}

pub fn render_scene(scenario: &Scenario, state: &GameState) {
    if let Some(output) = format_scene(scenario, state) {
        println!("{output}");
    }
}
