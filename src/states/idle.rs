use crate::devices::Robot;
use crate::states::State;
use crate::devices::timer::TIMER_EXPIRED;

pub const IDLE: State = State {
    handle_events: |robot| {
        if let Some(new_state) = handle_timer_expired(robot) {return new_state;};
        &IDLE
    },
    on_enter: |_robot| {},
    on_exit: |_robot| {},
};

fn handle_timer_expired(robot: &mut Robot) -> Option<&'static State> {
    if !TIMER_EXPIRED(&robot.timer) {return None};
    Some(IDLE.set(&IDLE, robot))
}