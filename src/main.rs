#![no_std]
#![no_main]

use esp32_hal::{clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, Delay};
use esp_backtrace as _;
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    // setup logger
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");

    // Set GPIO2 as an output, (the included LED)
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio2.into_push_pull_output();

    loop {
        led.toggle().unwrap();
        if led.is_set_high().unwrap() {
            log::info!("LED is on");
        } else {
            log::warn!("LED is off");
        }
        println!("Waiting 1 second");
        delay.delay_ms(1000u32);
    }
}
