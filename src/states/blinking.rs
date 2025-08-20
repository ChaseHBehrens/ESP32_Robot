use esp_hal::time::Duration;

use crate::devices::Robot;
use crate::states::State;

use crate::devices::timer::TIMER_EXPIRED;

pub const BLINKING: State = State {
    handle_events: |robot| {
        if let Some(new_state) = handle_timer_expired(robot) {return new_state;};
        &BLINKING
    },
    on_enter: |robot| {
        robot.led.on();
        robot.timer.set(Duration::from_secs(1));
    },
    on_exit: |robot| {
        robot.led.off();
    },
};

fn handle_timer_expired(robot: &mut Robot) -> Option<&'static State> {
    if !TIMER_EXPIRED(&robot.timer) {return None};
    robot.led.toggle();
    robot.timer.set(Duration::from_secs(5));
    None
}