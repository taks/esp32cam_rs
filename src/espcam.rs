use std::marker::PhantomData;

use camera::camera_status_t;
use esp_idf_svc::hal::{gpio::*, ledc::*, peripheral::Peripheral};
use esp_idf_svc::sys::*;

pub struct FrameBuffer<'a>(&'a mut camera::camera_fb_t);

impl<'a> FrameBuffer<'a> {
    pub fn data(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.0.buf, self.0.len) }
    }

    pub fn width(&self) -> usize {
        self.0.width
    }

    pub fn height(&self) -> usize {
        self.0.height
    }

    pub fn format(&self) -> camera::pixformat_t {
        self.0.format
    }

    pub fn timestamp(&self) -> camera::timeval {
        self.0.timestamp
    }

    fn fb_return(&mut self) {
        unsafe { camera::esp_camera_fb_return(self.0) }
    }
}

impl Drop for FrameBuffer<'_> {
    fn drop(&mut self) {
        self.fb_return();
    }
}

pub struct CameraSensor<'a>(&'a mut camera::sensor_t);

impl<'a> CameraSensor<'a> {
    pub fn status(&self) -> &camera_status_t {
        &self.0.status
    }
    pub fn init_status(&mut self) -> Result<(), EspError> {
        esp!(unsafe { self.0.init_status.unwrap()(self.0) })
    }
    pub fn reset(&mut self) -> Result<(), EspError> {
        esp!(unsafe { self.0.reset.unwrap()(self.0) })
    }
    pub fn set_pixformat(&mut self, format: camera::pixformat_t) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_pixformat.unwrap()(self.0, format) })
    }
    pub fn set_framesize(&mut self, framesize: camera::framesize_t) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_framesize.unwrap()(self.0, framesize) })
    }
    pub fn set_contrast(&mut self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_contrast.unwrap()(self.0, level) })
    }
    pub fn set_brightness(&mut self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_brightness.unwrap()(self.0, level) })
    }
    pub fn set_saturation(&mut self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_saturation.unwrap()(self.0, level) })
    }
    pub fn set_sharpness(&mut self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_sharpness.unwrap()(self.0, level) })
    }
    pub fn set_denoise(&mut self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_denoise.unwrap()(self.0, level) })
    }
    pub fn set_gainceiling(&mut self, gainceiling: camera::gainceiling_t) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_gainceiling.unwrap()(self.0, gainceiling) })
    }
    pub fn set_quality(&mut self, quality: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_quality.unwrap()(self.0, quality) })
    }
    pub fn set_colorbar(&mut self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_colorbar.unwrap()(self.0, if enable { 1 } else { 0 }) })
    }
    pub fn set_whitebal(&mut self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_whitebal.unwrap()(self.0, if enable { 1 } else { 0 }) })
    }
    pub fn set_gain_ctrl(&mut self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_gain_ctrl.unwrap()(self.0, if enable { 1 } else { 0 }) })
    }
    pub fn set_exposure_ctrl(&mut self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_exposure_ctrl.unwrap()(self.0, if enable { 1 } else { 0 }) })
    }
    pub fn set_hmirror(&mut self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_hmirror.unwrap()(self.0, if enable { 1 } else { 0 }) })
    }
    pub fn set_vflip(&mut self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_vflip.unwrap()(self.0, if enable { 1 } else { 0 }) })
    }
    pub fn set_aec2(&mut self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_aec2.unwrap()(self.0, if enable { 1 } else { 0 }) })
    }
    pub fn set_awb_gain(&mut self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_awb_gain.unwrap()(self.0, if enable { 1 } else { 0 }) })
    }
    pub fn set_agc_gain(&mut self, gain: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_agc_gain.unwrap()(self.0, gain) })
    }
    pub fn set_aec_value(&mut self, gain: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_aec_value.unwrap()(self.0, gain) })
    }
    pub fn set_special_effect(&mut self, effect: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_special_effect.unwrap()(self.0, effect) })
    }
    pub fn set_wb_mode(&mut self, mode: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_wb_mode.unwrap()(self.0, mode) })
    }
    pub fn set_ae_level(&mut self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_ae_level.unwrap()(self.0, level) })
    }
    pub fn set_dcw(&mut self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_dcw.unwrap()(self.0, if enable { 1 } else { 0 }) })
    }
    pub fn set_bpc(&mut self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_bpc.unwrap()(self.0, if enable { 1 } else { 0 }) })
    }
    pub fn set_wpc(&mut self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_wpc.unwrap()(self.0, if enable { 1 } else { 0 }) })
    }
    pub fn set_raw_gma(&mut self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_raw_gma.unwrap()(self.0, if enable { 1 } else { 0 }) })
    }
    pub fn set_lenc(&mut self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_lenc.unwrap()(self.0, if enable { 1 } else { 0 }) })
    }
    pub fn get_reg(&mut self, reg: i32, mask: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.get_reg.unwrap()(self.0, reg, mask) })
    }
    pub fn set_reg(&mut self, reg: i32, mask: i32, value: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_reg.unwrap()(self.0, reg, mask, value) })
    }
    pub fn set_res_raw(
        &mut self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        offset_x: i32,
        offset_y: i32,
        total_x: i32,
        total_y: i32,
        output_x: i32,
        output_y: i32,
        scale: bool,
        binning: bool,
    ) -> Result<(), EspError> {
        esp!(unsafe {
            self.0.set_res_raw.unwrap()(
                self.0, start_x, start_y, end_x, end_y, offset_x, offset_y, total_x, total_y,
                output_x, output_y, scale, binning,
            )
        })
    }
    pub fn set_pll(
        &mut self,
        bypass: i32,
        mul: i32,
        sys: i32,
        root: i32,
        pre: i32,
        seld5: i32,
        pclken: i32,
        pclk: i32,
    ) -> Result<(), EspError> {
        esp!(unsafe {
            self.0.set_pll.unwrap()(self.0, bypass, mul, sys, root, pre, seld5, pclken, pclk)
        })
    }
    pub fn set_xclk(&mut self, timer: i32, xclk: i32) -> Result<(), EspError> {
        esp!(unsafe { self.0.set_xclk.unwrap()(self.0, timer, xclk) })
    }
}

