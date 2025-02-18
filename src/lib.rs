//! Tiny safe abstraction for [SVT-AV1 encoder](https://gitlab.com/AOMediaCodec/SVT-AV1)
//!
//! ## Warning ⚠️
//! This crate must be used with SVT-AV1 2.3.0 version. SVT-AV1 library API is subject to change with new releases, so if you want to use it with different version, do it at your own risk!
//!
//! ## Building
//! For building options and requirements please follow [README]()
//!
//! ## Example
//! ```
//! use svt_av1_enc::*;
//!
//! // First create config
//! let mut cfg = SvtAv1EncoderConfig::new(1920, 1080, Some(5));
//!
//! // Set any desired properties
//! cfg.config.qp == 30;
//!
//! // Create encoder from config
//! let encoder = cfg.into_encoder();
//!
//! // use encoder.send_picture() to send raw YUV frames to encoder
//! // use encoder.get_packet() to get done packet
//! ```
//! More comprehensive example can be found [here]()

pub mod ffi;
use ffi::*;
use std::{
    ffi::CString,
    mem::MaybeUninit,
    ops::Deref,
    ptr::{self},
};

macro_rules! call_c_code {
    ($fn:expr) => {
        let res = unsafe { $fn };
        if res != ErrorType::ErrorNone {
            return Err(res);
        }
    };
}

type Result<T> = std::result::Result<T, ErrorType>;

/// Original name is `SvtIOFormat`.
/// [`Frame`] is the main format to send data (picture) to encoder. It borrows YUV data
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Frame<'a> {
    luma: &'a [u8],
    cb: &'a [u8],
    cr: &'a [u8],
    y_stride: u32,
    cr_stride: u32,
    cb_stride: u32,
    size: u32,
}

impl<'a> Frame<'a> {
    /// Create new raw frame with provided data. It borrows YUV data
    ///
    // TODO: probably more checks for provided data
    pub fn new(
        luma: &'a [u8],
        cb: &'a [u8],
        cr: &'a [u8],
        y_stride: u32,
        cb_stride: u32,
        cr_stride: u32,
        size: u32,
    ) -> Self {
        Self {
            luma,
            cb,
            cr,
            y_stride,
            cr_stride,
            cb_stride,
            size,
        }
    }
}

/// Encoded AV1 data (packet) in borrowed form. For owned form, use [`to_vec`]
///
/// [`to_vec`]: method@Self::to_vec
#[derive(Debug)]
pub struct Packet<'a> {
    pub flags: SvtFlags,
    data: &'a [u8],
    p_buffer: *mut BufferHeaderType,
}

unsafe impl Send for Packet<'_> {}
unsafe impl Sync for Packet<'_> {}

impl Packet<'_> {
    /// Clone packed AV1 data and return it as bytes
    pub fn to_vec(self) -> Vec<u8> {
        self.data.to_owned()
    }
}

impl Deref for Packet<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl Drop for Packet<'_> {
    fn drop(&mut self) {
        // SAFETY: we require Packet's lifetime to live not longer
        // than encoder itself so it's safe to call
        unsafe { svt_av1_enc_release_out_buffer(&mut self.p_buffer) };
    }
}

/// SVT-AV1 Encoder. Use [`send_picture`] to send some data to encoder and [`get_packet`] to get encoded packets
///
/// [`send_picture`]: method@Self::send_picture
/// [`get_packet`]: method@Self::get_packet
#[derive(Debug)]
pub struct SvtAv1Encoder {
    handle: *mut ComponentType,
}

unsafe impl Send for SvtAv1Encoder {}

impl SvtAv1Encoder {
    /// Send data to Encoder.
    /// * This function takes [`Frame`], which borrows provided YUV data
    pub fn send_picture(
        &self,
        frame: Frame<'_>,
        pts: Option<i64>,
        force_keyframe: bool,
    ) -> Result<()> {
        let mut svt_frame = SvtIOFormat {
            luma: frame.luma as *const [u8] as *const _,
            cb: frame.cb as *const [u8] as *const _,
            cr: frame.cr as *const [u8] as *const _,
            y_stride: frame.y_stride,
            cr_stride: frame.cr_stride,
            cb_stride: frame.cb_stride,
            ..Default::default()
        };
        let mut buf = BufferHeaderType {
            pic_type: Av1PictureType::Av1InvalidPicture,
            n_filled_len: frame.size,
            p_buffer: &mut svt_frame as *mut SvtIOFormat as *mut u8,
            ..Default::default()
        };
        if let Some(pts) = pts {
            buf.pts = pts
        }

        if force_keyframe {
            buf.pic_type = Av1PictureType::Av1KeyPicture
        }

        call_c_code!(svt_av1_enc_send_picture(self.handle, &mut buf));
        Ok(())
    }

    /// Send end of stream signal to encoder. Use it when you run out of frames to encode
    pub fn send_eos(&self) -> Result<()> {
        let mut buf = BufferHeaderType {
            flags: SvtFlags::BUFFERFLAG_EOS.bits(),
            ..Default::default()
        };
        call_c_code!(svt_av1_enc_send_picture(self.handle, &mut buf));

        Ok(())
    }

