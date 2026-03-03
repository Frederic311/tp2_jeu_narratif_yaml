use crate::engine::{CommandOutcome, enter_scene, render_scene};
use crate::errors::GameError;
use crate::scenario::Scenario;
use crate::state::GameState;

pub trait GameCommand {
    fn execute(
        &self,
        scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError>;
}

pub struct LookCommand;

impl GameCommand for LookCommand {
    fn execute(
        &self,
        scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        render_scene(scenario, state);
        Ok(CommandOutcome::Continue)
    }
}

pub struct InventoryCommand;

impl GameCommand for InventoryCommand {
    fn execute(
        &self,
        _scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        if state.inventory.is_empty() {
            println!("Inventory: empty");
        } else {
            println!("Inventory: {}", state.inventory.join(", "));
        }
        Ok(CommandOutcome::Continue)
    }
}

pub struct StatusCommand;

impl GameCommand for StatusCommand {
    fn execute(
        &self,
        _scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        println!("HP: {} | Scene: {}", state.hp, state.current_scene);
        Ok(CommandOutcome::Continue)
    }
}

pub struct QuitCommand;

impl GameCommand for QuitCommand {
    fn execute(
        &self,
        _scenario: &Scenario,
        _state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        Ok(CommandOutcome::Exit)
    }
}

pub struct ChooseCommand {
    pub index: usize,
}

impl GameCommand for ChooseCommand {
    fn execute(
        &self,
        scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        let scene = scenario
            .scene_by_id(&state.current_scene)
            .ok_or_else(|| GameError::UnknownScene(state.current_scene.clone()))?;

        if self.index == 0 || self.index > scene.choices.len() {
            return Err(GameError::InvalidChoice);
        }

        let choice = &scene.choices[self.index - 1];
        if let Some(required) = &choice.required_item
            && !state.has_item(required)
        {
            return Err(GameError::MissingItem(required.clone()));
        }

        state.current_scene = choice.next.clone();
        enter_scene(scenario, state)?;
        render_scene(scenario, state);

        let new_scene = scenario
            .scene_by_id(&state.current_scene)
            .ok_or_else(|| GameError::UnknownScene(state.current_scene.clone()))?;
        if let Some(ending) = &new_scene.ending {
            return Ok(CommandOutcome::SceneEnded(ending.clone()));
        }

        Ok(CommandOutcome::Continue)
    }
}
