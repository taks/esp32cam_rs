use anyhow::Result;
use esp_idf_hal::io::Write;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{gpio::AnyIOPin, peripherals::Peripherals},
    http::{server::EspHttpServer, Method},
    io::Write,
    wifi::{AccessPointConfiguration, AuthMethod, BlockingWifi, Configuration, EspWifi},
};
use espcam::{config::get_config, espcam::Camera};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let sysloop = EspSystemEventLoop::take()?;

    let peripherals = Peripherals::take().unwrap();

    let config = get_config();

    let camera = Camera::new(
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
        esp_idf_svc::sys::camera::pixformat_t_PIXFORMAT_JPEG,
        esp_idf_svc::sys::camera::framesize_t_FRAMESIZE_UXGA,
    )
    .unwrap();

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    wifi.set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
        ssid: config.wifi_ssid.try_into().unwrap(),
        ssid_hidden: false,
        password: config.wifi_psk.try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        channel: 11,
        ..Default::default()
    }))?;

    wifi.start()?;
    wifi.wait_netif_up()?;

    let server_configuration = esp_idf_svc::http::server::Configuration {
        stack_size: 10240,
        ..Default::default()
    };
    let mut server = EspHttpServer::new(&server_configuration)?;

    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    server.fn_handler::<anyhow::Error, _>("/camera.jpg", Method::Get, move |request| {
        let framebuffer = camera.get_framebuffer();

        if let Some(framebuffer) = framebuffer {
            let data = framebuffer.data();

            let headers = [
                ("Content-Type", "image/jpeg"),
                ("Content-Length", &data.len().to_string()),
            ];
            let mut response = request.into_response(200, Some("OK"), &headers).unwrap();
            response.write_all(data)?;
        } else {
            let mut response = request.into_ok_response()?;
            response.write_all("no framebuffer".as_bytes())?;
        }

        Ok(())
    })?;

    server.fn_handler::<anyhow::Error, _>("/", Method::Get, |request| {
        let mut response = request.into_ok_response()?;
        response.write_all("ok".as_bytes())?;
        Ok(())
    })?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
