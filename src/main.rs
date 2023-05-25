use anyhow::{bail, Result};
use log::*;

use std::env;

use esp_idf_sys::*;
use esp_idf_hal::prelude::*;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};

// use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

use ws2812_esp32_rmt_driver::{RGB8};

use thingbuf::mpsc::StaticChannel;

mod wifi;
mod led;
mod osc;

use led::Led;
use osc::Osc;

// use rosc::encoder;
// use rosc::{OscMessage, OscPacket, OscType};
// use std::net::{SocketAddrV4, UdpSocket, Ipv4Addr};


const OSC_WIFI_SSID: &str = env!("OSC_WIFI_SSID");
const OSC_WIFI_PASS: &str = env!("OSC_WIFI_PASS");
const OSC_WIFI_RECV_PORT_STR: &str = env!("OSC_WIFI_RECV_PORT_STR");
const OSC_WIFI_PONG_PORT_STR: &str = env!("OSC_WIFI_RECV_PONG_STR");

static LED_CHANNEL: StaticChannel<RGB8, 16> = StaticChannel::new();

fn main()-> Result<()> {
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    unsafe{
        esp_idf_sys::nvs_flash_init();
    }
    
    let peripherals = Peripherals::take().unwrap();

    // let button = PinDriver::input(peripherals.pins.gpio9)?; // M5 Stamp C3U
    // let button = PinDriver::input(peripherals.pins.gpio3)?; // M5 Stamp C3
    
    let nvs = EspDefaultNvsPartition::take()?;
    
    let sysloop = EspSystemEventLoop::take().unwrap();
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;

    wifi::connect_wifi(&mut wifi, OSC_WIFI_SSID, OSC_WIFI_PASS)?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    let ip = ip_info.ip;

    info!("Wifi DHCP info: {:?}", ip_info);

    // Create StaticChannel to send RGB data from OSC thread to LED thread
    let (sender, receiver) = LED_CHANNEL.split();

    // Create thread to handle LEDs
    let led_join_handle = std::thread::Builder::new()
        .stack_size(4096)
        .spawn(move || {
            let mut led = Led::new(receiver);
            loop {
                if let Err(e) = led.run() {
                    error!("Failed to run LEDs: {e}");
                    break;
                }
                led.idle();
            }
        })?;
    // Create thread to receive/send OSC
    // Larger stack size is required (default is 3 KB)
    // You can customize default value by CONFIG_ESP_SYSTEM_EVENT_TASK_STACK_SIZE in sdkconfig
    let recv_port = OSC_WIFI_RECV_PORT_STR.parse::<u16>().unwrap();
    let pong_port = OSC_WIFI_PONG_PORT_STR.parse::<u16>().unwrap();
    let osc_join_handle = std::thread::Builder::new()
        .stack_size(8192)
        .spawn(move || {
            let mut osc = Osc::new(ip, recv_port, pong_port, sender);
            loop {
                if let Err(e) = osc.run() {
                    error!("Failed to run OSC: {e}");
                    break;
                }
            }
        })?;

    led_join_handle.join().unwrap();
    osc_join_handle.join().unwrap();

    info!("Finish app");
    Ok(())
}

