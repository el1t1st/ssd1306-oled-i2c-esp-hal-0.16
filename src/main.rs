#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(unused_imports)]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*,
};

use embedded_graphics::{
    mono_font::{iso_8859_10::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

use fugit::ExtU32;
use fugit::RateExtU32;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    // let system = SystemControl::new(peripherals.SYSTEM);
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    esp_println::logger::init_logger_from_env();

    let sda = io.pins.gpio8;
    let sdc = io.pins.gpio9;
    let frequency = RateExtU32::kHz(100);

    let i2c = I2C::new(peripherals.I2C1, sda, sdc, frequency, &clocks);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Privet Mir!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {
        log::info!("Privet Mir!");
        delay.delay_ms(1000_u32);
    }
}
