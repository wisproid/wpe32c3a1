//! embassy hello world
//!
//! This is an example of running the embassy executor with multiple tasks
//! concurrently.

//% CHIPS: esp32 esp32c2 esp32c3 esp32c6 esp32h2 esp32s2 esp32s3
//% FEATURES: embassy esp-hal-embassy/integrated-timers

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    system::SystemControl,
    timer::timg::TimerGroup,
    prelude::*
};

#[embassy_executor::task]
async fn run() {
    loop {
        esp_println::println!("Hello world from embassy using esp-hal-async!");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    esp_println::println!("Init!");
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    esp_hal_embassy::init(&clocks, timg0.timer0);

    // Enable the RWDT watchdog timer:
    let timg1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt0 = timg1.wdt;
    wdt0.enable();
    wdt0.set_timeout(10u64.secs());
    log::info!("Watchdog enabled!");

    spawner.spawn(run()).ok();
    let mut count = 1000;
    loop {
        esp_println::println!("Bing!");
        wdt0.feed();
        Timer::after(Duration::from_millis(count)).await;
        count = count + 1000;
    }
}