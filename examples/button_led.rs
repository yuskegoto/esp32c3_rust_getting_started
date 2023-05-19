use anyhow::*;
use log::*;

use esp_idf_sys::*;
use esp_idf_hal::prelude::*;

use esp_idf_hal::delay::{FreeRtos};
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

// use smart_leds::RGB8;
use smart_leds::hsv::{hsv2rgb, Hsv, RGB};
use smart_leds::SmartLedsWrite;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, RGB8};
use ws2812_esp32_rmt_driver::driver::color::{LedPixelColorGrbw32};

fn main()-> Result<()> {
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    
    let peripherals = Peripherals::take().unwrap();
    let button = PinDriver::input(peripherals.pins.gpio9)?; // M5 Stamp C3U
    // let button = PinDriver::input(peripherals.pins.gpio3)?; // M5 Stamp C3
    
    // Init serial LED pin
    const LED_PIN: u32 = 2;
    let mut ws2812 = LedPixelEsp32Rmt::
    <RGB8, LedPixelColorGrbw32>::new(0, LED_PIN).unwrap();

    info!("init done");

    loop {
        if button.is_low() {
            info!("high");
            let pixels = std::iter::repeat( hsv2rgb(Hsv {
                hue: 0, 
                sat: 255,
                val: 128,
            }))
            .take(1);
            ws2812.write(pixels).unwrap();
        }
        else{
            let pixels = std::iter::repeat(RGB { r: 0, g: 0, b: 0})
            .take(1);
            ws2812.write(pixels).unwrap();
        }

        FreeRtos::delay_ms(100);
    }
}

