#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::gpio::{Output, Level, OutputConfig};
esp_bootloader_esp_idf::esp_app_desc!();

use esp32_robot::{devices::Robot, states::State};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    esp_hal::system::software_reset()
}

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut robot = Robot::new(Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default()));
    let mut state = State::init();

    loop {
        state = state.handle_events(&mut robot);
    }
}
