//! Bindings for SVT-AV1-PSYEX v3.0.2-A.
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
    CicpCpReserved0   = 0,
    /// BT.709
    CicpCpBt709       = 1,
    /// Unspecified
    CicpCpUnspecified = 2,
    /// For future use
    CicpCpReserved3   = 3,
    /// BT.470 System M (historical)
    CicpCpBt470M      = 4,
    /// BT.470 System B, G (historical)
    CicpCpBt470BG     = 5,
    /// BT.601
    CicpCpBt601       = 6,
    /// SMPTE 240
    CicpCpSmpte240    = 7,
    /// Generic film (color filters using illuminant C)
    CicpCpGenericFilm = 8,
    /// BT.2020, BT.2100
    CicpCpBt2020      = 9,
    /// SMPTE 428 (CIE 1921 XYZ)
    CicpCpXyz         = 10,
    /// SMPTE RP 431-2
    CicpCpSmpte431    = 11,
    /// SMPTE EG 432-1
    CicpCpSmpte432    = 12,
    /// For future use (values 13 - 21)
    CicpCpReserved13  = 13,
    /// EBU Tech. 3213-E
    CicpCpEbu3213     = 22,
    /// For future use (values 23 - 255)
    CicpCpReserved23  = 23,
    /// For future use (values 24 - 255)
    CicpCpReserved24  = 24,
    /// For future use (values 25 - 255)
    CicpCpReserved25  = 25,
    /// For future use (values 26 - 255)
    CicpCpReserved26  = 26,
}

#[repr(u32)]
/// List of supported transfer functions
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TransferCharacteristics {
    /// For future use
    CicpTcReserved0    = 0,
    /// BT.709
    CicpTcBt709        = 1,
    /// Unspecified
    CicpTcUnspecified  = 2,
    /// For future use
    CicpTcReserved3    = 3,
    /// BT.470 System M (historical)
    CicpTcBt470M       = 4,
    /// BT.470 System B, G (historical)
    CicpTcBt470BG      = 5,
    /// BT.601
    CicpTcBt601        = 6,
    /// SMPTE 240 M
    CicpTcSmpte240     = 7,
    /// Linear
    CicpTcLinear       = 8,
    /// Logarithmic (100 : 1 range)
    CicpTcLog100       = 9,
    /// Logarithmic (100 * Sqrt(10) : 1 range)
    CicpTcLog100Sqrt10 = 10,
    /// IEC 61966-2-4
    CicpTcIec61966     = 11,
    /// BT.1361
    CicpTcBt1361       = 12,
    /// sRGB or sYCC
    CicpTcSrgb         = 13,
    /// BT.2020 10-bit systems
    CicpTcBt2020_10Bit = 14,
    /// BT.2020 12-bit systems
    CicpTcBt2020_12Bit = 15,
    /// SMPTE ST 2084, ITU BT.2100 PQ
    CicpTcSmpte2084    = 16,
    /// SMPTE ST 428
    CicpTcSmpte428     = 17,
    /// BT.2100 HLG, ARIB STD-B67
    CicpTcHlg          = 18,
    /// For future use (values 19-255)
    CicpTcReserved19   = 19,
    /// For future use (values 20-255)
    CicpTcReserved20   = 20,
    /// For future use (values 21-255)
    CicpTcReserved21   = 21,
    /// For future use (values 22-255)
    CicpTcReserved22   = 22,
    /// For future use (values 23-255)
    CicpTcReserved23   = 23,
}

#[repr(u32)]
/// List of supported matrix coefficients
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MatrixCoefficients {
    /// Identity matrix
    CicpMcIdentity    = 0,
    /// BT.709
    CicpMcBt709       = 1,
    /// Unspecified
    CicpMcUnspecified = 2,
    /// For future use
    CicpMcReserved3   = 3,
    /// US FCC 73.628
    CicpMcFcc         = 4,
    /// BT.470 System B, G (historical)
    CicpMcBt470BG     = 5,
    /// BT.601
    CicpMcBt601       = 6,
    /// SMPTE 240 M
    CicpMcSmpte240    = 7,
    /// YCgCo
    CicpMcSmpteYcgco  = 8,
    /// BT.2020 non-constant luminance, BT.2100 YCbCr
    CicpMcBt2020Ncl   = 9,
    /// BT.2020 constant luminance
    CicpMcBt2020Cl    = 10,
    /// SMPTE ST 2085 YDzDx
    CicpMcSmpte2085   = 11,
    /// Chromaticity-derived non-constant luminance
    CicpMcChromatNcl  = 12,
    /// Chromaticity-derived constant luminance
    CicpMcChromatCl   = 13,
    /// BT.2100 ICtCp
    CicpMcIctcp       = 14,
    /// For future use (values 15-255)
    CicpMcReserved15  = 15,
    /// For future use (values 16-255)
    CicpMcReserved16  = 16,
    /// For future use (values 17-255)
    CicpMcReserved17  = 17,
    /// For future use (values 18-255)
    CicpMcReserved18  = 18,
}

#[repr(u32)]
/// List of supported color range
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ColorRange {
    /// Y [16..235], UV [16..240]
    CrStudioRange = 0,
    /// YUV/RGB [0..255]
    CrFullRange   = 1,
}

#[repr(u32)]
/// AV1 bit depth
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BitDepth {
    EightBit     = 8,
    TenBit       = 10,
    TwelveBit    = 12,
    FourteenBit  = 14, // Not supported
    SixteenBit   = 16, // Not supported
    ThirtytwoBit = 32, // Not supported
}

