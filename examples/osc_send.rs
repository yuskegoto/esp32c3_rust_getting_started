use anyhow::*;
use log::*;

use esp_idf_sys::*;
use esp_idf_hal::prelude::*;

use esp_idf_hal::delay::{FreeRtos};
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::net::{SocketAddrV4, UdpSocket, Ipv4Addr};
use std::str::FromStr;

const OSC_WIFI_SSID: &str = env!("OSC_WIFI_SSID");
const OSC_WIFI_PASS: &str = env!("OSC_WIFI_PASS");

// const SEND_PORT: u16 = 5100;
const SEND_PORT_STR: &str = env!("OSC_WIFI_SEND_PORT_STR");

// const DEST_PORT: u16 = 5101;
const DEST_PORT_STR: &str = env!("OSC_WIFI_DEST_PORT_STR");
// const OSC_DEST_IP: &str = "192.168.2.100";
const OSC_DEST_IP: &str = env!("OSC_WIFI_DEST_IP");

fn main()-> Result<()> {
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    unsafe{
        esp_idf_sys::nvs_flash_init();
    }
    
    let peripherals = Peripherals::take().unwrap();

    // set GPIO9 to button input
    let button = PinDriver::input(peripherals.pins.gpio9)?; // M5 Stamp C3U
    // let button = PinDriver::input(peripherals.pins.gpio3)?; // M5 Stamp C3


    let nvs = EspDefaultNvsPartition::take()?;
    
    let sysloop = EspSystemEventLoop::take().unwrap();
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;

    connect_wifi(&mut wifi, OSC_WIFI_SSID, OSC_WIFI_PASS)?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}", ip_info);

    let dest_ip = Ipv4Addr::from_str(OSC_DEST_IP)?;
    // let dest_ip = Ipv4Addr::new(192, 168, 2, 100);

    let send_port = SEND_PORT_STR.parse::<u16>().unwrap();
    let dest_port = DEST_PORT_STR.parse::<u16>().unwrap();

    let host_addr = SocketAddrV4::new(ip_info.ip, send_port);
    let to_addr =  SocketAddrV4::new(dest_ip, dest_port);
    let sock = UdpSocket::bind(host_addr).unwrap();

    // switch view
    let mut msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/3".to_string(),
        args: vec![],
    }))
    .unwrap();

    // sock.send_to(&msg_buf, to_addr).unwrap();

    let mut counter = 0i32;
    let mut counter_f = 0f32;

    loop {
        if button.is_low() {

            msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                addr: "/3/xy1".to_string(),
                args: vec![OscType::Int(counter), OscType::Float(counter_f)],
            }))
            .unwrap();
            sock.send_to(&msg_buf, to_addr).unwrap();

            counter += 1;

            info!("msg{}, {}", counter, counter_f);
        }
        counter_f += 0.1f32;

        FreeRtos::delay_ms(100);
     }
}

pub fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>,
 ssid_osc: &str, pass_osc: &str,) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: ssid_osc.into(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: pass_osc.into(),
        channel: None,
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    info!("Wifi started");

    wifi.connect()?;
    info!("Wifi connected");

    wifi.wait_netif_up()?;
    info!("Wifi netif up");

    Ok(())
}