    ///  Receive packet.
    /// * This function will become blocking if either pic_send_done is set to 1 or if we are in low-delay (pred-struct=1).
    /// * Otherwise, this function is non-blocking and will return NoErrorEmptyQueue if there are no packets available.
    pub fn get_packet(&self, pic_send_done: u8) -> Result<Packet<'_>> {
        let mut pict: MaybeUninit<*mut BufferHeaderType> = MaybeUninit::uninit();
        call_c_code!(svt_av1_enc_get_packet(
            self.handle,
            pict.as_mut_ptr(),
            pic_send_done
        ));
        unsafe {
            // SAFETY: we checked that svt_av1_enc_get_packet doesn't return error, so it's initialized
            let header_type = pict.assume_init();
            let buffer = &*std::ptr::slice_from_raw_parts_mut(
                (*header_type).p_buffer,
                (*header_type).n_filled_len as usize,
            );
            let packet = Packet {
                data: buffer,
                p_buffer: header_type,
                flags: SvtFlags::from_bits((*header_type).flags).unwrap(),
            };
            Ok(packet)
        }
    }
}

impl Drop for SvtAv1Encoder {
    fn drop(&mut self) {
        unsafe {
            svt_av1_enc_deinit(self.handle);
            svt_av1_enc_deinit_handle(self.handle);
        }
    }
}

/// Config for SvtAv1Enc encoder. Construct it with [`new`] function,
/// than create encoder with [`into_encoder`]
///
/// [`new`]: method@Self::new
/// [`into_encoder`]: method@Self::into_encoder
#[derive(Debug)]
pub struct SvtAv1EncoderConfig {
    pub config: SvtAv1EncConfiguration,
    handle: *mut ComponentType,
}

impl SvtAv1EncoderConfig {
    /// Create new config with default settings and set width and height parameters. Lift of settings can be found [here](https://gitlab.com/AOMediaCodec/SVT-AV1/-/blob/master/Docs/Parameters.md#encoder-global-options)
    pub fn new(width: u32, height: u32, preset: Option<i8>) -> Self {
        assert!((64..=16384).contains(&width));
        assert!((64..=8704).contains(&height));
        let mut handle: MaybeUninit<*mut ComponentType> = MaybeUninit::uninit();
        let mut config: MaybeUninit<SvtAv1EncConfiguration> = MaybeUninit::uninit();

        let mut cfg = unsafe {
            let res =
                svt_av1_enc_init_handle(handle.as_mut_ptr(), ptr::null_mut(), config.as_mut_ptr());
            assert!(res == ErrorType::ErrorNone, "Failed to init svt av1 handle");

            // SAFETY: we checked that result is ErrorNone
            SvtAv1EncoderConfig {
                handle: handle.assume_init(),
                config: config.assume_init(),
            }
        };
        cfg.config.source_width = width;
        cfg.config.source_height = height;
        if let Some(preset) = preset {
            assert!(
                (-2..=13).contains(&preset),
                "Preset must be in between -2 and 13"
            );
            cfg.config.enc_mode = preset;
        }

        cfg
    }

    /// Set parameter for config from string. Note that string parameter name differs from [`SvtAv1EncConfiguration`] fields
    /// Incorrect parameter name or value will cause error. You can find all legitimate parameters names [here](https://gitlab.com/AOMediaCodec/SVT-AV1/-/blob/master/Source/Lib/Globals/enc_settings.c#L1832)
    pub fn set_parameter_from_str(
        &mut self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Result<()> {
        let name = CString::new(name.as_ref()).unwrap();
        let value = CString::new(value.as_ref()).unwrap();
        call_c_code!(svt_av1_enc_parse_parameter(
            &mut self.config,
            name.as_ptr(),
            value.as_ptr()
        ));
        Ok(())
    }

    /// Apply configuration and initialize encoder. It can return error in case of invalid configuration
    pub fn into_encoder(mut self) -> Result<SvtAv1Encoder> {
        call_c_code!(svt_av1_enc_set_parameter(self.handle, &mut self.config));
        call_c_code!(svt_av1_enc_init(self.handle));

        Ok(SvtAv1Encoder {
            handle: self.handle,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let enc = SvtAv1EncoderConfig::new(1920, 1080, None);
        assert!(enc.config.enc_mode == 10);
        assert!(enc.config.source_width == 1920);
        assert!(enc.config.source_height == 1080);

        let cfg = SvtAv1EncoderConfig::new(800, 600, Some(3));
        assert!(cfg.config.enc_mode == 3);
        assert!(cfg.config.source_width == 800);
        assert!(cfg.config.source_height == 600);

        cfg.into_encoder().unwrap();
    }

    #[test]
    fn set_parameter() {
        let mut cfg = SvtAv1EncoderConfig::new(1920, 1080, None);
        cfg.set_parameter_from_str("qp", "30").unwrap();
        assert!(cfg.config.qp == 30);

        cfg.into_encoder().unwrap();
    }
}
