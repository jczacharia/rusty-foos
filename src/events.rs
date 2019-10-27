use rppal::gpio::{Gpio, InputPin, Level, Trigger};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::game_state::GameState;

#[derive(Serialize, Deserialize)]
pub struct Config {
    blue_goal_pin: u8,
    red_goal_pin: u8,
    ball_drop_1_pin: u8,
    ball_drop_2_pin: u8,
    pub port: u32,
}

pub struct FoosInputPins {
    blue_goal: InputPin,
    red_goal: InputPin,
    ball_drop_1: InputPin,
    ball_drop_2: InputPin,
}

impl FoosInputPins {
    pub fn new(config: &Config) -> Result<FoosInputPins, Box<dyn std::error::Error>> {
        let blue_goal = Gpio::new()?.get(config.blue_goal_pin)?.into_input_pullup();
        let red_goal = Gpio::new()?.get(config.red_goal_pin)?.into_input_pullup();
        let ball_drop_1 = Gpio::new()?
            .get(config.ball_drop_1_pin)?
            .into_input_pullup();
        let ball_drop_2 = Gpio::new()?
            .get(config.ball_drop_2_pin)?
            .into_input_pullup();

        Ok(FoosInputPins {
            blue_goal,
            red_goal,
            ball_drop_1,
            ball_drop_2,
        })
    }

    pub fn init_interrupts(
        &mut self,
        event_buffer: &Arc<Mutex<GameState>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Blue Goal Event
        let bg: Arc<Mutex<GameState>> = Arc::clone(event_buffer);
        self.blue_goal
            .set_async_interrupt(Trigger::FallingEdge, move |_: Level| {
                let mut events = bg.lock().unwrap();
                (*events).blue_goals += 1;
            })?;

        // Red Goal Event
        let rg: Arc<Mutex<GameState>> = Arc::clone(event_buffer);
        self.red_goal
            .set_async_interrupt(Trigger::FallingEdge, move |_: Level| {
                let mut events = rg.lock().unwrap();
                (*events).red_goals += 1;
            })?;

        // Ball Drop Goal Event 1
        let bd_1: Arc<Mutex<GameState>> = Arc::clone(event_buffer);
        self.ball_drop_1
            .set_async_interrupt(Trigger::FallingEdge, move |_: Level| {
                let mut events = bd_1.lock().unwrap();
                (*events).ball_dropped = true;
            })?;

        // Ball Drop Goal Event 2
        let bd_2: Arc<Mutex<GameState>> = Arc::clone(event_buffer);
        self.ball_drop_2
            .set_async_interrupt(Trigger::FallingEdge, move |_: Level| {
                let mut events = bd_2.lock().unwrap();
                (*events).ball_dropped = true;
            })?;

        Ok(())
    }
}
