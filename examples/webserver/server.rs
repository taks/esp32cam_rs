use std::sync::{Arc, Mutex};

use anyhow::Result;
use esp_idf_svc::http::{server::EspHttpServer, Method};
use esp_idf_svc::io::Write;
use espcam::espcam::Camera;

pub fn set_handlers(server: &mut EspHttpServer, camera: Arc<Mutex<Camera<'static>>>) -> Result<()> {
    server.fn_handler::<anyhow::Error, _>("/", Method::Get, |request| {
        // let headers = [("Content-Type", "text/html"), ("Content-Encoding", "gzip")];
        // let mut response = request.into_response(200, Some("OK"), &headers).unwrap();
        // response.write_all(include_bytes!("index_ov2640.html.gz"))?;

        let headers = [("Content-Type", "text/html")];
        let mut response = request.into_response(200, Some("OK"), &headers).unwrap();
        response.write_all(include_bytes!("index_ov2640.html"))?;

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
    server.fn_handler::<anyhow::Error, _>("/control", Method::Get, move |request| {
        let camera = camera_.lock().unwrap();
        let mut response = request.into_response(200, Some("OK"), &[])?;


        Ok(())
    })?;

    Ok(())
}

fn parse_get(req :httpd_req_t *req)