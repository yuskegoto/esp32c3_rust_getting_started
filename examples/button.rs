use anyhow::*;
use log::*;

use esp_idf_sys::*;
use esp_idf_hal::prelude::*;

use esp_idf_hal::delay::{FreeRtos};
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;


fn main()-> Result<()> {
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    
    // Take Peripheral object
    let peripherals = Peripherals::take().unwrap();

    // set GPIO9 to button input
    let button = PinDriver::input(peripherals.pins.gpio9)?; // M5 Stamp C3U
    // let button = PinDriver::input(peripherals.pins.gpio3)?; // M5 Stamp C3
    
    info!("init done");

    loop {
        if button.is_low() {
            info!("high");
        }

        FreeRtos::delay_ms(100);
    }
}
