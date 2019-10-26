#[derive(Debug, Copy, Clone)]
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

    pub fn reset(&mut self) {
        self.time = 0;
        self.blue_goals = 0;
        self.red_goals = 0;
        self.ball_dropped = false;
    }
}
