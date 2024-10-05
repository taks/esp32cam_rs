use std::sync::{Arc, Mutex};

use anyhow::Result;
use const_format::concatcp;
use esp_idf_svc::http::{server::EspHttpServer, Method};
use esp_idf_svc::io::Write;
use espcam::espcam::Camera;
use serde::{Deserialize, Serialize};
use esp_idf_svc::sys::camera::{framesize_t, camera_status_t};

#[derive(Serialize, Deserialize)]
#[serde(remote = "camera_status_t")]
pub struct CameraStatusT {
    pub framesize: framesize_t,
    pub scale: bool,
    pub binning: bool,
    pub quality: u8,
    pub brightness: i8,
    pub contrast: i8,
    pub saturation: i8,
    pub sharpness: i8,
    pub denoise: u8,
    pub special_effect: u8,
    pub wb_mode: u8,
    pub awb: u8,
    pub awb_gain: u8,
    pub aec: u8,
    pub aec2: u8,
    pub ae_level: i8,
    pub aec_value: u16,
    pub agc: u8,
    pub agc_gain: u8,
    pub gainceiling: u8,
    pub bpc: u8,
    pub wpc: u8,
    pub raw_gma: u8,
    pub lenc: u8,
    pub hmirror: u8,
    pub vflip: u8,
    pub dcw: u8,
    pub colorbar: u8,
}
#[derive(Serialize)]
struct Helper<'a>(#[serde(with = "CameraStatusT")] &'a camera_status_t);

#[derive(Serialize, Deserialize, Debug)]
struct Control {
    var: String,
    val: i32,
    // val: String,
}

#[allow(dead_code)]
const PART_BOUNDARY: &str = "123456789000000000000987654321";
const STREAM_CONTENT_TYPE: &str = concatcp!("multipart/x-mixed-replace;boundary=", PART_BOUNDARY);
const STREAM_BOUNDARY: &str = concatcp!("\r\n--", PART_BOUNDARY, "\r\n");

pub fn set_handlers(server: &mut EspHttpServer, camera: Arc<Mutex<Camera<'static>>>) -> Result<()> {
    server.fn_handler::<anyhow::Error, _>("/", Method::Get, |request| {
        let headers = [("Content-Type", "text/html"), ("Content-Encoding", "gzip")];
        let mut response = request.into_response(200, Some("OK"), &headers).unwrap();
        response.write_all(include_bytes!("index_ov2640.html.gz"))?;

        // let headers = [("Content-Type", "text/html")];
        // let mut response = request.into_response(200, Some("OK"), &headers).unwrap();
        // response.write_all(include_bytes!("index_ov2640.html"))?;

        Ok(())
    })?;

    let camera_ = camera.clone();
    server.fn_handler::<anyhow::Error, _>("/capture", Method::Get, move |request| {
        let camera = camera_.lock().unwrap();
        let framebuffer = camera.get_framebuffer();

        if let Some(framebuffer) = framebuffer {
            let data = framebuffer.data();

            let headers = [
                ("Content-Type", "image/jpeg"),
                ("Content-Length", &data.len().to_string()),
            ];
            let mut response = request.into_response(200, Some("OK"), &headers)?;
            response.write_all(data)?;
        } else {
            request.into_status_response(500)?;
        }

        Ok(())
    })?;

    let camera_ = camera.clone();
    server.fn_handler::<anyhow::Error, _>("/stream", Method::Get, move |request| {
        let mut response =
            request.into_response(200, Some("OK"), &[("Content-Type", STREAM_CONTENT_TYPE)])?;

        ::log::info!("/stream start");

        loop {
            let camera = camera_.lock().unwrap();
            let jpg = camera.get_framebuffer().unwrap().data();

            response.write_all(STREAM_BOUNDARY.as_bytes())?;

            write!(
                response,
                "Content-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                jpg.len()
            )?;
            response.write_all(jpg)?;
            response.flush()?;
        }
    })?;

    let camera_ = camera.clone();
    server.fn_handler::<anyhow::Error, _>("/control", Method::Post, move |mut request| {
        let camera = camera_.lock().unwrap();
        let mut sensor = camera.sensor();

        let mut buf = [0u8; 100];
        let read_len = request.read(&mut buf)?;
        let buf = &buf[..read_len];

        let c = serde_json::from_slice::<Control>(buf)?;

        let val = c.val;
        match &*c.var {
            "framesize" => sensor.set_framesize(val.into())?,
            "quality" => sensor.set_quality(val)?,
            "contrast" => sensor.set_contrast(val)?,
            "brightness" => sensor.set_brightness(val)?,
            "saturation" => sensor.set_saturation(val)?,
            _ => return Err(anyhow::Error::msg(c.var)),
        };

        request.into_response(200, Some("OK"), &[])?;

        Ok(())
    })?;

    let camera_ = camera.clone();
    server.fn_handler::<anyhow::Error, _>("/status", Method::Get, move |request| {
        let camera = camera_.lock().unwrap();
        let sensor = camera.sensor();
        let status = sensor.status();

        let headers = [];
        let mut response = request.into_response(200, Some("OK"), &headers)?;
        let json = serde_json::to_string(&Helper(status))?;
        response.write_all(json.as_bytes())?;

        Ok(())
    })?;

    Ok(())
}
