use re_rav1d::Decoder;
use std::{fs::File, io::Read as _};
use svt_av1_enc::*;

fn get_y4m_input() -> Vec<u8> {
    let mut input = File::open(format!(
        "{}/tests/small_input.y4m",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();
    let mut data = Vec::new();
    input.read_to_end(&mut data).unwrap();
    data
}

#[test]
fn encode_decode_from_y4m() {
    let data = get_y4m_input();
    let mut dec = y4m::decode(&*data).unwrap();
    let width = dec.get_width();
    let height = dec.get_height();
    let framerate = dec.get_framerate();

    let mut svt_config = SvtAv1EncoderConfig::new(width as u32, height as u32, Some(1));
    svt_config.config.frame_rate_numerator = framerate.num as u32;
    svt_config.config.frame_rate_denominator = framerate.den as u32;
    svt_config.config.force_key_frames = 1;

    let svt_encoder = svt_config.into_encoder().expect("Encoder");
    let mut encoded_frames = 0;

    while let Ok(frame) = dec.read_frame() {
        let svt_frame = Frame::new(
            frame.get_y_plane(),
            frame.get_u_plane(),
            frame.get_v_plane(),
            width as u32,
            (width / 2) as u32,
            (width / 2) as u32,
            (frame.get_y_plane().len() + frame.get_u_plane().len() + frame.get_v_plane().len())
                as u32,
        );

        svt_encoder
            .send_picture(svt_frame, None, true)
            .expect("Send frame");
        encoded_frames += 1;
    }
    svt_encoder.send_eos().expect("Send eos");
    let mut decoder_settings = re_rav1d::Settings::new();
    decoder_settings.set_n_threads(20);
    decoder_settings.set_decode_frame_type(re_rav1d::DecodeFrameType::All);
    let mut decoder = Decoder::with_settings(&decoder_settings).unwrap();
    let mut decoded_frames = 0;

    while let Ok(packet) = svt_encoder.get_packet(1) {
        _ = decoder
            .send_data(packet.to_vec(), None, None, None)
            .inspect_err(|err| match err {
                re_rav1d::Error::Again => {}
                err => panic!("Failed to send data to decoder: {err}"),
            });

        let res = decoder.send_pending_data();
        match res {
            Ok(_) | Err(re_rav1d::Error::Again) => {
                if decoder.get_picture().is_ok() {
                    decoded_frames += 1;
                }
            }
            _ => {}
        }
    }
    while decoder.get_picture().is_ok() {
        decoded_frames += 1;
    }

    assert_eq!(encoded_frames, decoded_frames);
}
