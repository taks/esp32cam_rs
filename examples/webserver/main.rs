mod server;

use std::sync::{Arc, Mutex};

use anyhow::Result;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{gpio::AnyIOPin, peripherals::Peripherals},
    http::server::EspHttpServer,
    nvs::EspDefaultNvsPartition,
    wifi::{
        AccessPointConfiguration, AuthMethod, BlockingWifi, ClientConfiguration, Configuration,
        EspWifi,
    },
};
use espcam::{config::get_config, espcam::Camera};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let config = get_config();

    let camera: Camera<'static> = Camera::new(
        None::<AnyIOPin>,        // pin_pwdn
        peripherals.pins.gpio15, // pin_xclk
        peripherals.pins.gpio11, // pin_d0
        peripherals.pins.gpio9,  // pin_d1
        peripherals.pins.gpio8,  // pin_d2
        peripherals.pins.gpio10, // pin_d3
        peripherals.pins.gpio12, // pin_d4
        peripherals.pins.gpio18, // pin_d5
        peripherals.pins.gpio17, // pin_d6
        peripherals.pins.gpio16, // pin_d7
        peripherals.pins.gpio6,  // pin_vsync
        peripherals.pins.gpio7,  // pin_href
        peripherals.pins.gpio13, // pin_pclk
        peripherals.pins.gpio4,  // pin_sda
        peripherals.pins.gpio5,  // pin_scl
        peripherals.ledc.channel0,
        peripherals.ledc.timer0,
        esp_idf_svc::sys::camera::pixformat_t_PIXFORMAT_JPEG,
        esp_idf_svc::sys::camera::framesize_t_FRAMESIZE_UXGA,
    )
    .unwrap();

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    if config.accss_point {
        wifi.set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
            ssid: config.wifi_ssid.try_into().unwrap(),
            ssid_hidden: false,
            password: config.wifi_psk.try_into().unwrap(),
            auth_method: AuthMethod::WPA2Personal,
            channel: 11,
            ..Default::default()
        }))?;
        wifi.start()?;
    } else {
        // wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;
        // wifi.start()?;
        // let ap_infos = wifi.scan()?;
        // let ours = ap_infos.into_iter().find(|a| a.ssid == config.wifi_ssid);
        wifi.set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: config.wifi_ssid.try_into().unwrap(),
            password: config.wifi_psk.try_into().unwrap(),
            // channel: ours.map(|x| x.channel),
            channel: None,
            auth_method: AuthMethod::WPA2Personal,
            ..Default::default()
        }))?;
        wifi.start()?;
        wifi.connect()?;
    }

    wifi.wait_netif_up()?;

    let server_configuration = esp_idf_svc::http::server::Configuration {
        stack_size: 10240,
        ..Default::default()
    };
    let mut server = EspHttpServer::new(&server_configuration)?;
    server::set_handlers(&mut server, Arc::new(Mutex::new(camera)))?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
