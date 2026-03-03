use crate::errors::ValidationError;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Scenario {
    pub start_scene: String,
    pub initial_hp: i32,
    pub scenes: Vec<Scene>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Scene {
    pub id: String,
    pub title: String,
    pub text: String,
    #[serde(default)]
    pub choices: Vec<Choice>,
    pub found_item: Option<String>,
    pub hp_delta: Option<i32>,
    pub ending: Option<Ending>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Choice {
    pub label: String,
    pub next: String,
    pub required_item: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Ending {
    Victory,
    Escape,
    Defeat,
}

impl Ending {
    pub fn as_label(&self) -> &'static str {
        match self {
            Ending::Victory => "Victory",
            Ending::Escape => "Escape",
            Ending::Defeat => "Defeat",
        }
    }
}

impl Scenario {
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_yaml::from_str(&content).map_err(|e| e.to_string())
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut ids = HashSet::new();
        for scene in &self.scenes {
            if !ids.insert(scene.id.clone()) {
                return Err(ValidationError::DuplicateSceneId(scene.id.clone()));
            }
        }

        if !ids.contains(&self.start_scene) {
            return Err(ValidationError::StartSceneMissing(self.start_scene.clone()));
        }

        for scene in &self.scenes {
            for choice in &scene.choices {
                if !ids.contains(&choice.next) {
                    return Err(ValidationError::MissingScene(choice.next.clone()));
                }
            }
        }

        Ok(())
    }

    pub fn scene_by_id(&self, id: &str) -> Option<&Scene> {
        self.scenes.iter().find(|scene| scene.id == id)
    }
}
