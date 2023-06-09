# What is this?
- Getting start project for ESP32-C3 Rust hosted at TDA.
- This project is mainly referenced from Tai Hideaki san's [rust-esp32-osc-led](https://github.com/hideakitai/rust-esp32-osc-led.git) project.
- For the instruction plase see [TDA's workshop notion page](https://www.notion.so/techdirector/Rust-a501b600bbc349a09112fe94d3f6291b)

## Projects
- examples/button.rs
    - Button input example
- examples/button_led.rs
    - Serial LED example
- examples/connect_wifi.rs
    - Wifi connect example
- examples/osc_send.rs
    - OSC send example
- examples/main.rs
    - OSC receive and led control

Before running wifi related projects, export wifi credentials and port no.
 
```bash
export OSC_WIFI_SSID=LAB3
export OSC_WIFI_PASS=aabbccddeeff
export OSC_WIFI_RECV_PORT_STR=5000
export OSC_WIFI_RECV_PONG_STR=5001

# osc send port
export OSC_WIFI_SEND_PORT_STR=5100
# osc destination (PC) port
export OSC_WIFI_DEST_PORT_STR=5101
export OSC_WIFI_DEST_IP=192.168.2.100
```

## Commands
```bash
source ~/export-esp.sh  # if needed
cargo run               # build & flash & monitor
cargo build --example button
cargo run --example connect_wifi
espmonitor /dev/ttyACM0
```

## Crate
- [rosc](https://crates.io/crates/rosc) is used to encode/decode OSC packet
- [smart-leds](https://crates.io/crates/smart-leds) trait and its implementation [ws2812-esp32-rmt-driver](https://crates.io/crates/ws2812-esp32-rmt-driver) are used to control LEDs

## Samples
- [Wifi connect sequence for esp_idf_svc 0.46](https://github.com/esp-rs/esp-idf-svc/blob/master/examples/wifi.rs)
- Tai Hideaki san's [rust-esp32-osc-led](https://github.com/hideakitai/rust-esp32-osc-led.git)


# OSC test with OSCD
- Commandline OSC tester. You can also use other tools like TouchDesigner.
- With another terminal, run [`oscd`](https://crates.io/crates/oscd) to send OSC packet (IP should be your device's IP and PORT should be `OSC_WIFI_RECV_PORT`)

```bash
cargo install oscd
oscd
```

## Following OSC commands are available

```bash
/ping       # reply /pong 1 to your_ip:OSC_WIFI_PONG_PORT from your device
/rgb r g b  # set color to LED (int 0-255 are available for r, g, b)
```
