//! Bindings for SVT-AV1-Enc v2.3.0.
use bitflags::bitflags;
bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct SvtFlags: u32 {
        const BUFFERFLAG_EOS = 1;
        const BUFFERFLAG_SHOW_EXT = 2;
        const BUFFERFLAG_HAS_TD = 4;
        const BUFFERFLAG_IS_ALT_REF = 8;
        const BUFFERFLAG_ERROR_MASK = 4294967280;
    }
}

#[repr(u32)]
/// List of supported color primaries
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ColorPrimaries {
    /// For future use
    CicpCpReserved0 = 0,
    /// BT.709
    CicpCpBt709 = 1,
    /// Unspecified
    CicpCpUnspecified = 2,
    /// For future use
    CicpCpReserved3 = 3,
    /// BT.470 System M (historical)
    CicpCpBt470M = 4,
    /// BT.470 System B, G (historical)
    CicpCpBt470BG = 5,
    /// BT.601
    CicpCpBt601 = 6,
    /// SMPTE 240
    CicpCpSmpte240 = 7,
    /// Generic film (color filters using illuminant C)
    CicpCpGenericFilm = 8,
    /// BT.2020, BT.2100
    CicpCpBt2020 = 9,
    /// SMPTE 428 (CIE 1921 XYZ)
    CicpCpXyz = 10,
    /// SMPTE RP 431-2
    CicpCpSmpte431 = 11,
    /// SMPTE EG 432-1
    CicpCpSmpte432 = 12,
    /// For future use (values 13 - 21)
    CicpCpReserved13 = 13,
    /// EBU Tech. 3213-E
    CicpCpEbu3213 = 22,
    /// For future use (values 23 - 255)
    CicpCpReserved23 = 23,
    /// For future use (values 24 - 255)
    CicpCpReserved24 = 24,
    /// For future use (values 25 - 255)
    CicpCpReserved25 = 25,
    /// For future use (values 26 - 255)
    CicpCpReserved26 = 26,
}
#[repr(u32)]
/// List of supported transfer functions
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TransferCharacteristics {
    /// For future use
    CicpTcReserved0 = 0,
    /// BT.709
    CicpTcBt709 = 1,
    /// Unspecified
    CicpTcUnspecified = 2,
    /// For future use
    CicpTcReserved3 = 3,
    /// BT.470 System M (historical)
    CicpTcBt470M = 4,
    /// BT.470 System B, G (historical)
    CicpTcBt470BG = 5,
    /// BT.601
    CicpTcBt601 = 6,
    /// SMPTE 240 M
    CicpTcSmpte240 = 7,
    /// Linear
    CicpTcLinear = 8,
    /// Logarithmic (100 : 1 range)
    CicpTcLog100 = 9,
    /// Logarithmic (100 * Sqrt(10) : 1 range)
    CicpTcLog100Sqrt10 = 10,
    /// IEC 61966-2-4
    CicpTcIec61966 = 11,
    /// BT.1361
    CicpTcBt1361 = 12,
    /// sRGB or sYCC
    CicpTcSrgb = 13,
    /// BT.2020 10-bit systems
    CicpTcBt2020_10Bit = 14,
    /// BT.2020 12-bit systems
    CicpTcBt2020_12Bit = 15,
    /// SMPTE ST 2084, ITU BT.2100 PQ
    CicpTcSmpte2084 = 16,
    /// SMPTE ST 428
    CicpTcSmpte428 = 17,
    /// BT.2100 HLG, ARIB STD-B67
    CicpTcHlg = 18,
    /// For future use (values 19-255)
    CicpTcReserved19 = 19,
    /// For future use (values 20-255)
    CicpTcReserved20 = 20,
    /// For future use (values 21-255)
    CicpTcReserved2121,
    /// For future use (values 22-255)
    CicpTcReserved22 = 22,
    /// For future use (values 23-255)
    CicpTcReserved23 = 23,
}
#[repr(u32)]
/// List of supported matrix coefficients
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MatrixCoefficients {
    /// Identity matrix
    CicpMcIdentity = 0,
    /// BT.709
    CicpMcBt709 = 1,
    /// Unspecified
    CicpMcUnspecified = 2,
    /// For future use
    CicpMcReserved3 = 3,
    /// US FCC 73.628
    CicpMcFcc = 4,
    /// BT.470 System B, G (historical)
    CicpMcBt470BG = 5,
    /// BT.601
    CicpMcBt601 = 6,
    /// SMPTE 240 M
    CicpMcSmpte240 = 7,
    /// YCgCo
    CicpMcSmpteYcgco = 8,
    /// BT.2020 non-constant luminance, BT.2100 YCbCr
    CicpMcBt2020Ncl = 9,
    /// BT.2020 constant luminance
    CicpMcBt2020Cl = 10,
    /// SMPTE ST 2085 YDzDx
    CicpMcSmpte2085 = 11,
    /// Chromaticity-derived non-constant luminance
    CicpMcChromatNcl = 12,
    /// Chromaticity-derived constant luminance
    CicpMcChromatCl = 13,
    /// BT.2100 ICtCp
    CicpMcIctcp = 14,
    /// For future use (values 15-255)
    CicpMcReserved15 = 15,
    /// For future use (values 16-255)
    CicpMcReserved16 = 16,
    /// For future use (values 17-255)
    CicpMcReserved17 = 17,
    /// For future use (values 18-255)
    CicpMcReserved18 = 18,
}
#[repr(u32)]
/// List of supported color range
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ColorRange {
    /// Y [16..235], UV [16..240]
    CrStudioRange = 0,
    /// YUV/RGB [0..255]
    CrFullRange = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BitDepth {
    EightBit = 8,
    TenBit = 10,
    TwelveBit = 12,
    FourteenBit = 14,
    SixteenBit = 16,
    ThirtytwoBit = 32,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ColorFormat {
    YUV400 = 0,
    YUV420 = 1,
    YUV422 = 2,
    YUV444 = 3,
}
#[repr(u32)]
/// List of chroma sample positions
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ChromaSamplePosition {
    /// Unknown
    CspUnknown = 0,
    /// Horizontally co-located with luma(0, 0)*/\n/**< sample, between two vertical samples
    CspVertical = 1,
    /// Co-located with luma(0, 0) sample
    CspColocated = 2,
    /// Reserved value
    CspReserved = 3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SvtMetadataArray {
    _unused: [u8; 0],
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Av1PictureType {
    Av1InterPicture = 0,
    Av1AltRefPicture = 1,
    Av1IntraOnlyPicture = 2,
    Av1KeyPicture = 3,
    Av1NonRefPicture = 4,
    Av1ShowExistingPicture = 6,
    Av1FwKeyPicture = 5,
    Av1SwitchPicture = 7,
    Av1InvalidPicture = 255,
}
/// The Bool type is intended to be used to represent a true or a false
/// value when passing parameters to and from the eBrisk API.
/// The Bool is an 8 bit quantity.
pub type Bool = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BufferHeaderType {
    pub size: u32,
    pub p_buffer: *mut u8,
    pub n_filled_len: u32,
    pub n_alloc_len: u32,
    pub p_app_private: *mut ::std::os::raw::c_void,
    pub wrapper_ptr: *mut ::std::os::raw::c_void,
    pub n_tick_count: u32,
    pub dts: i64,
    pub pts: i64,
    pub qp: u32,
    pub pic_type: Av1PictureType,
    pub luma_sse: u64,
    pub cr_sse: u64,
    pub cb_sse: u64,
    pub flags: u32,
    pub luma_ssim: f64,
    pub cr_ssim: f64,
    pub cb_ssim: f64,
    pub metadata: *mut SvtMetadataArray,
}

impl Default for BufferHeaderType {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ComponentType {
    pub size: u32,
    pub p_component_private: *mut ::std::os::raw::c_void,
    pub p_application_private: *mut ::std::os::raw::c_void,
}

impl Default for ComponentType {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ErrorType {
    ErrorNone = 0,
    DecUnsupportedBitstream = 1073745920,
    DecNoOutputPicture = 1073745924,
    DecDecodingError = 1073745928,
    CorruptFrame = 1073745932,
    ErrorInsufficientResources = -2147479552,
    ErrorUndefined = -2147479551,
    ErrorInvalidComponent = -2147479548,
    ErrorBadParameter = -2147479547,
    ErrorDestroyThreadFailed = -2147475438,
    ErrorSemaphoreUnresponsive = -2147475423,
    ErrorDestroySemaphoreFailed = -2147475422,
    ErrorCreateMutexFailed = -2147475408,
    ErrorMutexUnresponsive = -2147475407,
    ErrorDestroyMutexFailed = -2147475406,
    NoErrorEmptyQueue = -2147475405,
    NoErrorFifoShutdown = -2147475404,
    ErrorMax = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Av1SeqProfile {
    MainProfile = 0,
    HighProfile = 1,
    ProfessionalProfile = 2,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct OperatingParametersInfo {
    pub decoder_buffer_delay: u32,
    pub encoder_buffer_delay: u32,
    pub low_delay_mode_flag: u8,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AV1OperatingPoint {
    pub op_idc: u32,
    pub seq_level_idx: u32,
    pub seq_tier: u32,
    pub decoder_model_present_for_this_op: u8,
    pub operating_parameters_info: OperatingParametersInfo,
    pub initial_display_delay_present_for_this_op: u32,
    pub initial_display_delay: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SvtIOFormat {
    pub luma: *const u8,
    pub cb: *const u8,
    pub cr: *const u8,
    pub y_stride: u32,
    pub cr_stride: u32,
    pub cb_stride: u32,
    pub width: u32,
    pub height: u32,
    pub org_x: u32,
    pub org_y: u32,
    pub color_fmt: ColorFormat,
    pub bit_depth: BitDepth,
}

impl Default for SvtIOFormat {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

pub type Av1OperatingPoint = AV1OperatingPoint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ColorConfig {
    pub bit_depth: BitDepth,
    pub mono_chrome: Bool,
    pub subsampling_x: u8,
    pub subsampling_y: u8,
    pub color_description_present_flag: Bool,
    pub color_primaries: ColorPrimaries,
    pub transfer_characteristics: TransferCharacteristics,
    pub matrix_coefficients: MatrixCoefficients,
    pub color_range: ColorRange,
    pub chroma_sample_position: ChromaSamplePosition,
    pub separate_uv_delta_q: Bool,
}

impl Default for ColorConfig {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TimingInfo {
    pub timing_info_present: Bool,
    pub num_units_in_display_tick: u32,
    pub time_scale: u32,
    pub equal_picture_interval: u8,
    pub num_ticks_per_picture: u32,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PrivDataType {
    PrivateData = 0,
    RefFrameScalingEvent = 1,
    RoiMapEvent = 2,
    ResChangeEvent = 3,
    RateChangeEvent = 4,
    PrivateDataTypes = 5,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PrivDataNode {
    pub node_type: PrivDataType,
    pub data: *mut ::std::os::raw::c_void,
    pub size: u32,
    pub next: *mut PrivDataNode,
}

impl Default for PrivDataNode {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct RefFrameScale {
    pub scale_mode: u8,
    pub scale_denom: u32,
    pub scale_kf_denom: u32,
}

/// Structure containing film grain synthesis parameters for a frame
/// This structure contains input parameters for film grain synthesis
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AomFilmGrain {
    pub apply_grain: i32,
    pub update_parameters: i32,
    pub scaling_points_y: [[i32; 2usize]; 14usize],
    pub num_y_points: i32,
    pub scaling_points_cb: [[i32; 2usize]; 10usize],
    pub num_cb_points: i32,
    pub scaling_points_cr: [[i32; 2usize]; 10usize],
    pub num_cr_points: i32,
    pub scaling_shift: i32,
    pub ar_coeff_lag: i32,
    pub ar_coeffs_y: [i32; 24usize],
    pub ar_coeffs_cb: [i32; 25usize],
    pub ar_coeffs_cr: [i32; 25usize],
    pub ar_coeff_shift: i32,
    pub cb_mult: i32,
    pub cb_luma_mult: i32,
    pub cb_offset: i32,
    pub cr_mult: i32,
    pub cr_luma_mult: i32,
    pub cr_offset: i32,
    pub overlap_flag: i32,
    pub clip_to_restricted_range: i32,
    pub bit_depth: i32,
    pub chroma_scaling_from_luma: i32,
    pub grain_scale_shift: i32,
    pub random_seed: u16,
    pub ignore_ref: i32,
}

/// CPU FLAGS
pub type CpuFlags = u64;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ContentLightLevel {
    pub max_cll: u16,
    pub max_fall: u16,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct SvtAv1ChromaPoints {
    pub x: u16,
    pub y: u16,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct SvtAv1MasteringDisplayInfo {
    pub r: SvtAv1ChromaPoints,
    pub g: SvtAv1ChromaPoints,
    pub b: SvtAv1ChromaPoints,
    pub white_point: SvtAv1ChromaPoints,
    pub max_luma: u32,
    pub min_luma: u32,
}

#[repr(u32)]
///  The SvtAv1IntraRefreshType is used to describe the intra refresh type.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SvtAv1IntraRefreshType {
    SvtAv1FwdkfRefresh = 1,
    SvtAv1KfRefresh = 2,
}
/// Generic fixed size buffer structure\n\n This structure is able to hold a reference to any fixed size buffer.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SvtAv1FixedBuf {
    /// Pointer to the data. Does NOT own the data!
    pub buf: *mut ::std::os::raw::c_void,
    /// Length of the buffer, in chars
    pub sz: u64,
}

impl Default for SvtAv1FixedBuf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(u32)]
///  Indicates how an S-Frame should be inserted.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SFrameMode {
    SframeStrictBase = 1,
    SframeNearestBase = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SvtAv1FrameScaleEvts {
    pub evt_num: u32,
    pub start_frame_nums: *mut u64,
    pub resize_kf_denoms: *mut u32,
    pub resize_denoms: *mut u32,
}

impl Default for SvtAv1FrameScaleEvts {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SvtAv1EncConfiguration {
    ///  Encoder preset used.
    /// -2 and -1 are for debug purposes and should not be used.
    /// 0 is the highest quality mode but is the slowest,
    /// 13 is the fastest mode but is not as high quality.\
    ///
    /// Min value is -2.
    /// Max value is 13.
    /// Default is 12.
    pub enc_mode: i8,
    pub intra_period_length: i32,
    pub intra_refresh_type: SvtAv1IntraRefreshType,
    pub hierarchical_levels: u32,
    pub pred_structure: u8,
    ///  Frame width in pixels.
    ///  Min is 64.
    ///  Max is 16384.
    ///  Default is 0.
    pub source_width: u32,
    ///  Frame height in pixels
    ///  Min is 64.
    ///  Max is 8704.
    ///  Default is 0.
    pub source_height: u32,
    pub forced_max_frame_width: u32,
    pub forced_max_frame_height: u32,
    pub frame_rate_numerator: u32,
    pub frame_rate_denominator: u32,
    pub encoder_bit_depth: u32,
    ///  Encoder color format.
    ///  Only YUV420 is supported for now.
    ///  Min is YUV400.
    ///  Max is YUV444.
    ///  Default is YUV420.
    pub encoder_color_format: ColorFormat,
    ///  Currently unused.
    ///  Default is 0.
    pub high_dynamic_range_input: u8,
    ///  Bitstream profile to use.
    ///  0: main, 1: high, 2: professional.
    ///  Min is MAIN_PROFILE.
    ///  Max is PROFESSIONAL_PROFILE.
    ///  Default is MAIN_PROFILE.
    pub profile: Av1SeqProfile,
    pub tier: u32,
    ///  Bitstream level.
    ///  0: autodetect from bitstream, 20: level 2.0, 63: level 6.3, only levels 2.0-6.3 are properly defined.
    ///  The levels are defined at https:///aomediacodec.github.io/av1-spec/av1-spec.pdf
    ///  under \A.3. Levels\".
    ///  Min is 0.
    ///  Max is 73.
    ///  Default is 0.
    pub level: u32,
    pub color_description_present_flag: Bool,
    pub color_primaries: ColorPrimaries,
    pub transfer_characteristics: TransferCharacteristics,
    pub matrix_coefficients: MatrixCoefficients,
    pub color_range: ColorRange,
    pub mastering_display: SvtAv1MasteringDisplayInfo,
    pub content_light_level: ContentLightLevel,
    pub chroma_sample_position: ChromaSamplePosition,
    pub rate_control_mode: u32,
    pub qp: u32,
    pub use_qp_file: Bool,
    pub target_bit_rate: u32,
    pub max_bit_rate: u32,
    pub max_qp_allowed: u32,
    pub min_qp_allowed: u32,
    ///  Variable Bit Rate Minimum Section Percentage
    ///  Indicates the minimum bitrate to be used for a single GOP as a percentage
    ///  of the target bitrate.
    ///  Min is 0.
    ///  Max is 100.
    ///  Default is 0.
    pub vbr_min_section_pct: u32,
    ///  Variable Bit Rate Maximum Section Percentage
    ///  Indicates the maximum bitrate to be used for a single GOP as a percentage
    ///  of the target bitrate.
    ///  Min is 0.
    ///  Max is 10000.
    ///  Default is 2000.
    pub vbr_max_section_pct: u32,
    ///  UnderShoot Percentage
    ///  Only applicable for VBR and CBR.
    ///  Indicates the tolerance of the VBR algorithm to undershoot and is used
    ///  as a trigger threshold for more agressive adaptation of Quantization.
    ///  Min is 0.
    ///  Max is 100.
    ///  Default is 25 for CBR and 50 for VBR.
    pub under_shoot_pct: u32,
    ///  OverShoot Percentage
    ///  Only applicable for VBR and CBR
    ///  Indicates the tolerance of the VBR algorithm to overshoot and is used as
    ///  a trigger threshold for more agressive adaptation of Quantization.
    ///  Min is 0.
    ///  Max is 100.
    ///  Default is 25.
    pub over_shoot_pct: u32,
    ///  MaxBitRate OverShoot Percentage
    ///  Only applicable for Capped CRF.
    ///  Indicates the tolerance of the Capped CRF algorithm to overshoot
    ///  and is used as a trigger threshold for more agressive adaptation of
    ///  Quantization.
    ///  Min is 0.
    ///  Max is 100.
    ///  Default is 50.
    pub mbr_over_shoot_pct: u32,
    ///  Starting Buffer Level in MilliSeconds
    ///  Only applicable for CBR.
    ///  Indicates the amount of data that will be buffered by the decoding
    ///  application prior to beginning playback, and is expressed in units of
    ///  time. Must be less than maximum_buffer_size_ms.
    ///  Min is 20.
    ///  Max is 10000.
    ///  Default is 600.
    pub starting_buffer_level_ms: i64,
    ///  Optimal Buffer Level in MilliSeconds
    ///  Only applicable for CBR.
    ///  indicates the amount of data that the encoder should try to maintain in the
    ///  decoder's buffer, and is expressed in units of time. Must be less than
    ///  maximum_buffer_size_ms.
    ///  Min is 20.
    ///  Max is 10000.
    ///  Default is 600.
    pub optimal_buffer_level_ms: i64,
    ///  Maximum Buffer Size in MilliSeconds
    ///  Only applicable for CBR.
    ///  indicates the maximum amount of data that may be buffered by the
    ///  decoding application, and is expressed in units of time.
    ///  Min is 20.
    ///  Max is 10000.
    ///  Default is 1000.
    pub maximum_buffer_size_ms: i64,
    pub rc_stats_buffer: SvtAv1FixedBuf,
    pub pass: ::std::os::raw::c_int,
    pub use_fixed_qindex_offsets: u8,
    pub qindex_offsets: [i32; 6usize],
    pub key_frame_chroma_qindex_offset: i32,
    pub key_frame_qindex_offset: i32,
    pub chroma_qindex_offsets: [i32; 6usize],
    pub luma_y_dc_qindex_offset: i32,
    pub chroma_u_dc_qindex_offset: i32,
    pub chroma_u_ac_qindex_offset: i32,
    pub chroma_v_dc_qindex_offset: i32,
    pub chroma_v_ac_qindex_offset: i32,
    ///  Deblocking loop filter control
    ///  Default is true.
    pub enable_dlf_flag: Bool,
    pub film_grain_denoise_strength: u32,
    ///  Determines how much denoising is used.
    ///  Only applicable when film grain is ON.
    ///  0 is no denoising
    ///  1 is full denoising
    ///  Default is 0.
    pub film_grain_denoise_apply: u8,
    pub cdef_level: ::std::os::raw::c_int,
    pub enable_restoration_filtering: ::std::os::raw::c_int,
    pub enable_mfmv: ::std::os::raw::c_int,
    pub scene_change_detection: u32,
    ///  API signal to constrain motion vectors.
    ///  Default is false.
    pub restricted_motion_vector: Bool,
    pub tile_columns: i32,
    pub tile_rows: i32,
    pub look_ahead_distance: u32,
    pub enable_tpl_la: u8,
    pub recode_loop: u32,
    pub screen_content_mode: u32,
    pub enable_adaptive_quantization: u8,
    ///  Enable use of ALT-REF (temporally filtered) frames.
    ///  Default is true.
    pub enable_tf: Bool,
    pub enable_overlays: Bool,
    ///  Tune for a particular metric; 0: VQ, 1: PSNR, 2: SSIM.
    ///  Default is 1.
    pub tune: u8,
    pub superres_mode: u8,
    pub superres_denom: u8,
    pub superres_kf_denom: u8,
    pub superres_qthres: u8,
    pub superres_kf_qthres: u8,
    pub superres_auto_search_type: u8,
    pub fast_decode: u8,
    pub sframe_dist: i32,
    pub sframe_mode: SFrameMode,
    ///  API signal for the library to know the channel ID (used for pinning to cores).
    ///  Min value is 0.
    ///  Max value is 0xFFFFFFFF.
    ///  Default is 0.
    pub channel_id: u32,
    ///  API signal for the library to know the active number of channels being encoded simultaneously.
    ///  Min value is 1.
    ///  Max value is 0xFFFFFFFF.
    ///  Default is 1.
    pub active_channel_count: u32,
    pub logical_processors: u32,
    pub level_of_parallelism: u32,
    pub pin_threads: u32,
    pub target_socket: i32,
    pub use_cpu_flags: CpuFlags,
    pub stat_report: u32,
    ///  API Signal to output reconstructed yuv used for debug purposes.
    ///  Using this will affect the speed of encoder.
    ///  Default is false.
    pub recon_enabled: Bool,
    ///  Signal that force-key-frames is enabled.
    ///
    pub force_key_frames: Bool,
    ///  Signal to the library to treat intra_period_length as seconds and
    ///  multiply by fps_num/fps_den.
    pub multiply_keyint: Bool,
    ///  Reference scaling mode
    ///  the available modes are defined in RESIZE_MODE
    pub resize_mode: u8,
    ///  Resize denominator
    ///  this value can be from 8 to 16, means downscaling to 8/8-8/16 of original
    ///  resolution in both width and height
    pub resize_denom: u8,
    ///  Resize denominator of key frames
    ///  this value can be from 8 to 16, means downscaling to 8/8-8/16 of original
    ///  resolution in both width and height
    pub resize_kf_denom: u8,
    ///  Signal to the library to enable quantisation matrices
    ///  Default is false.
    pub enable_qm: Bool,
    ///  Min quant matrix flatness. Applicable when enable_qm is true.
    ///  Min value is 0.
    ///  Max value is 15.
    ///  Default is 8.
    pub min_qm_level: u8,
    ///  Max quant matrix flatness. Applicable when enable_qm is true.
    ///  Min value is 0.
    ///  Max value is 15.
    ///  Default is 15.
    pub max_qm_level: u8,
    ///  gop_constraint_rc
    ///  Currently, only applicable for VBR and  when GoP size is greater than 119 frames.
    ///  When enabled, the rate control matches the target rate for each GoP.
    ///  0: off
    ///  1: on
    ///  Default is 0.
    pub gop_constraint_rc: Bool,
    ///  scale factors for lambda value for different frame update types
    ///  factor >> 7 (/ 128) is the actual value in float
    pub lambda_scale_factors: [i32; 7usize],
    pub enable_dg: Bool,
    ///  startup_mg_size
    ///  When enabled, a MG with specified size will be inserted after the key frame.
    ///  The MG size is determined by 2^startup_mg_size.
    ///  0: off
    ///  2: set hierarchical levels to 2 (MG size 4)
    ///  3: set hierarchical levels to 3 (MG size 8)
    ///  4: set hierarchical levels to 4 (MG size 16)
    ///  Default is 0.
    pub startup_mg_size: u8,
    pub frame_scale_evts: SvtAv1FrameScaleEvts,
    pub enable_roi_map: Bool,
    pub fgs_table: *mut AomFilmGrain,
    pub enable_variance_boost: Bool,
    pub variance_boost_strength: u8,
    pub variance_octile: u8,
    pub padding: [u8; 121usize],
}

impl Default for SvtAv1EncConfiguration {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

unsafe extern "C" {
    /// Returns a string containing version
    pub fn svt_av1_get_version() -> *const ::std::os::raw::c_char;

    /// Prints the version header and build information to the file\n specified by the SVT_LOG_FILE environment variable or stderr
    pub fn svt_av1_print_version();

    pub fn svt_av1_enc_init_handle(
        p_handle: *mut *mut ComponentType,
        p_app_data: *mut ::std::os::raw::c_void,
        config_ptr: *mut SvtAv1EncConfiguration,
    ) -> ErrorType;

    pub fn svt_av1_enc_set_parameter(
        svt_enc_component: *mut ComponentType,
        pComponentParameterStructure: *mut SvtAv1EncConfiguration,
    ) -> ErrorType;

    pub fn svt_av1_enc_parse_parameter(
        pComponentParameterStructure: *mut SvtAv1EncConfiguration,
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> ErrorType;

    pub fn svt_av1_enc_init(svt_enc_component: *mut ComponentType) -> ErrorType;

    pub fn svt_av1_enc_stream_header(
        svt_enc_component: *mut ComponentType,
        output_stream_ptr: *mut *mut BufferHeaderType,
    ) -> ErrorType;

    pub fn svt_av1_enc_stream_header_release(stream_header_ptr: *mut BufferHeaderType)
        -> ErrorType;

    pub fn svt_av1_enc_send_picture(
        svt_enc_component: *mut ComponentType,
        p_buffer: *mut BufferHeaderType,
    ) -> ErrorType;

    pub fn svt_av1_enc_get_packet(
        svt_enc_component: *mut ComponentType,
        p_buffer: *mut *mut BufferHeaderType,
        pic_send_done: u8,
    ) -> ErrorType;

    pub fn svt_av1_enc_release_out_buffer(p_buffer: *mut *mut BufferHeaderType);

    pub fn svt_av1_get_recon(
        svt_enc_component: *mut ComponentType,
        p_buffer: *mut BufferHeaderType,
    ) -> ErrorType;

    pub fn svt_av1_enc_get_stream_info(
        svt_enc_component: *mut ComponentType,
        stream_info_id: u32,
        info: *mut ::std::os::raw::c_void,
    ) -> ErrorType;

    pub fn svt_av1_enc_deinit(svt_enc_component: *mut ComponentType) -> ErrorType;

    pub fn svt_av1_enc_deinit_handle(svt_enc_component: *mut ComponentType) -> ErrorType;
}