#[repr(u32)]
/// AV1 Chroma Format
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
    CspUnknown   = 0,
    /// Horizontally co-located with luma(0, 0)
    /// sample, between two vertical samples
    CspVertical  = 1,
    /// Co-located with luma(0, 0) sample
    CspColocated = 2,
    /// Reserved value
    CspReserved  = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SvtMetadataArray {
    _unused: [u8; 0],
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Av1PictureType {
    Av1InterPicture        = 0,
    Av1AltRefPicture       = 1,
    Av1IntraOnlyPicture    = 2,
    Av1KeyPicture          = 3,
    Av1NonRefPicture       = 4,
    Av1ShowExistingPicture = 6,
    Av1FwKeyPicture        = 5,
    Av1SwitchPicture       = 7,
    Av1InvalidPicture      = 0xFF,
}

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
    pub temporal_layer_index: u8,
    pub qp: u32,
    pub avg_qp: u32,
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
    ErrorNone                   = 0,
    DecUnsupportedBitstream     = 0x40001000u32 as i32,
    DecNoOutputPicture          = 0x40001004u32 as i32,
    DecDecodingError            = 0x40001008u32 as i32,
    CorruptFrame                = 0x4000100Cu32 as i32,
    ErrorInsufficientResources  = 0x80001000u32 as i32,
    ErrorUndefined              = 0x80001001u32 as i32,
    ErrorInvalidComponent       = 0x80001004u32 as i32,
    ErrorBadParameter           = 0x80001005u32 as i32,
    ErrorDestroyThreadFailed    = 0x80002012u32 as i32,
    ErrorSemaphoreUnresponsive  = 0x80002021u32 as i32,
    ErrorDestroySemaphoreFailed = 0x80002022u32 as i32,
    ErrorCreateMutexFailed      = 0x80002030u32 as i32,
    ErrorMutexUnresponsive      = 0x80002031u32 as i32,
    ErrorDestroyMutexFailed     = 0x80002032u32 as i32,
    NoErrorEmptyQueue           = 0x80002033u32 as i32,
    NoErrorFifoShutdown         = 0x80002034u32 as i32,
    ErrorMax                    = 0x7FFFFFFF,
}

