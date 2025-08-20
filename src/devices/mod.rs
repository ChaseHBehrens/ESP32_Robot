use esp_hal::gpio::Output;
pub mod timer; pub use timer::Timer;
pub mod led; pub use led::LED;


pub struct Robot {
    pub timer: Timer,
    pub led: LED,
}

impl Robot {
    pub fn new(led_pin: Output<'static>) -> Self {
        Robot {
            timer: Timer::new(),
            led: LED::new(led_pin),
        }
    }
}