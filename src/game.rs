use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct FoosGameData {
    time: u64,
    blue_goals: u32,
    red_goals: u32,
}

impl FoosGameData {
    fn new() -> FoosGameData {
        FoosGameData {
            time: 0,
            blue_goals: 0,
            red_goals: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FoosState {
    Reset,
    Running(FoosGameData),
    Paused(FoosGameData),
}

pub struct FoosStateMachine {
    max_score: u32,
    game_state: FoosState,
}

#[derive(Debug, Clone, Copy)]
pub enum FoosEvent {
    Reset,
    BallDrop,
    BlueGoal,
    RedGoal,
    NoEvent,
}

impl FoosStateMachine {
    pub fn new(max_score: u32) -> FoosStateMachine {
        FoosStateMachine {
            max_score,
            game_state: FoosState::Reset,
        }
    }

    pub fn get_game_data(&self) -> FoosGameData {
        match self.game_state {
            FoosState::Paused(game_data) => game_data.clone(),
            FoosState::Running(game_data) => game_data.clone(),
            FoosState::Reset => FoosGameData::new(),
        }
    }

    pub fn next(&mut self, event: FoosEvent) {
        match (self.game_state.clone(), event) {
            // Reset Game
            (_, FoosEvent::Reset) => self.game_state = FoosState::Reset,
            // Game Start
            (FoosState::Reset, FoosEvent::BallDrop) => {
                self.game_state = FoosState::Running(FoosGameData::new())
            }
            // Blue Goal
            (
                FoosState::Running(FoosGameData {
                    time,
                    blue_goals,
                    red_goals,
                }),
                FoosEvent::BlueGoal,
            ) => {
                if blue_goals >= self.max_score - 1 {
                    println!("\nBlue Wins!\n");
                    self.game_state = FoosState::Reset;
                } else {
                    self.game_state = FoosState::Paused(FoosGameData {
                        time,
                        blue_goals: blue_goals + 1,
                        red_goals,
                    });
                }
            }
            // Red Goal
            (
                FoosState::Running(FoosGameData {
                    time,
                    blue_goals,
                    red_goals,
                }),
                FoosEvent::RedGoal,
            ) => {
                if red_goals >= self.max_score - 1 {
                    println!("\nRed Wins!\n");
                    self.game_state = FoosState::Reset
                } else {
                    self.game_state = FoosState::Paused(FoosGameData {
                        time,
                        blue_goals,
                        red_goals: red_goals + 1,
                    })
                }
            }
            // Ball Drop
            (
                FoosState::Paused(FoosGameData {
                    time,
                    blue_goals,
                    red_goals,
                }),
                FoosEvent::BallDrop,
            ) => {
                self.game_state = FoosState::Running(FoosGameData {
                    time: time + 1,
                    blue_goals,
                    red_goals,
                })
            }
            // Other event when timer running -> timer elapse
            (
                FoosState::Running(FoosGameData {
                    time,
                    blue_goals,
                    red_goals,
                }),
                _,
            ) => {
                self.game_state = FoosState::Running(FoosGameData {
                    time: time + 1,
                    blue_goals,
                    red_goals,
                })
            }
            // Other event when timer running -> do nothing
            (
                FoosState::Paused(FoosGameData {
                    time,
                    blue_goals,
                    red_goals,
                }),
                _,
            ) => {
                self.game_state = FoosState::Paused(FoosGameData {
                    time,
                    blue_goals,
                    red_goals,
                })
            }
            // Something else, then just don't do anything
            (_, _) => {}
        }
    }
}
