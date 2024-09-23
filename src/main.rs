use anyhow::Result;

use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::peripherals::Peripherals;
use espcam::espcam::Camera;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

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

    loop {
        let framebuffer = camera.get_framebuffer();

        if let Some(framebuffer) = framebuffer {
            println!("Got framebuffer!");
            println!("width: {}", framebuffer.width());
            println!("height: {}", framebuffer.height());
            println!("len: {}", framebuffer.data().len());
            println!("format: {}", framebuffer.format());

            std::thread::sleep(std::time::Duration::from_millis(1000));
        } else {
            log::info!("no framebuffer");
        }
    }
}
