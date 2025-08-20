use esp_hal::gpio::Output;

pub struct LED {
    pin: Output<'static>,
}

impl LED {
    pub fn new(pin: Output<'static>) -> Self {
        Self {pin}
    }

    pub fn on(&mut self) {
        self.pin.set_high();
    }

    pub fn off(&mut self) {
        self.pin.set_low();
    }

    pub fn toggle(&mut self) {
        self.pin.toggle();
    }
}