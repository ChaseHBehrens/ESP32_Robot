use crate::devices::Robot;
mod idle; use idle::IDLE;
mod blinking; use blinking::BLINKING;

pub type Event<T> = fn(&T) -> bool;

pub struct State {
    pub(crate) handle_events: fn(&mut Robot) -> &'static State,
    pub(crate) on_enter: fn(&mut Robot),
    pub(crate) on_exit: fn(&mut Robot),
}

impl State {
    pub fn init() -> &'static State {&BLINKING}

    fn set(&self, state: &'static State, robot: &mut Robot) -> &'static State {
        (self.on_exit)(robot);
        (state.on_enter)(robot);
        state
    }

    pub fn handle_events(&self, robot: &mut Robot) -> &'static State {
        (self.handle_events)(robot)
    }
}