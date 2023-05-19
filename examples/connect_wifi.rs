use anyhow::*;
use log::*;

use esp_idf_sys::*;
use esp_idf_hal::prelude::*;

use esp_idf_hal::delay::{FreeRtos};
use esp_idf_hal::peripherals::Peripherals;

use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};

const OSC_WIFI_SSID: &str = env!("OSC_WIFI_SSID");
const OSC_WIFI_PASS: &str = env!("OSC_WIFI_PASS");

fn main()-> Result<()> {
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    unsafe{
        esp_idf_sys::nvs_flash_init();
    }
    
    let peripherals = Peripherals::take().unwrap();
    
    let nvs = EspDefaultNvsPartition::take()?;
    
    let sysloop = EspSystemEventLoop::take().unwrap();
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;

    connect_wifi(&mut wifi, OSC_WIFI_SSID, OSC_WIFI_PASS)?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}", ip_info);

    loop {
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