#[repr(u32)]
/// AV1 bistream profile (seq_profile syntax element)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Av1SeqProfile {
    MainProfile = 0,
    HighProfile = 1,
    ProfessionalProfile = 2,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct OperatingParametersInfo {
    /// Specifies the time interval between the arrival of the first bit in the
    /// smoothing buffer and the subsequent removal of the data that belongs to
    /// the first coded frame for operating point
    pub decoder_buffer_delay: u32,
    /// Specifies, in combination with decoder_buffer_delay[op] syntax element,
    /// the first bit arrival time of frames to be decoded to the smoothing
    /// buffer
    pub encoder_buffer_delay: u32,
    /// Equal to 1 indicates that the smoothing buffer operates in low-delay
    /// mode for operating point
    pub low_delay_mode_flag: u8,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AV1OperatingPoint {
    pub op_idc: u32,
    pub seq_level_idx: u32,
    pub seq_tier: u32,
    /// 1 -> Indicates that there is a decoder model associated with operating
    ///     point,
    /// 0 -> Indicates that there is not a decoder model associated with
    ///     operating point
    pub decoder_model_present_for_this_op: u8,
    /// Operating Parameters Information structure
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
    /// bit depth
    pub bit_depth: BitDepth,
    /// 1: Indicates that the video does not contain U and V color planes.
    /// 0: Indicates that the video contains Y, U, and V color planes.
    pub mono_chrome: bool,
    /// Specify the chroma subsampling format
    pub subsampling_x: u8,
    /// Specify the chroma subsampling format
    pub subsampling_y: u8,
    /// An integer that is defined by the "Color primaries" section of
    /// ISO/IEC 23091-4/ITU-T H.273
    pub color_primaries: ColorPrimaries,
    /// An integer that is defined by the "Transfer characteristics" section
    /// of ISO/IEC 23091-4/ITU-T H.273
    pub transfer_characteristics: TransferCharacteristics,
    /// An integer that is defined by the "Matrix coefficients" section of
    /// ISO/IEC 23091-4/ITU-T H.273
    pub matrix_coefficients: MatrixCoefficients,
    /// 0: shall be referred to as the studio swing representation
    /// 1: shall be referred to as the full swing representation
    pub color_range: ColorRange,
    /// Specifies the sample position for subsampled streams
    pub chroma_sample_position: ChromaSamplePosition,
    /// 1: Indicates that the U and V planes may have separate delta quantizer
    /// 0: Indicates that the U and V planes will share the same delta
    ///    quantizer value
    pub separate_uv_delta_q: bool,
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
    /// Timing info present flag
    pub timing_info_present: bool,
    /// Number of time units of a clock operating at the frequency time_scale
    /// Hz that corresponds to one increment of a clock tick counter
    pub num_units_in_display_tick: u32,
    /// Number of time units that pass in one second
    pub time_scale: u32,
    /// Equal to 1 indicates that pictures should be displayed according to
    /// their output order with the number of ticks between two consecutive
    /// pictures specified by num_ticks_per_picture.
    pub equal_picture_interval: u8,
    /// Specifies the number of clock ticks corresponding to output time
    /// between two consecutive pictures in the output order.
    /// Range - [0 to (1 << 32) - 2]
    pub num_ticks_per_picture: u32,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PrivDataType {
    PrivateData          = 0,
    RefFrameScalingEvent = 1,
    RoiMapEvent          = 2,
    ResChangeEvent       = 3,
    RateChangeEvent      = 4,
    PrivateDataTypes     = 5,
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
/// Struct for storing content light level information
/// Values are stored in BE format
/// Refer to the AV1 specification 6.7.3 for more details
#[derive(Debug, Default, Copy, Clone)]
pub struct ContentLightLevel {
    pub max_cll: u16,
    pub max_fall: u16,
}

#[repr(C)]
/// Struct for storing x and y chroma points, values are stored in BE format
#[derive(Debug, Default, Copy, Clone)]
pub struct SvtAv1ChromaPoints {
    pub x: u16,
    pub y: u16,
}

#[repr(C)]
/// Struct for storing mastering-display information
/// values are stored in BE format
/// Refer to the AV1 specification 6.7.4 for more details
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
/// The SvtAv1IntraRefreshType is used to describe the intra refresh type.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SvtAv1IntraRefreshType {
    SvtAv1FwdkfRefresh = 1,
    SvtAv1KfRefresh    = 2,
}

/// Generic fixed size buffer structure
/// 
/// This structure is able to hold a reference to any fixed size buffer.
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
/// Indicates how an S-Frame should be inserted.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SFrameMode {
    /// The considered frame will be made into an S-Frame only if it is a base layer inter frame
    SframeStrictBase  = 1,
    /// If the considered frame is not an altref frame, the next base layer inter frame will be made into an S-Frame
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
    /// Encoder preset used.
    /// -2 and -1 are for debug purposes and should not be used.
    /// 0 is the highest quality mode but is the slowest,
    /// 13 is the fastest mode but is not as high quality.
    ///
    /// Min value is -2.
    /// Max value is 13.
    /// Default is 12.
    pub enc_mode: i8,
    /// The intra period defines the interval of frames after which you insert an
    /// Intra refresh. It is strongly recommended to set the value to multiple of
    /// 2^(hierarchical_levels), subtracting one if using open GOP (intra_refresh_type == 1).
    /// For instance, to get a 5-second GOP (default being >=5 seconds)
    /// with hierarchical_levels = 3 and open GOP you could use 319, 279, 159
    /// for 60, 50, or 30 respectively.
    /// 
    /// -1 = no intra update.
    /// -2 = auto.
    /// 
    /// Default is -2.
    pub intra_period_length: i32,
    /// Random access.
    /// 
    /// 1 = CRA, open GOP.
    /// 2 = IDR, closed GOP.
    /// 
    /// Default is 1.
    pub intra_refresh_type: SvtAv1IntraRefreshType,
    /// Number of hierarchical layers used to construct GOP.
    /// Minigop size = 2^HierarchicalLevels.
    /// 
    /// Default is 5 upt to M12 4, for M13.
    pub hierarchical_levels: u32,
    /// Prediction structure used to construct GOP. There are two main structures
    /// supported, which are: Low Delay (P or B) and Random Access.
    /// 
    /// In Low Delay structure, pictures within a mini GOP refer to the previously
    /// encoded pictures in display order. In other words, pictures with display
    /// order N can only be referenced by pictures with display order greater than
    /// N, and it can only refer pictures with picture order lower than N. The Low
    /// Delay structure can be flat structured (e.g. IPPPPPPP...) or hierarchically
    /// structured. B/b pictures can be used instead of P/p pictures. However, the
    /// reference picture list 0 and the reference picture list 1 will contain the
    /// same reference picture.
    /// 
    /// In Random Access structure, the B/b pictures can refer to reference pictures
    /// from both directions (past and future).
    /// 
    /// Refer to SvtAv1PredStructure enum for valid values.
    /// 
    /// Default is SVT_AV1_PRED_RANDOM_ACCESS.
    pub pred_structure: u8,
    /// Frame width in pixels.
    /// 
    /// Min is 64.
    /// Max is 16384.
    /// Default is 0.
    pub source_width: u32,
    /// Frame height in pixels
    /// 
    /// Min is 64.
    /// Max is 8704.
    /// Default is 0.
    pub source_height: u32,
    /// Specifies the maximum frame width/height for the frames represented by the sequence header
    /// (max_frame_width_minus_1 and max_frame_height_minus_1, spec 5.5.1).
    /// Actual frame height could be equal to or less than this value. E.g. Use this value to indicate
    /// the maximum height between renditions when switch frame feature is on.
    pub forced_max_frame_width: u32,
    pub forced_max_frame_height: u32,
    /// Frame rate numerator. When zero, the encoder will use -fps if
    /// FrameRateDenominator is also zero, otherwise an error is returned.
    /// 
    /// Default is 0.
    pub frame_rate_numerator: u32,
    /// Frame rate denominator. When zero, the encoder will use -fps if
    /// FrameRateNumerator is also zero, otherwise an error is returned.
    /// 
    /// Default is 0.
    pub frame_rate_denominator: u32,
    /// Specifies the bit depth of input video.
    /// 
    /// 8 = 8 bit.
    /// 10 = 10 bit.
    /// 
    /// Default is 10 for SVT-AV1-PSY, mainline default is 8.
    pub encoder_bit_depth: u32,
    /// Encoder color format.
    /// Only YUV420 is supported for now.
    /// 
    /// Min is YUV400.
    /// Max is YUV444.
    /// Default is YUV420.
    pub encoder_color_format: ColorFormat,
    /// Bitstream profile to use.
    /// 0: main, 1: high, 2: professional.
    /// 
    /// Min is MAIN_PROFILE.
    /// Max is PROFESSIONAL_PROFILE.
    /// Default is MAIN_PROFILE.
    pub profile: Av1SeqProfile,
    /// Constraints for bitstream in terms of max bitrate and max buffer size.
    /// 
    /// 0 = Main, for most applications.
    /// 1 = High, for demanding applications.
    /// 
    /// Default is 0.
    pub tier: u32,
    /// Bitstream level.
    /// 0: autodetect from bitstream, 20: level 2.0, 63: level 6.3, only levels 2.0-6.3 are properly defined.
    /// The levels are defined at https:///aomediacodec.github.io/av1-spec/av1-spec.pdf
    /// under \"A.3. Levels\".
    /// 
    /// Min is 0.
    /// Max is 73.
    /// Default is 0.
    pub level: u32,
    /// Color primaries
    /// values are from EbColorPrimaries
    /// Default is 2 (CP_UNSPECIFIED).
    pub color_primaries: ColorPrimaries,
    /// Transfer characteristics
    /// values are from EbTransferCharacteristics
    /// Default is 2 (TC_UNSPECIFIED).
    pub transfer_characteristics: TransferCharacteristics,
    /// Matrix coefficients
    /// values are from EbMatrixCoefficients
    /// Default is 2 (MC_UNSPECIFIED).
    pub matrix_coefficients: MatrixCoefficients,
    /// Color range
    /// values are from EbColorRange
    /// 0: studio swing.
    /// 1: full swing.
    /// Default is 0.
    pub color_range: ColorRange,
    /// Mastering display metadata
    /// values are from set using svt_aom_parse_mastering_display()
    pub mastering_display: SvtAv1MasteringDisplayInfo,
    /// Content light level
    /// values are from set using svt_aom_parse_content_light_level()
    pub content_light_level: ContentLightLevel,
    /// Chroma sample position
    /// Values as per 6.4.2 of the specification:
    /// EB_CSP_UNKNOWN:   default
    /// EB_CSP_VERTICAL:  value 0 from H.273 AKA "left"
    /// EB_CSP_COLOCATED: value 2 from H.273 AKA "top left"
    pub chroma_sample_position: ChromaSamplePosition,
    /// Rate control mode.
    /// 
    /// Refer to the SvtAv1RcMode enum for valid values
    /// Default is 0.
    pub rate_control_mode: u8,
    /// Initial quantization parameter for the Intra pictures used under constant
    /// qp rate control mode.
    /// 
    /// Default is 50.
    pub qp: u32,
    /// force qp values for every picture that are passed in the header pointer
    /// 
    /// Default is 0.
    pub use_qp_file: bool,
    /// Target bitrate in bits/second, only applicable when rate control mode is
    /// set to 1 (VBR) or 2 (CBR).
    /// 
    /// Default is 2000513.
    pub target_bit_rate: u32,
    /// maximum bitrate in bits/second, only apllicable when rate control mode is
    /// set to 0.
    /// 
    /// Default is 0.
    pub max_bit_rate: u32,
    /// Maxium QP value allowed for rate control use, only applicable when rate
    /// control mode is set to 1. It has to be greater or equal to minQpAllowed.
    /// 
    /// Default is 63.
    pub max_qp_allowed: u32,
    /// Minimum QP value allowed for rate control use, only applicable when rate
    /// control mode is set to 1 or 2. It has to be smaller or equal to maxQpAllowed.
    /// 
    /// Default is 4.
    pub min_qp_allowed: u32,
    /// Variable Bit Rate Minimum Section Percentage
    /// 
    /// Indicates the minimum bitrate to be used for a single GOP as a percentage
    /// of the target bitrate.
    /// 
    /// Min is 0.
    /// Max is 100.
    /// Default is 0.
    pub vbr_min_section_pct: u32,
    /// Variable Bit Rate Maximum Section Percentage
    /// 
    /// Indicates the maximum bitrate to be used for a single GOP as a percentage
    /// of the target bitrate.
    /// 
    /// Min is 0.
    /// Max is 10000.
    /// Default is 2000.
    pub vbr_max_section_pct: u32,
    /// UnderShoot Percentage
    /// 
    /// Only applicable for VBR and CBR.
    /// 
    /// Indicates the tolerance of the VBR algorithm to undershoot and is used
    /// as a trigger threshold for more agressive adaptation of Quantization.
    /// 
    /// Min is 0.
    /// Max is 100.
    /// Default is 25 for CBR and 50 for VBR.
    pub under_shoot_pct: u32,
    /// OverShoot Percentage
    /// 
    /// Only applicable for VBR and CBR
    /// 
    /// Indicates the tolerance of the VBR algorithm to overshoot and is used as
    /// a trigger threshold for more agressive adaptation of Quantization.
    /// 
    /// Min is 0.
    /// Max is 100.
    /// Default is 25.
    pub over_shoot_pct: u32,
    /// MaxBitRate OverShoot Percentage
    /// 
    /// Only applicable for Capped CRF.
    /// 
    /// Indicates the tolerance of the Capped CRF algorithm to overshoot
    /// and is used as a trigger threshold for more agressive adaptation of
    /// Quantization.
    /// 
    /// Min is 0.
    /// Max is 100.
    /// Default is 50.
    pub mbr_over_shoot_pct: u32,
    /// Starting Buffer Level in MilliSeconds
    /// 
    /// Only applicable for CBR.
    /// 
    /// Indicates the amount of data that will be buffered by the decoding
    /// application prior to beginning playback, and is expressed in units of
    /// time. Must be less than maximum_buffer_size_ms.
    /// 
    /// Min is 20.
    /// Max is 10000.
    /// Default is 600.
    pub starting_buffer_level_ms: i64,
    /// Optimal Buffer Level in MilliSeconds
    /// 
    /// Only applicable for CBR.
    /// 
    /// indicates the amount of data that the encoder should try to maintain in the
    /// decoder's buffer, and is expressed in units of time. Must be less than
    /// maximum_buffer_size_ms.
    /// 
    /// Min is 20.
    /// Max is 10000.
    /// Default is 600.
    pub optimal_buffer_level_ms: i64,
    /// Maximum Buffer Size in MilliSeconds
    /// 
    /// Only applicable for CBR.
    /// 
    /// indicates the maximum amount of data that may be buffered by the
    /// decoding application, and is expressed in units of time.
    /// 
    /// Min is 20.
    /// Max is 10000.
    /// Default is 1000.
    pub maximum_buffer_size_ms: i64,
    pub rc_stats_buffer: SvtAv1FixedBuf,
    pub pass: ::std::os::raw::c_int,
    /// use fixed qp offset for every picture based on temporal layer index
    /// 0: off (use the auto mode QP)
    /// 1: on (the offset is applied on top of the user QP)
    /// 2: on (the offset is applied on top of the auto mode QP)
    /// 
    /// Default is 0.
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
    /// Deblocking loop filter control
    /// 
    /// 0: disabled
    /// 1: enabled
    /// 2: more accurate (slower)
    pub enable_dlf_flag: u8,
    /// Film grain denoising the input picture
    /// Flag to enable the denoising
    /// 
    /// Default is 0.
    pub film_grain_denoise_strength: u32,
    /// Determines how much denoising is used.
    /// Only applicable when film grain is ON.
    /// 
    /// 0 is no denoising (default in SVT-AV1-PSY)
    /// 1 is full denoising
    /// 
    /// Default is 0.
    pub film_grain_denoise_apply: u8,
    /// CDEF Level
    /// 
    /// Default is -1.
    pub cdef_level: ::std::os::raw::c_int,
    /// Restoration filtering
    /// enable/disable
    /// set Self-Guided (sg) mode
    /// set Wiener (wn) mode
    /// 
    /// Default is -1.
    pub enable_restoration_filtering: ::std::os::raw::c_int,
    /// motion field motion vector
    /// 
    /// Default is -1.
    pub enable_mfmv: ::std::os::raw::c_int,
    /// Flag to enable the scene change detection algorithm.
    /// 
    /// Default is 1.
    pub scene_change_detection: u32,
    /// Log 2 Tile Rows and columns . 0 means no tiling,1 means that we split the dimension
    /// into 2
    /// Default is 0.
    pub tile_columns: i32,
    pub tile_rows: i32,
    /// When RateControlMode is set to 1 it's best to set this parameter to be
    /// equal to the Intra period value (such is the default set by the encoder).
    /// When CQP is chosen, then a (2 * minigopsize +1) look ahead is recommended.
    /// 
    /// Default depends on rate control mode.
    pub look_ahead_distance: u32,
    /// Enable TPL in look ahead
    /// 0 = disable TPL in look ahead
    /// 1 = enable TPL in look ahead
    /// Default is 0
    pub enable_tpl_la: u8,
    /// recode_loop indicates the recode levels,
    /// DISALLOW_RECODE = 0, No recode.
    /// ALLOW_RECODE_KFMAXBW = 1, Allow recode for KF and exceeding maximum frame bandwidth.
    /// ALLOW_RECODE_KFARFGF = 2, Allow recode only for KF/ARF/GF frames.
    /// ALLOW_RECODE = 3, Allow recode for all frames based on bitrate constraints.
    /// ALLOW_RECODE_DEFAULT = 4, Default setting, ALLOW_RECODE_KFARFGF for M0~5 and
    ///                                            ALLOW_RECODE_KFMAXBW for M6~8.
    /// default is 4
    pub recode_loop: u32,
    /// Flag to signal the content being a screen sharing content type
    /// 
    /// Default is 0.
    pub screen_content_mode: u32,
    /// Enable adaptive quantization within a frame using segmentation.
    /// 
    /// For rate control mode 0, setting this to 0 will use CQP mode, else CRF mode will be used.
    /// Default is 2.
    pub enable_adaptive_quantization: u8,
    /// Enable use of ALT-REF (temporally filtered) frames.
    /// 0 = off
    /// 1 = on
    /// 2 = adaptive
    /// Default is 1.
    pub enable_tf: u8,
    pub enable_overlays: bool,
    /// Tune for a particular metric; 0: VQ, 1: PSNR, 2: SSIM.
    /// 
    /// Default is 2 (Tune SSIM) for SVT-AV1-PSY. Mainline SVT-AV1 uses 1 (Tune PSNR) as default.
    pub tune: u8,
    pub superres_mode: u8,
    pub superres_denom: u8,
    pub superres_kf_denom: u8,
    pub superres_qthres: u8,
    pub superres_kf_qthres: u8,
    pub superres_auto_search_type: u8,
    /// Decoder-speed-targeted encoder optimization level (produce bitstreams that can be decoded faster).
    /// 0: No decoder-targeted speed optimization
    /// 1: Level 1 of decoder-targeted speed optimizations (faster decoder-speed than level 0)
    /// 2: Level 2 of decoder-targeted speed optimizations (faster decoder-speed than level 1)
    pub fast_decode: u8,
    /// S-Frame interval (frames)
    /// 0: S-Frame off
    /// >0: S-Frame on and indicates the number of frames after which a frame may be coded as an S-Frame
    pub sframe_dist: i32,
    /// Indicates how an S-Frame should be inserted
    /// values are from EbSFrameMode
    /// SFRAME_STRICT_ARF: the considered frame will be made into an S-Frame only if it is an altref frame
    /// SFRAME_NEAREST_ARF: if the considered frame is not an altref frame, the next altref frame will be made into an S-Frame
    pub sframe_mode: SFrameMode,
    /// API signal for the library to know the channel ID (used for pinning to cores).
    /// 
    /// Min value is 0.
    /// Max value is 0xFFFFFFFF.
    /// Default is 0.
    pub channel_id: u32,
    /// API signal for the library to know the active number of channels being encoded simultaneously.
    /// 
    /// Min value is 1.
    /// Max value is 0xFFFFFFFF.
    /// Default is 1.
    pub active_channel_count: u32,
    /// The level of parallelism refers to how much parallelization the encoder will perform
    /// by setting the number of threads and pictures that can be handled simultaneously. If
    /// the value is 0, a deafult level will be chosen based on the number of cores on the
    /// machine. Levels 1-6 are supported. Beyond that, higher inputs
    /// will map to the highest level.
    pub level_of_parallelism: u32,
    /// Pin the execution of threads to the first N logical processors.
    /// 0: unpinned
    /// N: Pin threads to socket's first N processors
    /// default 0
    pub pin_threads: u32,
    /// Target socket to run on. For dual socket systems, this can specify which
    /// socket the encoder runs on.
    /// 
    /// -1 = Both Sockets.
    ///  0 = Socket 0.
    ///  1 = Socket 1.
    /// 
    /// Default is -1.
    pub target_socket: i32,
    /// CPU FLAGS to limit assembly instruction set used by encoder.
    /// Default is EB_CPU_FLAGS_ALL.
    pub use_cpu_flags: CpuFlags,
    /// Instruct the library to calculate the recon to source for PSNR calculation
    /// 
    /// Default is 0.
    pub stat_report: u32,
    /// API Signal to output reconstructed yuv used for debug purposes.
    /// Using this will affect the speed of encoder.
    /// 
    /// Default is false.
    pub recon_enabled: bool,
    /// Signal that force-key-frames is enabled.
    ///
    pub force_key_frames: bool,
    /// Signal to the library to treat intra_period_length as seconds and
    /// multiply by fps_num/fps_den.
    pub multiply_keyint: bool,
    /// Reference scaling mode
    /// the available modes are defined in RESIZE_MODE
    pub resize_mode: u8,
    /// Resize denominator
    /// this value can be from 8 to 16, means downscaling to 8/8-8/16 of original
    /// resolution in both width and height
    pub resize_denom: u8,
    /// Resize denominator of key frames
    /// this value can be from 8 to 16, means downscaling to 8/8-8/16 of original
    /// resolution in both width and height
    pub resize_kf_denom: u8,
    /// Signal to the library to enable quantisation matrices
    /// 
    /// Default is true in SVT-AV1-PSY.
    pub enable_qm: bool,
    /// Min quant matrix flatness. Applicable when enable_qm is true.
    /// Min value is 0.
    /// Max value is 15.
    /// Default is 0 in SVT-AV1-PSY, mainline default is 8.
    pub min_qm_level: u8,
    /// Max quant matrix flatness. Applicable when enable_qm is true.
    /// Min value is 0.
    /// Max value is 15.
    /// Default is 15.
    pub max_qm_level: u8,
    /// gop_constraint_rc
    /// 
    /// Currently, only applicable for VBR and  when GoP size is greater than 119 frames.
    /// 
    /// When enabled, the rate control matches the target rate for each GoP.
    /// 
    /// 0: off
    /// 1: on
    /// Default is 0.
    pub gop_constraint_rc: bool,
    /// scale factors for lambda value for different frame update types
    /// factor >> 7 (/ 128) is the actual value in float
    pub lambda_scale_factors: [i32; 7usize],
    /// Dynamic gop
    /// 
    /// 0 = disable Dynamic GoP
    /// 1 = enable Dynamic GoP
    ///  Default is 1.
    pub enable_dg: bool,
    /// startup_mg_size
    /// 
    /// When enabled, a MG with specified size will be inserted after the key frame.
    /// The MG size is determined by 2^startup_mg_size.
    /// 
    /// 0: off
    /// 2: set hierarchical levels to 2 (MG size 4)
    /// 3: set hierarchical levels to 3 (MG size 8)
    /// 4: set hierarchical levels to 4 (MG size 16)
    /// Default is 0.
    pub startup_mg_size: u8,
    /// startup_qp_offset
    /// 
    /// When enabled, an offset will be added to the input-qp of the startup GOP prior to the picture-qp derivation
    /// 
    /// Min value is -63.
    /// Max value is 63.
    /// Default is 0.
    pub startup_qp_offset: i8,
    /// reference scaling events for random access mode (resize-mode = 4)
    /// 
    /// evt_num:          total count of events
    /// start_frame_nums: array of scaling start frame numbers
    /// resize_kf_denoms: array of scaling denominators of key-frame
    /// resize_denoms:    array of scaling denominators of non-key-frame
    pub frame_scale_evts: SvtAv1FrameScaleEvts,
    /// ROI map
    /// 
    /// 0 = disable ROI
    /// 1 = enable ROI
    ///  Default is 0.
    pub enable_roi_map: bool,
    /// Manually adjust temporal filtering strength
    /// 10 + (4 - 0) = 14 (8x weaker)
    /// 10 + (4 - 1) = 13 (4x weaker, PSY default)
    /// 10 + (4 - 2) = 12 (2x weaker)
    /// 10 + (4 - 3) = 11 (mainline default)
    /// 10 + (4 - 4) = 10 (2x stronger)
    pub tf_strength: u8,
    /// Stores the optional film grain synthesis info
    pub fgs_table: *mut AomFilmGrain,
    /// Variance boost
    /// false = disable variance boost
    /// true = enable variance boost
    /// Default is true in SVT-AV1-PSY.
    pub enable_variance_boost: bool,
    /// Selects the curve strength to boost low variance regions according to a fast-growing formula
    /// Default is 2.
    pub variance_boost_strength: u8,
    /// Picks a set of eight 8x8 variance values per superblock to determine boost
    /// Lower values enable detecting more blocks that need boosting, at the expense of more possible false positives (overall bitrate increase)
    ///  1: 1st octile
    ///  4: 4th octile
    ///  8: 8th octile
    ///  Default is 6
    pub variance_octile: u8,
    /// Bias towards decreased/increased sharpness in the deblocking loop filter & during rate distortion
    /// Minimum value is -7 (less sharp).
    /// Maximum value is 7 (more sharp).
    /// Default is 1 in svt-av1-psy (medium sharpness).
    pub sharpness: i8,
    /// Enable the user to configure which curve variance boost uses.
    /// Curve 1 emphasizes boosting low-medium contrast regions at a modest bitrate increase over the default curve
    ///  0: default curve
    ///  1: low-medium contrast boost curve
    ///  2: still picture curve, tuned for SSIMULACRA2 performance on the CID22 Validation Set
    ///  Default is 0.
    pub variance_boost_curve: u8,
    /// Frame-level luminance-based QP bias to improve quality in low luma scenarios
    /// Works by adjusting frame-level QP based on average luminance across a frame
    ///  0: Disable luminance-based QP bias
    ///  1-100: Enable frame-level luminance-based QP bias. Higher values strengthen the bias
    ///  Default is 0 (disabled).
    pub luminance_qp_bias: u8,
    /// Signal to the library to enable losless coding
    /// 
    /// Default is false.
    pub lossless: bool,
    /// Signal to the library to enable still-picture coding
    /// 
    /// Default is false.
    pub avif: bool,
    /// Q index for extended CRF support
    /// Value is internally determined by CRF parameter value
    /// Default is 0 if CRF is an integer
    pub extended_crf_qindex_offset: u8,
    /// compresses the QP hierarchical layer scale to improve temporal video consistency
    /// 0.0: no compression, original SVT-AV1 scaling
    /// 0.0-8.0: enable compression, the higher the number the stronger the compression
    ///         (different frame quality fluctuation/mean quality tradeoffs)
    /// Default is 1.0
    pub qp_scale_compress_strength: f64,
    /// Limit transform sizes to a maximum of 32x32 pixels
    /// 0: disabled, use transform sizes up to 64x64 pixels
    /// 1: enabled, use transform sizes up to 32x32 pixels
    /// Default is 0
    pub max_32_tx_size: bool,
    /// Min quant matrix flatness. Applicable when enable_qm is true.
    /// Min value is 0.
    /// Max value is 15.
    /// Default is 8.
    pub min_chroma_qm_level: u8,
    /// Max quant matrix flatness. Applicable when enable_qm is true.
    /// Min value is 0.
    /// Max value is 15.
    /// Default is 15.
    pub max_chroma_qm_level: u8,
    /// Noise normalization strength; modifies the encoder's willingness
    /// to boost AC coefficients in low-noise blocks.
    /// Min value is 0.
    /// Max value is 4.
    /// Default is 3.
    pub noise_norm_strength: u8,
    /// Manually adjust TF strength on keyframes
    /// 0: disable alt-ref TF on keyframes
    /// 1: 10 + (4 - 1) = 13 (4x weaker, PSY default)
    /// 2: 10 + (4 - 2) = 12 (2x weaker)
    /// 3: 10 + (4 - 3) = 11 (mainline default)
    /// 4: 10 + (4 - 4) = 10 (2x stronger) */
    pub kf_tf_strength: u8,
    /// Enable psychovisual rate distortion
    /// 0.00: disable PSY-RD
    /// 6.00: enable PSY-RD with a strength of 4.00
    /// Default is 1.0.
    pub psy_rd: f64,
    /// Enable spy-rd, an alternate RD metric that biases towards sharpness/detail retention,
    /// at the possible expense of increased blocking and banding
    /// 0: disabled
    /// 1: full
    /// 2: partial (interpolation filter tweaks only)
    /// Default is 0
    pub spy_rd: u8,
    /// Prevent macroblocks from being boosted to very low q.
    /// 
    /// Default is 0. 0 = off, 1 = on.
    pub low_q_taper: bool,
    /// Enable sharp-tx, a toggle that enables much sharper transforms decisions for higher fidelity ouput,
    /// at the possible cost of increasing artifacting
    /// 0: disabled
    /// 1: enabled
    /// Default is 1
    pub sharp_tx: u8,
    /// High Bit-Depth Mode Decision, used to control the bit-depth of the mode decision path.
    /// 0: default behavior
    /// 1: full 10-bit MD
    /// 2: hybrid 8/10-bit MD
    /// 3: full 8-bit MD
    /// Default is 0
    pub hbd_mds: u8,
    /// Enable complex-hvs, a feature that enables the highest complexity and highest fidelity
    /// HVS model at the cost of higher CPU time
    /// 0: default preset behavior
    /// 1: highest complexity HVS model (SSD-Psy)
    /// Default is 0
    pub complex_hvs: u8,
    /// Toggle default film grain blocksize behavior
    /// 0: use default blocksize behavior (32x32)
    /// 1: use adaptive blocksize based on resolution
    ///  - 8x8 for <4k
    ///  - 16x16 for 4k
    /// Default is 1
    pub adaptive_film_grain: bool,
    /// Controls noise detection for CDEF/restoration filtering
    /// 0: default tune behavior
    /// 1: on
    /// 2: off
    /// 2: on (CDEF only)
    /// 3: on (restoration only)
    /// Default is 0
    pub filtering_noise_detection: u8,
    /// Add 128 Byte Padding to Struct to avoid changing the size of the public configuration struct
    pub padding: [u8; 128 - 3 * size_of::<bool>() - 10 * size_of::<u8>() - 2 * size_of::<f64>()],
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
    /// Returns a string containing "v$tag-$commit_count-g$hash${dirty:+-dirty}"
    pub fn svt_av1_get_version() -> *const ::std::os::raw::c_char;

    /// Returns a string containing only the SVT-AV1-PSY micro-release letter
    pub fn svt_psy_get_version() -> *const ::std::os::raw::c_char;

    /// Prints the version header and build information to the file
    /// specified by the SVT_LOG_FILE environment variable or stderr
    pub fn svt_av1_print_version();

    /// STEP 1: Call the library to construct a Component Handle.
    pub fn svt_av1_enc_init_handle(
        p_handle: *mut *mut ComponentType,
        config_ptr: *mut SvtAv1EncConfiguration,
    ) -> ErrorType;

    /// STEP 2: Set all configuration parameters.
    pub fn svt_av1_enc_set_parameter(
        svt_enc_component: *mut ComponentType,
        pComponentParameterStructure: *mut SvtAv1EncConfiguration,
    ) -> ErrorType;

    /// OPTIONAL: Set a single configuration parameter.
    pub fn svt_av1_enc_parse_parameter(
        pComponentParameterStructure: *mut SvtAv1EncConfiguration,
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> ErrorType;

    /// STEP 3: Initialize encoder and allocates memory to necessary buffers.
    pub fn svt_av1_enc_init(svt_enc_component: *mut ComponentType) -> ErrorType;

    /// OPTIONAL: Get stream headers at init time.
    pub fn svt_av1_enc_stream_header(
        svt_enc_component: *mut ComponentType,
        output_stream_ptr: *mut *mut BufferHeaderType,
    ) -> ErrorType;

    /// OPTIONAL: Release stream headers at init time.
    pub fn svt_av1_enc_stream_header_release(stream_header_ptr: *mut BufferHeaderType)
        -> ErrorType;

    /// STEP 4: Send the picture.
    pub fn svt_av1_enc_send_picture(
        svt_enc_component: *mut ComponentType,
        p_buffer: *mut BufferHeaderType,
    ) -> ErrorType;

    /// Step 5: Receive packet.
    /// This function will become blocking if either pic_send_done is set to 1 or if we are in low-delay (pred-struct=1).
    /// Otherwise, this function is non-blocking and will return EB_NoErrorEmptyQueue if there are no packets available.
    pub fn svt_av1_enc_get_packet(
        svt_enc_component: *mut ComponentType,
        p_buffer: *mut *mut BufferHeaderType,
        pic_send_done: u8,
    ) -> ErrorType;

    /// STEP 5-1: Release output buffer back into the pool.
    pub fn svt_av1_enc_release_out_buffer(p_buffer: *mut *mut BufferHeaderType);

    /// OPTIONAL: Fill buffer with reconstructed picture.
    pub fn svt_av1_get_recon(
        svt_enc_component: *mut ComponentType,
        p_buffer: *mut BufferHeaderType,
    ) -> ErrorType;

    /// OPTIONAL: get stream information
    pub fn svt_av1_enc_get_stream_info(
        svt_enc_component: *mut ComponentType,
        stream_info_id: u32,
        info: *mut ::std::os::raw::c_void,
    ) -> ErrorType;

    /// STEP 6: Deinitialize encoder library.
    pub fn svt_av1_enc_deinit(svt_enc_component: *mut ComponentType) -> ErrorType;

    /// STEP 7: Deconstruct encoder handler.
    pub fn svt_av1_enc_deinit_handle(svt_enc_component: *mut ComponentType) -> ErrorType;
}
