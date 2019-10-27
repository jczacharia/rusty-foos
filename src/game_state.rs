use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub time: u64,
    pub blue_goals: u32,
    pub red_goals: u32,
    pub ball_dropped: bool,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            time: 0,
            blue_goals: 0,
            red_goals: 0,
            ball_dropped: false,
        }
    }
}
