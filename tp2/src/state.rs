#[derive(Debug, Clone)]
pub struct GameState {
    pub current_scene: String,
    pub hp: i32,
    pub inventory: Vec<String>,
}

impl GameState {
    pub fn new(start_scene: String, initial_hp: i32) -> Self {
        Self {
            current_scene: start_scene,
            hp: initial_hp,
            inventory: Vec::new(),
        }
    }

    pub fn has_item(&self, item: &str) -> bool {
        self.inventory.iter().any(|i| i == item)
    }

    pub fn add_item(&mut self, item: String) {
        if !self.has_item(&item) {
            self.inventory.push(item);
        }
    }
}