pub struct Camera<'a> {
    _p: PhantomData<&'a ()>,
}

impl<'a> Camera<'a> {
    #[allow(unused_variables)]
    pub fn new<C, T>(
        pin_pwdn: Option<impl Peripheral<P = impl InputPin + OutputPin> + 'a>,
        pin_xclk: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_d0: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_d1: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_d2: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_d3: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_d4: impl Peripheral<P = impl InputPin> + 'a,
        pin_d5: impl Peripheral<P = impl InputPin> + 'a,
        pin_d6: impl Peripheral<P = impl InputPin> + 'a,
        pin_d7: impl Peripheral<P = impl InputPin> + 'a,
        pin_vsync: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_href: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_pclk: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_sda: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        pin_scl: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        ledc_channel: impl Peripheral<P = C> + 'a,
        ledc_timer: impl Peripheral<P = T> + 'a,
        pixel_format: camera::pixformat_t,
        frame_size: camera::framesize_t,
    ) -> Result<Self, EspError>
    where
        C: LedcChannel<SpeedMode = <T as LedcTimer>::SpeedMode>,
        T: LedcTimer,
    {
        esp_idf_svc::hal::into_ref!(
            pin_xclk, pin_d0, pin_d1, pin_d2, pin_d3, pin_d4, pin_d5, pin_d6, pin_d7, pin_vsync,
            pin_href, pin_pclk, pin_sda, pin_scl
        );
        let config = camera::camera_config_t {
            pin_pwdn: pin_pwdn.map_or_else(|| -1, |p| p.into_ref().pin()),
            pin_xclk: pin_xclk.pin(),
            pin_reset: -1,

            pin_d0: pin_d0.pin(),
            pin_d1: pin_d1.pin(),
            pin_d2: pin_d2.pin(),
            pin_d3: pin_d3.pin(),
            pin_d4: pin_d4.pin(),
            pin_d5: pin_d5.pin(),
            pin_d6: pin_d6.pin(),
            pin_d7: pin_d7.pin(),
            pin_vsync: pin_vsync.pin(),
            pin_href: pin_href.pin(),
            pin_pclk: pin_pclk.pin(),

            xclk_freq_hz: 20000000,
            ledc_channel: C::channel(),
            ledc_timer: T::timer(),

            pixel_format,
            frame_size,

            jpeg_quality: 12,
            fb_count: 2,
            grab_mode: camera::camera_grab_mode_t_CAMERA_GRAB_WHEN_EMPTY,

            fb_location: camera::camera_fb_location_t_CAMERA_FB_IN_PSRAM,

            __bindgen_anon_1: camera::camera_config_t__bindgen_ty_1 {
                pin_sccb_sda: pin_sda.pin(),
            },
            __bindgen_anon_2: camera::camera_config_t__bindgen_ty_2 {
                pin_sccb_scl: pin_scl.pin(),
            },

            ..Default::default()
        };

        esp!(unsafe { camera::esp_camera_init(&config) })?;
        Ok(Self { _p: PhantomData })
    }

    pub fn get_framebuffer(&self) -> Option<FrameBuffer> {
        let fb = unsafe { camera::esp_camera_fb_get().as_mut() };
        fb.map(|fb| FrameBuffer(fb))
    }

    pub fn sensor(&self) -> CameraSensor<'a> {
        CameraSensor(unsafe { camera::esp_camera_sensor_get().as_mut().unwrap() })
    }
}

impl<'a> Drop for Camera<'a> {
    fn drop(&mut self) {
        esp!(unsafe { camera::esp_camera_deinit() }).expect("error during esp_camera_deinit")
    }
}
unsafe impl Send for Camera<'_> {}
unsafe impl Sync for Camera<'_> {}
