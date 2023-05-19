use log::*;

use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};

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