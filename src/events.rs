use crate::game::FoosEvent;
use crate::Config;
use rppal::gpio::{Gpio, InputPin, Level, Trigger};
use std::sync::mpsc::Sender;
use std::time::SystemTime;

pub struct FoosInputPins {
    blue_goal: InputPin,
    red_goal: InputPin,
    ball_drop_1: InputPin,
    ball_drop_2: InputPin,
    reset_pin: InputPin,
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

        let reset_pin = Gpio::new()?.get(config.reset_pin)?.into_input_pullup();

        Ok(FoosInputPins {
            blue_goal,
            red_goal,
            ball_drop_1,
            ball_drop_2,
            reset_pin,
        })
    }

    pub fn init_interrupts(
        &mut self,
        event_buffer: &Sender<FoosEvent>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Blue Goal Event
        let bg: Sender<FoosEvent> = event_buffer.clone();
        let mut last_interrupt_time_bg: u128 = 0;
        self.blue_goal
            .set_async_interrupt(Trigger::FallingEdge, move |_: Level| {
                let interrupt_time = get_now();
                if get_now() - last_interrupt_time_bg > 200 {
                    bg.send(FoosEvent::BlueGoal).unwrap();
                }
                last_interrupt_time_bg = interrupt_time;
            })?;

        // Red Goal Event
        let rg: Sender<FoosEvent> = event_buffer.clone();
        let mut last_interrupt_time_rg: u128 = 0;
        self.red_goal
            .set_async_interrupt(Trigger::FallingEdge, move |_: Level| {
                let interrupt_time = get_now();
                if get_now() - last_interrupt_time_rg > 200 {
                    rg.send(FoosEvent::RedGoal).unwrap();
                }
                last_interrupt_time_rg = interrupt_time;
            })?;

        // Ball Drop Goal Event 1
        let bd_1: Sender<FoosEvent> = event_buffer.clone();
        let mut last_interrupt_time_bd_1: u128 = 0;
        self.ball_drop_1
            .set_async_interrupt(Trigger::FallingEdge, move |_: Level| {
                let interrupt_time = get_now();
                if get_now() - last_interrupt_time_bd_1 > 200 {
                    bd_1.send(FoosEvent::BallDrop).unwrap();
                }
                last_interrupt_time_bd_1 = interrupt_time;
            })?;

        // Ball Drop Goal Event 2
        let bd_2: Sender<FoosEvent> = event_buffer.clone();
        let mut last_interrupt_time_bd_2: u128 = 0;
        self.ball_drop_2
            .set_async_interrupt(Trigger::FallingEdge, move |_: Level| {
                let interrupt_time = get_now();
                if get_now() - last_interrupt_time_bd_2 > 200 {
                    bd_2.send(FoosEvent::BallDrop).unwrap();
                }
                last_interrupt_time_bd_2 = interrupt_time;
            })?;

        // Reset Event
        let reset: Sender<FoosEvent> = event_buffer.clone();
        let mut last_interrupt_time_reset: u128 = 0;
        self.reset_pin
            .set_async_interrupt(Trigger::FallingEdge, move |_: Level| {
                let interrupt_time = get_now();
                if get_now() - last_interrupt_time_reset > 200 {
                    reset.send(FoosEvent::Reset).unwrap();
                }
                last_interrupt_time_reset = interrupt_time;
            })?;

        Ok(())
    }
}

fn get_now() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
