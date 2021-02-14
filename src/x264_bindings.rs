extern crate std;

pub unsafe fn x264_encoder_open(params: *mut x264_param_t) -> *mut x264_t {
                               x264_encoder_open_161(params)
                          }

pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __STDC_NO_THREADS__: u32 = 1;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 27;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const __GNUC_VA_LIST: u32 = 1;
pub const X264_GPL: u32 = 1;
pub const X264_INTERLACED: u32 = 1;
pub const X264_BIT_DEPTH: u32 = 0;
pub const X264_CHROMA_FORMAT: u32 = 0;
pub const X264_VERSION: &'static [u8; 1usize] = b"\0";
pub const X264_POINTVER: &'static [u8; 8usize] = b"0.161.x\0";
pub const X264_BUILD: u32 = 161;
pub const X264_CPU_MMX: u32 = 1;
pub const X264_CPU_MMX2: u32 = 2;
pub const X264_CPU_MMXEXT: u32 = 2;
pub const X264_CPU_SSE: u32 = 4;
pub const X264_CPU_SSE2: u32 = 8;
pub const X264_CPU_LZCNT: u32 = 16;
pub const X264_CPU_SSE3: u32 = 32;
pub const X264_CPU_SSSE3: u32 = 64;
pub const X264_CPU_SSE4: u32 = 128;
pub const X264_CPU_SSE42: u32 = 256;
pub const X264_CPU_AVX: u32 = 512;
pub const X264_CPU_XOP: u32 = 1024;
pub const X264_CPU_FMA4: u32 = 2048;
pub const X264_CPU_FMA3: u32 = 4096;
pub const X264_CPU_BMI1: u32 = 8192;
pub const X264_CPU_BMI2: u32 = 16384;
pub const X264_CPU_AVX2: u32 = 32768;
pub const X264_CPU_AVX512: u32 = 65536;
pub const X264_CPU_CACHELINE_32: u32 = 131072;
pub const X264_CPU_CACHELINE_64: u32 = 262144;
pub const X264_CPU_SSE2_IS_SLOW: u32 = 524288;
pub const X264_CPU_SSE2_IS_FAST: u32 = 1048576;
pub const X264_CPU_SLOW_SHUFFLE: u32 = 2097152;
pub const X264_CPU_STACK_MOD4: u32 = 4194304;
pub const X264_CPU_SLOW_ATOM: u32 = 8388608;
pub const X264_CPU_SLOW_PSHUFB: u32 = 16777216;
pub const X264_CPU_SLOW_PALIGNR: u32 = 33554432;
pub const X264_CPU_ALTIVEC: u32 = 1;
pub const X264_CPU_ARMV6: u32 = 1;
pub const X264_CPU_NEON: u32 = 2;
pub const X264_CPU_FAST_NEON_MRC: u32 = 4;
pub const X264_CPU_ARMV8: u32 = 8;
pub const X264_CPU_MSA: u32 = 1;
pub const X264_ANALYSE_I4x4: u32 = 1;
pub const X264_ANALYSE_I8x8: u32 = 2;
pub const X264_ANALYSE_PSUB16x16: u32 = 16;
pub const X264_ANALYSE_PSUB8x8: u32 = 32;
pub const X264_ANALYSE_BSUB16x16: u32 = 256;
pub const X264_DIRECT_PRED_NONE: u32 = 0;
pub const X264_DIRECT_PRED_SPATIAL: u32 = 1;
pub const X264_DIRECT_PRED_TEMPORAL: u32 = 2;
pub const X264_DIRECT_PRED_AUTO: u32 = 3;
pub const X264_ME_DIA: u32 = 0;
pub const X264_ME_HEX: u32 = 1;
pub const X264_ME_UMH: u32 = 2;
pub const X264_ME_ESA: u32 = 3;
pub const X264_ME_TESA: u32 = 4;
pub const X264_CQM_FLAT: u32 = 0;
pub const X264_CQM_JVT: u32 = 1;
pub const X264_CQM_CUSTOM: u32 = 2;
pub const X264_RC_CQP: u32 = 0;
pub const X264_RC_CRF: u32 = 1;
pub const X264_RC_ABR: u32 = 2;
pub const X264_QP_AUTO: u32 = 0;
pub const X264_AQ_NONE: u32 = 0;
pub const X264_AQ_VARIANCE: u32 = 1;
pub const X264_AQ_AUTOVARIANCE: u32 = 2;
pub const X264_AQ_AUTOVARIANCE_BIASED: u32 = 3;
pub const X264_B_ADAPT_NONE: u32 = 0;
pub const X264_B_ADAPT_FAST: u32 = 1;
pub const X264_B_ADAPT_TRELLIS: u32 = 2;
pub const X264_WEIGHTP_NONE: u32 = 0;
pub const X264_WEIGHTP_SIMPLE: u32 = 1;
pub const X264_WEIGHTP_SMART: u32 = 2;
pub const X264_B_PYRAMID_NONE: u32 = 0;
pub const X264_B_PYRAMID_STRICT: u32 = 1;
pub const X264_B_PYRAMID_NORMAL: u32 = 2;
pub const X264_KEYINT_MIN_AUTO: u32 = 0;
pub const X264_KEYINT_MAX_INFINITE: u32 = 1073741824;
pub const X264_AVCINTRA_FLAVOR_PANASONIC: u32 = 0;
pub const X264_AVCINTRA_FLAVOR_SONY: u32 = 1;
pub const X264_CSP_MASK: u32 = 255;
pub const X264_CSP_NONE: u32 = 0;
pub const X264_CSP_I400: u32 = 1;
pub const X264_CSP_I420: u32 = 2;
pub const X264_CSP_YV12: u32 = 3;
pub const X264_CSP_NV12: u32 = 4;
pub const X264_CSP_NV21: u32 = 5;
pub const X264_CSP_I422: u32 = 6;
pub const X264_CSP_YV16: u32 = 7;
pub const X264_CSP_NV16: u32 = 8;
pub const X264_CSP_YUYV: u32 = 9;
pub const X264_CSP_UYVY: u32 = 10;
pub const X264_CSP_V210: u32 = 11;
pub const X264_CSP_I444: u32 = 12;
pub const X264_CSP_YV24: u32 = 13;
pub const X264_CSP_BGR: u32 = 14;
pub const X264_CSP_BGRA: u32 = 15;
pub const X264_CSP_RGB: u32 = 16;
pub const X264_CSP_MAX: u32 = 17;
pub const X264_CSP_VFLIP: u32 = 4096;
pub const X264_CSP_HIGH_DEPTH: u32 = 8192;
pub const X264_TYPE_AUTO: u32 = 0;
pub const X264_TYPE_IDR: u32 = 1;
pub const X264_TYPE_I: u32 = 2;
pub const X264_TYPE_P: u32 = 3;
pub const X264_TYPE_BREF: u32 = 4;
pub const X264_TYPE_B: u32 = 5;
pub const X264_TYPE_KEYFRAME: u32 = 6;
pub const X264_LOG_NONE: i32 = -1;
pub const X264_LOG_ERROR: u32 = 0;
pub const X264_LOG_WARNING: u32 = 1;
pub const X264_LOG_INFO: u32 = 2;
pub const X264_LOG_DEBUG: u32 = 3;
pub const X264_THREADS_AUTO: u32 = 0;
pub const X264_SYNC_LOOKAHEAD_AUTO: i32 = -1;
pub const X264_NAL_HRD_NONE: u32 = 0;
pub const X264_NAL_HRD_VBR: u32 = 1;
pub const X264_NAL_HRD_CBR: u32 = 2;
pub const X264_PARAM_BAD_NAME: i32 = -1;
pub const X264_PARAM_BAD_VALUE: i32 = -2;
pub const X264_PARAM_ALLOC_FAILED: i32 = -3;
pub const X264_MBINFO_CONSTANT: u32 = 1;
pub type __u_char = std::os::raw::c_uchar;
pub type __u_short = std::os::raw::c_ushort;
pub type __u_int = std::os::raw::c_uint;
pub type __u_long = std::os::raw::c_ulong;
pub type __int8_t = std::os::raw::c_schar;
pub type __uint8_t = std::os::raw::c_uchar;
pub type __int16_t = std::os::raw::c_short;
pub type __uint16_t = std::os::raw::c_ushort;
pub type __int32_t = std::os::raw::c_int;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_long;
pub type __uint64_t = std::os::raw::c_ulong;
pub type __quad_t = std::os::raw::c_long;
pub type __u_quad_t = std::os::raw::c_ulong;
pub type __intmax_t = std::os::raw::c_long;
pub type __uintmax_t = std::os::raw::c_ulong;
pub type __dev_t = std::os::raw::c_ulong;
pub type __uid_t = std::os::raw::c_uint;
pub type __gid_t = std::os::raw::c_uint;
pub type __ino_t = std::os::raw::c_ulong;
pub type __ino64_t = std::os::raw::c_ulong;
pub type __mode_t = std::os::raw::c_uint;
pub type __nlink_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __pid_t = std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(
        std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__fsid_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = std::os::raw::c_long;
pub type __rlim_t = std::os::raw::c_ulong;
pub type __rlim64_t = std::os::raw::c_ulong;
pub type __id_t = std::os::raw::c_uint;
pub type __time_t = std::os::raw::c_long;
pub type __useconds_t = std::os::raw::c_uint;
pub type __suseconds_t = std::os::raw::c_long;
pub type __daddr_t = std::os::raw::c_int;
pub type __key_t = std::os::raw::c_int;
pub type __clockid_t = std::os::raw::c_int;
pub type __timer_t = *mut std::os::raw::c_void;
pub type __blksize_t = std::os::raw::c_long;
pub type __blkcnt_t = std::os::raw::c_long;
pub type __blkcnt64_t = std::os::raw::c_long;
pub type __fsblkcnt_t = std::os::raw::c_ulong;
pub type __fsblkcnt64_t = std::os::raw::c_ulong;
pub type __fsfilcnt_t = std::os::raw::c_ulong;
pub type __fsfilcnt64_t = std::os::raw::c_ulong;
pub type __fsword_t = std::os::raw::c_long;
pub type __ssize_t = std::os::raw::c_long;
pub type __syscall_slong_t = std::os::raw::c_long;
pub type __syscall_ulong_t = std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut std::os::raw::c_char;
pub type __intptr_t = std::os::raw::c_long;
pub type __socklen_t = std::os::raw::c_uint;
pub type __sig_atomic_t = std::os::raw::c_int;
pub type int_least8_t = std::os::raw::c_schar;
pub type int_least16_t = std::os::raw::c_short;
pub type int_least32_t = std::os::raw::c_int;
pub type int_least64_t = std::os::raw::c_long;
pub type uint_least8_t = std::os::raw::c_uchar;
pub type uint_least16_t = std::os::raw::c_ushort;
pub type uint_least32_t = std::os::raw::c_uint;
pub type uint_least64_t = std::os::raw::c_ulong;
pub type int_fast8_t = std::os::raw::c_schar;
pub type int_fast16_t = std::os::raw::c_long;
pub type int_fast32_t = std::os::raw::c_long;
pub type int_fast64_t = std::os::raw::c_long;
pub type uint_fast8_t = std::os::raw::c_uchar;
pub type uint_fast16_t = std::os::raw::c_ulong;
pub type uint_fast32_t = std::os::raw::c_ulong;
pub type uint_fast64_t = std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_t {
    _unused: [u8; 0],
}
pub const nal_unit_type_e_NAL_UNKNOWN: nal_unit_type_e = 0;
pub const nal_unit_type_e_NAL_SLICE: nal_unit_type_e = 1;
pub const nal_unit_type_e_NAL_SLICE_DPA: nal_unit_type_e = 2;
pub const nal_unit_type_e_NAL_SLICE_DPB: nal_unit_type_e = 3;
pub const nal_unit_type_e_NAL_SLICE_DPC: nal_unit_type_e = 4;
pub const nal_unit_type_e_NAL_SLICE_IDR: nal_unit_type_e = 5;
pub const nal_unit_type_e_NAL_SEI: nal_unit_type_e = 6;
pub const nal_unit_type_e_NAL_SPS: nal_unit_type_e = 7;
pub const nal_unit_type_e_NAL_PPS: nal_unit_type_e = 8;
pub const nal_unit_type_e_NAL_AUD: nal_unit_type_e = 9;
pub const nal_unit_type_e_NAL_FILLER: nal_unit_type_e = 12;
#[doc = " NAL structure and functions"]
pub type nal_unit_type_e = std::os::raw::c_uint;
pub const nal_priority_e_NAL_PRIORITY_DISPOSABLE: nal_priority_e = 0;
pub const nal_priority_e_NAL_PRIORITY_LOW: nal_priority_e = 1;
pub const nal_priority_e_NAL_PRIORITY_HIGH: nal_priority_e = 2;
pub const nal_priority_e_NAL_PRIORITY_HIGHEST: nal_priority_e = 3;
pub type nal_priority_e = std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_nal_t {
    pub i_ref_idc: std::os::raw::c_int,
    pub i_type: std::os::raw::c_int,
    pub b_long_startcode: std::os::raw::c_int,
    pub i_first_mb: std::os::raw::c_int,
    pub i_last_mb: std::os::raw::c_int,
    pub i_payload: std::os::raw::c_int,
    pub p_payload: *mut u8,
    pub i_padding: std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_x264_nal_t() {
    assert_eq!(
        std::mem::size_of::<x264_nal_t>(),
        40usize,
        concat!("Size of: ", stringify!(x264_nal_t))
    );
    assert_eq!(
        std::mem::align_of::<x264_nal_t>(),
        8usize,
        concat!("Alignment of ", stringify!(x264_nal_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_nal_t>())).i_ref_idc as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_nal_t),
            "::",
            stringify!(i_ref_idc)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_nal_t>())).i_type as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_nal_t),
            "::",
            stringify!(i_type)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_nal_t>())).b_long_startcode as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_nal_t),
            "::",
            stringify!(b_long_startcode)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_nal_t>())).i_first_mb as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_nal_t),
            "::",
            stringify!(i_first_mb)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_nal_t>())).i_last_mb as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_nal_t),
            "::",
            stringify!(i_last_mb)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_nal_t>())).i_payload as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_nal_t),
            "::",
            stringify!(i_payload)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_nal_t>())).p_payload as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_nal_t),
            "::",
            stringify!(p_payload)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_nal_t>())).i_padding as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_nal_t),
            "::",
            stringify!(i_padding)
        )
    );
}
extern "C" {
    pub static x264_direct_pred_names: [*const std::os::raw::c_char; 5usize];
}
extern "C" {
    pub static x264_motion_est_names: [*const std::os::raw::c_char; 6usize];
}
extern "C" {
    pub static x264_b_pyramid_names: [*const std::os::raw::c_char; 4usize];
}
extern "C" {
    pub static x264_overscan_names: [*const std::os::raw::c_char; 4usize];
}
extern "C" {
    pub static x264_vidformat_names: [*const std::os::raw::c_char; 7usize];
}
extern "C" {
    pub static x264_fullrange_names: [*const std::os::raw::c_char; 3usize];
}
extern "C" {
    pub static x264_colorprim_names: [*const std::os::raw::c_char; 14usize];
}
extern "C" {
    pub static x264_transfer_names: [*const std::os::raw::c_char; 20usize];
}
extern "C" {
    pub static x264_colmatrix_names: [*const std::os::raw::c_char; 16usize];
}
extern "C" {
    pub static x264_nal_hrd_names: [*const std::os::raw::c_char; 4usize];
}
extern "C" {
    pub static x264_avcintra_flavor_names: [*const std::os::raw::c_char; 3usize];
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_zone_t {
    pub i_start: std::os::raw::c_int,
    pub i_end: std::os::raw::c_int,
    pub b_force_qp: std::os::raw::c_int,
    pub i_qp: std::os::raw::c_int,
    pub f_bitrate_factor: f32,
    pub param: *mut x264_param_t,
}
#[test]
fn bindgen_test_layout_x264_zone_t() {
    assert_eq!(
        std::mem::size_of::<x264_zone_t>(),
        32usize,
        concat!("Size of: ", stringify!(x264_zone_t))
    );
    assert_eq!(
        std::mem::align_of::<x264_zone_t>(),
        8usize,
        concat!("Alignment of ", stringify!(x264_zone_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_zone_t>())).i_start as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_zone_t),
            "::",
            stringify!(i_start)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_zone_t>())).i_end as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_zone_t),
            "::",
            stringify!(i_end)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_zone_t>())).b_force_qp as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_zone_t),
            "::",
            stringify!(b_force_qp)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_zone_t>())).i_qp as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_zone_t),
            "::",
            stringify!(i_qp)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_zone_t>())).f_bitrate_factor as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_zone_t),
            "::",
            stringify!(f_bitrate_factor)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_zone_t>())).param as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_zone_t),
            "::",
            stringify!(param)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x264_param_t {
    pub cpu: u32,
    pub i_threads: std::os::raw::c_int,
    pub i_lookahead_threads: std::os::raw::c_int,
    pub b_sliced_threads: std::os::raw::c_int,
    pub b_deterministic: std::os::raw::c_int,
    pub b_cpu_independent: std::os::raw::c_int,
    pub i_sync_lookahead: std::os::raw::c_int,
    pub i_width: std::os::raw::c_int,
    pub i_height: std::os::raw::c_int,
    pub i_csp: std::os::raw::c_int,
    pub i_bitdepth: std::os::raw::c_int,
    pub i_level_idc: std::os::raw::c_int,
    pub i_frame_total: std::os::raw::c_int,
    pub i_nal_hrd: std::os::raw::c_int,
    pub vui: x264_param_t__bindgen_ty_1,
    pub i_frame_reference: std::os::raw::c_int,
    pub i_dpb_size: std::os::raw::c_int,
    pub i_keyint_max: std::os::raw::c_int,
    pub i_keyint_min: std::os::raw::c_int,
    pub i_scenecut_threshold: std::os::raw::c_int,
    pub b_intra_refresh: std::os::raw::c_int,
    pub i_bframe: std::os::raw::c_int,
    pub i_bframe_adaptive: std::os::raw::c_int,
    pub i_bframe_bias: std::os::raw::c_int,
    pub i_bframe_pyramid: std::os::raw::c_int,
    pub b_open_gop: std::os::raw::c_int,
    pub b_bluray_compat: std::os::raw::c_int,
    pub i_avcintra_class: std::os::raw::c_int,
    pub i_avcintra_flavor: std::os::raw::c_int,
    pub b_deblocking_filter: std::os::raw::c_int,
    pub i_deblocking_filter_alphac0: std::os::raw::c_int,
    pub i_deblocking_filter_beta: std::os::raw::c_int,
    pub b_cabac: std::os::raw::c_int,
    pub i_cabac_init_idc: std::os::raw::c_int,
    pub b_interlaced: std::os::raw::c_int,
    pub b_constrained_intra: std::os::raw::c_int,
    pub i_cqm_preset: std::os::raw::c_int,
    pub psz_cqm_file: *mut std::os::raw::c_char,
    pub cqm_4iy: [u8; 16usize],
    pub cqm_4py: [u8; 16usize],
    pub cqm_4ic: [u8; 16usize],
    pub cqm_4pc: [u8; 16usize],
    pub cqm_8iy: [u8; 64usize],
    pub cqm_8py: [u8; 64usize],
    pub cqm_8ic: [u8; 64usize],
    pub cqm_8pc: [u8; 64usize],
    pub pf_log: std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut std::os::raw::c_void,
            i_level: std::os::raw::c_int,
            psz: *const std::os::raw::c_char,
            arg2: *mut __va_list_tag,
        ),
    >,
    pub p_log_private: *mut std::os::raw::c_void,
    pub i_log_level: std::os::raw::c_int,
    pub b_full_recon: std::os::raw::c_int,
    pub psz_dump_yuv: *mut std::os::raw::c_char,
    pub analyse: x264_param_t__bindgen_ty_2,
    pub rc: x264_param_t__bindgen_ty_3,
    pub crop_rect: x264_param_t__bindgen_ty_4,
    pub i_frame_packing: std::os::raw::c_int,
    pub i_alternative_transfer: std::os::raw::c_int,
    pub b_aud: std::os::raw::c_int,
    pub b_repeat_headers: std::os::raw::c_int,
    pub b_annexb: std::os::raw::c_int,
    pub i_sps_id: std::os::raw::c_int,
    pub b_vfr_input: std::os::raw::c_int,
    pub b_pulldown: std::os::raw::c_int,
    pub i_fps_num: u32,
    pub i_fps_den: u32,
    pub i_timebase_num: u32,
    pub i_timebase_den: u32,
    pub b_tff: std::os::raw::c_int,
    pub b_pic_struct: std::os::raw::c_int,
    pub b_fake_interlaced: std::os::raw::c_int,
    pub b_stitchable: std::os::raw::c_int,
    pub b_opencl: std::os::raw::c_int,
    pub i_opencl_device: std::os::raw::c_int,
    pub opencl_device_id: *mut std::os::raw::c_void,
    pub psz_clbin_file: *mut std::os::raw::c_char,
    pub i_slice_max_size: std::os::raw::c_int,
    pub i_slice_max_mbs: std::os::raw::c_int,
    pub i_slice_min_mbs: std::os::raw::c_int,
    pub i_slice_count: std::os::raw::c_int,
    pub i_slice_count_max: std::os::raw::c_int,
    pub param_free: std::option::Option<unsafe extern "C" fn(arg1: *mut std::os::raw::c_void)>,
    pub nalu_process: std::option::Option<
        unsafe extern "C" fn(
            h: *mut x264_t,
            nal: *mut x264_nal_t,
            opaque: *mut std::os::raw::c_void,
        ),
    >,
    pub opaque: *mut std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_param_t__bindgen_ty_1 {
    pub i_sar_height: std::os::raw::c_int,
    pub i_sar_width: std::os::raw::c_int,
    pub i_overscan: std::os::raw::c_int,
    pub i_vidformat: std::os::raw::c_int,
    pub b_fullrange: std::os::raw::c_int,
    pub i_colorprim: std::os::raw::c_int,
    pub i_transfer: std::os::raw::c_int,
    pub i_colmatrix: std::os::raw::c_int,
    pub i_chroma_loc: std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_x264_param_t__bindgen_ty_1() {
    assert_eq!(
        std::mem::size_of::<x264_param_t__bindgen_ty_1>(),
        36usize,
        concat!("Size of: ", stringify!(x264_param_t__bindgen_ty_1))
    );
    assert_eq!(
        std::mem::align_of::<x264_param_t__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(x264_param_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_1>())).i_sar_height as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_1),
            "::",
            stringify!(i_sar_height)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_1>())).i_sar_width as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_1),
            "::",
            stringify!(i_sar_width)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_1>())).i_overscan as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_1),
            "::",
            stringify!(i_overscan)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_1>())).i_vidformat as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_1),
            "::",
            stringify!(i_vidformat)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_1>())).b_fullrange as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_1),
            "::",
            stringify!(b_fullrange)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_1>())).i_colorprim as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_1),
            "::",
            stringify!(i_colorprim)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_1>())).i_transfer as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_1),
            "::",
            stringify!(i_transfer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_1>())).i_colmatrix as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_1),
            "::",
            stringify!(i_colmatrix)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_1>())).i_chroma_loc as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_1),
            "::",
            stringify!(i_chroma_loc)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_param_t__bindgen_ty_2 {
    pub intra: std::os::raw::c_uint,
    pub inter: std::os::raw::c_uint,
    pub b_transform_8x8: std::os::raw::c_int,
    pub i_weighted_pred: std::os::raw::c_int,
    pub b_weighted_bipred: std::os::raw::c_int,
    pub i_direct_mv_pred: std::os::raw::c_int,
    pub i_chroma_qp_offset: std::os::raw::c_int,
    pub i_me_method: std::os::raw::c_int,
    pub i_me_range: std::os::raw::c_int,
    pub i_mv_range: std::os::raw::c_int,
    pub i_mv_range_thread: std::os::raw::c_int,
    pub i_subpel_refine: std::os::raw::c_int,
    pub b_chroma_me: std::os::raw::c_int,
    pub b_mixed_references: std::os::raw::c_int,
    pub i_trellis: std::os::raw::c_int,
    pub b_fast_pskip: std::os::raw::c_int,
    pub b_dct_decimate: std::os::raw::c_int,
    pub i_noise_reduction: std::os::raw::c_int,
    pub f_psy_rd: f32,
    pub f_psy_trellis: f32,
    pub b_psy: std::os::raw::c_int,
    pub b_mb_info: std::os::raw::c_int,
    pub b_mb_info_update: std::os::raw::c_int,
    pub i_luma_deadzone: [std::os::raw::c_int; 2usize],
    pub b_psnr: std::os::raw::c_int,
    pub b_ssim: std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_x264_param_t__bindgen_ty_2() {
    assert_eq!(
        std::mem::size_of::<x264_param_t__bindgen_ty_2>(),
        108usize,
        concat!("Size of: ", stringify!(x264_param_t__bindgen_ty_2))
    );
    assert_eq!(
        std::mem::align_of::<x264_param_t__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(x264_param_t__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).intra as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(intra)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).inter as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(inter)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).b_transform_8x8 as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(b_transform_8x8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).i_weighted_pred as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(i_weighted_pred)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).b_weighted_bipred as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(b_weighted_bipred)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).i_direct_mv_pred as *const _
                as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(i_direct_mv_pred)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).i_chroma_qp_offset as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(i_chroma_qp_offset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).i_me_method as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(i_me_method)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).i_me_range as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(i_me_range)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).i_mv_range as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(i_mv_range)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).i_mv_range_thread as *const _
                as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(i_mv_range_thread)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).i_subpel_refine as *const _
                as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(i_subpel_refine)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).b_chroma_me as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(b_chroma_me)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).b_mixed_references as *const _
                as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(b_mixed_references)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).i_trellis as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(i_trellis)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).b_fast_pskip as *const _ as usize
        },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(b_fast_pskip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).b_dct_decimate as *const _
                as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(b_dct_decimate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).i_noise_reduction as *const _
                as usize
        },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(i_noise_reduction)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).f_psy_rd as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(f_psy_rd)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).f_psy_trellis as *const _
                as usize
        },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(f_psy_trellis)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).b_psy as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(b_psy)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).b_mb_info as *const _ as usize
        },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(b_mb_info)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).b_mb_info_update as *const _
                as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(b_mb_info_update)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).i_luma_deadzone as *const _
                as usize
        },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(i_luma_deadzone)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).b_psnr as *const _ as usize
        },
        100usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(b_psnr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_2>())).b_ssim as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_2),
            "::",
            stringify!(b_ssim)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_param_t__bindgen_ty_3 {
    pub i_rc_method: std::os::raw::c_int,
    pub i_qp_constant: std::os::raw::c_int,
    pub i_qp_min: std::os::raw::c_int,
    pub i_qp_max: std::os::raw::c_int,
    pub i_qp_step: std::os::raw::c_int,
    pub i_bitrate: std::os::raw::c_int,
    pub f_rf_constant: f32,
    pub f_rf_constant_max: f32,
    pub f_rate_tolerance: f32,
    pub i_vbv_max_bitrate: std::os::raw::c_int,
    pub i_vbv_buffer_size: std::os::raw::c_int,
    pub f_vbv_buffer_init: f32,
    pub f_ip_factor: f32,
    pub f_pb_factor: f32,
    pub b_filler: std::os::raw::c_int,
    pub i_aq_mode: std::os::raw::c_int,
    pub f_aq_strength: f32,
    pub b_mb_tree: std::os::raw::c_int,
    pub i_lookahead: std::os::raw::c_int,
    pub b_stat_write: std::os::raw::c_int,
    pub psz_stat_out: *mut std::os::raw::c_char,
    pub b_stat_read: std::os::raw::c_int,
    pub psz_stat_in: *mut std::os::raw::c_char,
    pub f_qcompress: f32,
    pub f_qblur: f32,
    pub f_complexity_blur: f32,
    pub zones: *mut x264_zone_t,
    pub i_zones: std::os::raw::c_int,
    pub psz_zones: *mut std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_x264_param_t__bindgen_ty_3() {
    assert_eq!(
        std::mem::size_of::<x264_param_t__bindgen_ty_3>(),
        144usize,
        concat!("Size of: ", stringify!(x264_param_t__bindgen_ty_3))
    );
    assert_eq!(
        std::mem::align_of::<x264_param_t__bindgen_ty_3>(),
        8usize,
        concat!("Alignment of ", stringify!(x264_param_t__bindgen_ty_3))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).i_rc_method as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(i_rc_method)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).i_qp_constant as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(i_qp_constant)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).i_qp_min as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(i_qp_min)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).i_qp_max as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(i_qp_max)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).i_qp_step as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(i_qp_step)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).i_bitrate as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(i_bitrate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).f_rf_constant as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(f_rf_constant)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).f_rf_constant_max as *const _
                as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(f_rf_constant_max)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).f_rate_tolerance as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(f_rate_tolerance)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).i_vbv_max_bitrate as *const _
                as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(i_vbv_max_bitrate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).i_vbv_buffer_size as *const _
                as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(i_vbv_buffer_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).f_vbv_buffer_init as *const _
                as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(f_vbv_buffer_init)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).f_ip_factor as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(f_ip_factor)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).f_pb_factor as *const _ as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(f_pb_factor)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).b_filler as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(b_filler)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).i_aq_mode as *const _ as usize
        },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(i_aq_mode)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).f_aq_strength as *const _
                as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(f_aq_strength)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).b_mb_tree as *const _ as usize
        },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(b_mb_tree)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).i_lookahead as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(i_lookahead)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).b_stat_write as *const _ as usize
        },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(b_stat_write)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).psz_stat_out as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(psz_stat_out)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).b_stat_read as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(b_stat_read)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).psz_stat_in as *const _ as usize
        },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(psz_stat_in)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).f_qcompress as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(f_qcompress)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).f_qblur as *const _ as usize
        },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(f_qblur)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).f_complexity_blur as *const _
                as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(f_complexity_blur)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).zones as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(zones)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).i_zones as *const _ as usize
        },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(i_zones)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_3>())).psz_zones as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_3),
            "::",
            stringify!(psz_zones)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_param_t__bindgen_ty_4 {
    pub i_left: std::os::raw::c_int,
    pub i_top: std::os::raw::c_int,
    pub i_right: std::os::raw::c_int,
    pub i_bottom: std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_x264_param_t__bindgen_ty_4() {
    assert_eq!(
        std::mem::size_of::<x264_param_t__bindgen_ty_4>(),
        16usize,
        concat!("Size of: ", stringify!(x264_param_t__bindgen_ty_4))
    );
    assert_eq!(
        std::mem::align_of::<x264_param_t__bindgen_ty_4>(),
        4usize,
        concat!("Alignment of ", stringify!(x264_param_t__bindgen_ty_4))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_4>())).i_left as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_4),
            "::",
            stringify!(i_left)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_4>())).i_top as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_4),
            "::",
            stringify!(i_top)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_4>())).i_right as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_4),
            "::",
            stringify!(i_right)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t__bindgen_ty_4>())).i_bottom as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t__bindgen_ty_4),
            "::",
            stringify!(i_bottom)
        )
    );
}
#[test]
fn bindgen_test_layout_x264_param_t() {
    assert_eq!(
        std::mem::size_of::<x264_param_t>(),
        952usize,
        concat!("Size of: ", stringify!(x264_param_t))
    );
    assert_eq!(
        std::mem::align_of::<x264_param_t>(),
        8usize,
        concat!("Alignment of ", stringify!(x264_param_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).cpu as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(cpu)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_threads as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_threads)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t>())).i_lookahead_threads as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_lookahead_threads)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_sliced_threads as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_sliced_threads)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_deterministic as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_deterministic)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_cpu_independent as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_cpu_independent)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_sync_lookahead as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_sync_lookahead)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_width as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_width)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_height as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_height)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_csp as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_csp)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_bitdepth as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_bitdepth)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_level_idc as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_level_idc)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_frame_total as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_frame_total)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_nal_hrd as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_nal_hrd)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).vui as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(vui)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_frame_reference as *const _ as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_frame_reference)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_dpb_size as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_dpb_size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_keyint_max as *const _ as usize },
        100usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_keyint_max)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_keyint_min as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_keyint_min)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t>())).i_scenecut_threshold as *const _ as usize
        },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_scenecut_threshold)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_intra_refresh as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_intra_refresh)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_bframe as *const _ as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_bframe)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_bframe_adaptive as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_bframe_adaptive)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_bframe_bias as *const _ as usize },
        124usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_bframe_bias)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_bframe_pyramid as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_bframe_pyramid)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_open_gop as *const _ as usize },
        132usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_open_gop)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_bluray_compat as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_bluray_compat)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_avcintra_class as *const _ as usize },
        140usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_avcintra_class)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_avcintra_flavor as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_avcintra_flavor)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t>())).b_deblocking_filter as *const _ as usize
        },
        148usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_deblocking_filter)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t>())).i_deblocking_filter_alphac0 as *const _
                as usize
        },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_deblocking_filter_alphac0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t>())).i_deblocking_filter_beta as *const _ as usize
        },
        156usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_deblocking_filter_beta)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_cabac as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_cabac)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_cabac_init_idc as *const _ as usize },
        164usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_cabac_init_idc)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_interlaced as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_interlaced)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t>())).b_constrained_intra as *const _ as usize
        },
        172usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_constrained_intra)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_cqm_preset as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_cqm_preset)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).psz_cqm_file as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(psz_cqm_file)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).cqm_4iy as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(cqm_4iy)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).cqm_4py as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(cqm_4py)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).cqm_4ic as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(cqm_4ic)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).cqm_4pc as *const _ as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(cqm_4pc)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).cqm_8iy as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(cqm_8iy)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).cqm_8py as *const _ as usize },
        320usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(cqm_8py)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).cqm_8ic as *const _ as usize },
        384usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(cqm_8ic)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).cqm_8pc as *const _ as usize },
        448usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(cqm_8pc)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).pf_log as *const _ as usize },
        512usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(pf_log)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).p_log_private as *const _ as usize },
        520usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(p_log_private)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_log_level as *const _ as usize },
        528usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_log_level)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_full_recon as *const _ as usize },
        532usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_full_recon)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).psz_dump_yuv as *const _ as usize },
        536usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(psz_dump_yuv)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).analyse as *const _ as usize },
        544usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(analyse)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).rc as *const _ as usize },
        656usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(rc)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).crop_rect as *const _ as usize },
        800usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(crop_rect)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_frame_packing as *const _ as usize },
        816usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_frame_packing)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_param_t>())).i_alternative_transfer as *const _ as usize
        },
        820usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_alternative_transfer)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_aud as *const _ as usize },
        824usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_aud)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_repeat_headers as *const _ as usize },
        828usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_repeat_headers)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_annexb as *const _ as usize },
        832usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_annexb)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_sps_id as *const _ as usize },
        836usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_sps_id)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_vfr_input as *const _ as usize },
        840usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_vfr_input)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_pulldown as *const _ as usize },
        844usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_pulldown)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_fps_num as *const _ as usize },
        848usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_fps_num)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_fps_den as *const _ as usize },
        852usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_fps_den)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_timebase_num as *const _ as usize },
        856usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_timebase_num)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_timebase_den as *const _ as usize },
        860usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_timebase_den)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_tff as *const _ as usize },
        864usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_tff)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_pic_struct as *const _ as usize },
        868usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_pic_struct)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_fake_interlaced as *const _ as usize },
        872usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_fake_interlaced)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_stitchable as *const _ as usize },
        876usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_stitchable)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).b_opencl as *const _ as usize },
        880usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(b_opencl)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_opencl_device as *const _ as usize },
        884usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_opencl_device)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).opencl_device_id as *const _ as usize },
        888usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(opencl_device_id)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).psz_clbin_file as *const _ as usize },
        896usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(psz_clbin_file)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_slice_max_size as *const _ as usize },
        904usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_slice_max_size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_slice_max_mbs as *const _ as usize },
        908usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_slice_max_mbs)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_slice_min_mbs as *const _ as usize },
        912usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_slice_min_mbs)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_slice_count as *const _ as usize },
        916usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_slice_count)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).i_slice_count_max as *const _ as usize },
        920usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(i_slice_count_max)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).param_free as *const _ as usize },
        928usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(param_free)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).nalu_process as *const _ as usize },
        936usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(nalu_process)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_param_t>())).opaque as *const _ as usize },
        944usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_param_t),
            "::",
            stringify!(opaque)
        )
    );
}
extern "C" {
    pub fn x264_nal_encode(h: *mut x264_t, dst: *mut u8, nal: *mut x264_nal_t);
}
#[doc = " H.264 level restriction information"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_level_t {
    pub level_idc: u8,
    pub mbps: i32,
    pub frame_size: i32,
    pub dpb: i32,
    pub bitrate: i32,
    pub cpb: i32,
    pub mv_range: u16,
    pub mvs_per_2mb: u8,
    pub slice_rate: u8,
    pub mincr: u8,
    pub bipred8x8: u8,
    pub direct8x8: u8,
    pub frame_only: u8,
}
#[test]
fn bindgen_test_layout_x264_level_t() {
    assert_eq!(
        std::mem::size_of::<x264_level_t>(),
        32usize,
        concat!("Size of: ", stringify!(x264_level_t))
    );
    assert_eq!(
        std::mem::align_of::<x264_level_t>(),
        4usize,
        concat!("Alignment of ", stringify!(x264_level_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_level_t>())).level_idc as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_level_t),
            "::",
            stringify!(level_idc)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_level_t>())).mbps as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_level_t),
            "::",
            stringify!(mbps)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_level_t>())).frame_size as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_level_t),
            "::",
            stringify!(frame_size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_level_t>())).dpb as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_level_t),
            "::",
            stringify!(dpb)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_level_t>())).bitrate as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_level_t),
            "::",
            stringify!(bitrate)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_level_t>())).cpb as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_level_t),
            "::",
            stringify!(cpb)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_level_t>())).mv_range as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_level_t),
            "::",
            stringify!(mv_range)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_level_t>())).mvs_per_2mb as *const _ as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_level_t),
            "::",
            stringify!(mvs_per_2mb)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_level_t>())).slice_rate as *const _ as usize },
        27usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_level_t),
            "::",
            stringify!(slice_rate)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_level_t>())).mincr as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_level_t),
            "::",
            stringify!(mincr)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_level_t>())).bipred8x8 as *const _ as usize },
        29usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_level_t),
            "::",
            stringify!(bipred8x8)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_level_t>())).direct8x8 as *const _ as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_level_t),
            "::",
            stringify!(direct8x8)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_level_t>())).frame_only as *const _ as usize },
        31usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_level_t),
            "::",
            stringify!(frame_only)
        )
    );
}
extern "C" {
    pub static mut x264_levels: [x264_level_t; 0usize];
}
extern "C" {
    #[doc = " Basic parameter handling functions"]
    pub fn x264_param_default(arg1: *mut x264_param_t);
}
extern "C" {
    pub fn x264_param_parse(
        arg1: *mut x264_param_t,
        name: *const std::os::raw::c_char,
        value: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
}
extern "C" {
    pub fn x264_param_cleanup(param: *mut x264_param_t);
}
extern "C" {
    pub static x264_preset_names: [*const std::os::raw::c_char; 11usize];
}
extern "C" {
    pub static x264_tune_names: [*const std::os::raw::c_char; 9usize];
}
extern "C" {
    pub fn x264_param_default_preset(
        arg1: *mut x264_param_t,
        preset: *const std::os::raw::c_char,
        tune: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
}
extern "C" {
    pub fn x264_param_apply_fastfirstpass(arg1: *mut x264_param_t);
}
extern "C" {
    pub static x264_profile_names: [*const std::os::raw::c_char; 7usize];
}
extern "C" {
    pub fn x264_param_apply_profile(
        arg1: *mut x264_param_t,
        profile: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
}
extern "C" {
    pub static x264_chroma_format: std::os::raw::c_int;
}
pub const pic_struct_e_PIC_STRUCT_AUTO: pic_struct_e = 0;
pub const pic_struct_e_PIC_STRUCT_PROGRESSIVE: pic_struct_e = 1;
pub const pic_struct_e_PIC_STRUCT_TOP_BOTTOM: pic_struct_e = 4;
pub const pic_struct_e_PIC_STRUCT_BOTTOM_TOP: pic_struct_e = 5;
pub const pic_struct_e_PIC_STRUCT_TOP_BOTTOM_TOP: pic_struct_e = 6;
pub const pic_struct_e_PIC_STRUCT_BOTTOM_TOP_BOTTOM: pic_struct_e = 7;
pub const pic_struct_e_PIC_STRUCT_DOUBLE: pic_struct_e = 8;
pub const pic_struct_e_PIC_STRUCT_TRIPLE: pic_struct_e = 9;
pub type pic_struct_e = std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_hrd_t {
    pub cpb_initial_arrival_time: f64,
    pub cpb_final_arrival_time: f64,
    pub cpb_removal_time: f64,
    pub dpb_output_time: f64,
}
#[test]
fn bindgen_test_layout_x264_hrd_t() {
    assert_eq!(
        std::mem::size_of::<x264_hrd_t>(),
        32usize,
        concat!("Size of: ", stringify!(x264_hrd_t))
    );
    assert_eq!(
        std::mem::align_of::<x264_hrd_t>(),
        8usize,
        concat!("Alignment of ", stringify!(x264_hrd_t))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_hrd_t>())).cpb_initial_arrival_time as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_hrd_t),
            "::",
            stringify!(cpb_initial_arrival_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_hrd_t>())).cpb_final_arrival_time as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_hrd_t),
            "::",
            stringify!(cpb_final_arrival_time)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_hrd_t>())).cpb_removal_time as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_hrd_t),
            "::",
            stringify!(cpb_removal_time)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_hrd_t>())).dpb_output_time as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_hrd_t),
            "::",
            stringify!(dpb_output_time)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_sei_payload_t {
    pub payload_size: std::os::raw::c_int,
    pub payload_type: std::os::raw::c_int,
    pub payload: *mut u8,
}
#[test]
fn bindgen_test_layout_x264_sei_payload_t() {
    assert_eq!(
        std::mem::size_of::<x264_sei_payload_t>(),
        16usize,
        concat!("Size of: ", stringify!(x264_sei_payload_t))
    );
    assert_eq!(
        std::mem::align_of::<x264_sei_payload_t>(),
        8usize,
        concat!("Alignment of ", stringify!(x264_sei_payload_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_sei_payload_t>())).payload_size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_sei_payload_t),
            "::",
            stringify!(payload_size)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_sei_payload_t>())).payload_type as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_sei_payload_t),
            "::",
            stringify!(payload_type)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_sei_payload_t>())).payload as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_sei_payload_t),
            "::",
            stringify!(payload)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_sei_t {
    pub num_payloads: std::os::raw::c_int,
    pub payloads: *mut x264_sei_payload_t,
    pub sei_free: std::option::Option<unsafe extern "C" fn(arg1: *mut std::os::raw::c_void)>,
}
#[test]
fn bindgen_test_layout_x264_sei_t() {
    assert_eq!(
        std::mem::size_of::<x264_sei_t>(),
        24usize,
        concat!("Size of: ", stringify!(x264_sei_t))
    );
    assert_eq!(
        std::mem::align_of::<x264_sei_t>(),
        8usize,
        concat!("Alignment of ", stringify!(x264_sei_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_sei_t>())).num_payloads as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_sei_t),
            "::",
            stringify!(num_payloads)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_sei_t>())).payloads as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_sei_t),
            "::",
            stringify!(payloads)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_sei_t>())).sei_free as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_sei_t),
            "::",
            stringify!(sei_free)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_image_t {
    pub i_csp: std::os::raw::c_int,
    pub i_plane: std::os::raw::c_int,
    pub i_stride: [std::os::raw::c_int; 4usize],
    pub plane: [*mut u8; 4usize],
}
#[test]
fn bindgen_test_layout_x264_image_t() {
    assert_eq!(
        std::mem::size_of::<x264_image_t>(),
        56usize,
        concat!("Size of: ", stringify!(x264_image_t))
    );
    assert_eq!(
        std::mem::align_of::<x264_image_t>(),
        8usize,
        concat!("Alignment of ", stringify!(x264_image_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_image_t>())).i_csp as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_image_t),
            "::",
            stringify!(i_csp)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_image_t>())).i_plane as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_image_t),
            "::",
            stringify!(i_plane)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_image_t>())).i_stride as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_image_t),
            "::",
            stringify!(i_stride)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_image_t>())).plane as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_image_t),
            "::",
            stringify!(plane)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_image_properties_t {
    pub quant_offsets: *mut f32,
    pub quant_offsets_free:
        std::option::Option<unsafe extern "C" fn(arg1: *mut std::os::raw::c_void)>,
    pub mb_info: *mut u8,
    pub mb_info_free:
        std::option::Option<unsafe extern "C" fn(arg1: *mut std::os::raw::c_void)>,
    pub f_ssim: f64,
    pub f_psnr_avg: f64,
    pub f_psnr: [f64; 3usize],
    pub f_crf_avg: f64,
}
#[test]
fn bindgen_test_layout_x264_image_properties_t() {
    assert_eq!(
        std::mem::size_of::<x264_image_properties_t>(),
        80usize,
        concat!("Size of: ", stringify!(x264_image_properties_t))
    );
    assert_eq!(
        std::mem::align_of::<x264_image_properties_t>(),
        8usize,
        concat!("Alignment of ", stringify!(x264_image_properties_t))
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_image_properties_t>())).quant_offsets as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_image_properties_t),
            "::",
            stringify!(quant_offsets)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_image_properties_t>())).quant_offsets_free as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_image_properties_t),
            "::",
            stringify!(quant_offsets_free)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_image_properties_t>())).mb_info as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_image_properties_t),
            "::",
            stringify!(mb_info)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_image_properties_t>())).mb_info_free as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_image_properties_t),
            "::",
            stringify!(mb_info_free)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_image_properties_t>())).f_ssim as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_image_properties_t),
            "::",
            stringify!(f_ssim)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_image_properties_t>())).f_psnr_avg as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_image_properties_t),
            "::",
            stringify!(f_psnr_avg)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_image_properties_t>())).f_psnr as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_image_properties_t),
            "::",
            stringify!(f_psnr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(std::ptr::null::<x264_image_properties_t>())).f_crf_avg as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_image_properties_t),
            "::",
            stringify!(f_crf_avg)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct x264_picture_t {
    pub i_type: std::os::raw::c_int,
    pub i_qpplus1: std::os::raw::c_int,
    pub i_pic_struct: std::os::raw::c_int,
    pub b_keyframe: std::os::raw::c_int,
    pub i_pts: i64,
    pub i_dts: i64,
    pub param: *mut x264_param_t,
    pub img: x264_image_t,
    pub prop: x264_image_properties_t,
    pub hrd_timing: x264_hrd_t,
    pub extra_sei: x264_sei_t,
    pub opaque: *mut std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_x264_picture_t() {
    assert_eq!(
        std::mem::size_of::<x264_picture_t>(),
        240usize,
        concat!("Size of: ", stringify!(x264_picture_t))
    );
    assert_eq!(
        std::mem::align_of::<x264_picture_t>(),
        8usize,
        concat!("Alignment of ", stringify!(x264_picture_t))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_picture_t>())).i_type as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_picture_t),
            "::",
            stringify!(i_type)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_picture_t>())).i_qpplus1 as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_picture_t),
            "::",
            stringify!(i_qpplus1)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_picture_t>())).i_pic_struct as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_picture_t),
            "::",
            stringify!(i_pic_struct)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_picture_t>())).b_keyframe as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_picture_t),
            "::",
            stringify!(b_keyframe)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_picture_t>())).i_pts as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_picture_t),
            "::",
            stringify!(i_pts)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_picture_t>())).i_dts as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_picture_t),
            "::",
            stringify!(i_dts)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_picture_t>())).param as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_picture_t),
            "::",
            stringify!(param)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_picture_t>())).img as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_picture_t),
            "::",
            stringify!(img)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_picture_t>())).prop as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_picture_t),
            "::",
            stringify!(prop)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_picture_t>())).hrd_timing as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_picture_t),
            "::",
            stringify!(hrd_timing)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_picture_t>())).extra_sei as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_picture_t),
            "::",
            stringify!(extra_sei)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<x264_picture_t>())).opaque as *const _ as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(x264_picture_t),
            "::",
            stringify!(opaque)
        )
    );
}
extern "C" {
    pub fn x264_picture_init(pic: *mut x264_picture_t);
}
extern "C" {
    pub fn x264_picture_alloc(
        pic: *mut x264_picture_t,
        i_csp: std::os::raw::c_int,
        i_width: std::os::raw::c_int,
        i_height: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
}
extern "C" {
    pub fn x264_picture_clean(pic: *mut x264_picture_t);
}
extern "C" {
    pub fn x264_encoder_open_161(arg1: *mut x264_param_t) -> *mut x264_t;
}
extern "C" {
    pub fn x264_encoder_reconfig(
        arg1: *mut x264_t,
        arg2: *mut x264_param_t,
    ) -> std::os::raw::c_int;
}
extern "C" {
    pub fn x264_encoder_parameters(arg1: *mut x264_t, arg2: *mut x264_param_t);
}
extern "C" {
    pub fn x264_encoder_headers(
        arg1: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut std::os::raw::c_int,
    ) -> std::os::raw::c_int;
}
extern "C" {
    pub fn x264_encoder_encode(
        arg1: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut std::os::raw::c_int,
        pic_in: *mut x264_picture_t,
        pic_out: *mut x264_picture_t,
    ) -> std::os::raw::c_int;
}
extern "C" {
    pub fn x264_encoder_close(arg1: *mut x264_t);
}
extern "C" {
    pub fn x264_encoder_delayed_frames(arg1: *mut x264_t) -> std::os::raw::c_int;
}
extern "C" {
    pub fn x264_encoder_maximum_delayed_frames(arg1: *mut x264_t) -> std::os::raw::c_int;
}
extern "C" {
    pub fn x264_encoder_intra_refresh(arg1: *mut x264_t);
}
extern "C" {
    pub fn x264_encoder_invalidate_reference(arg1: *mut x264_t, pts: i64) -> std::os::raw::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: std::os::raw::c_uint,
    pub fp_offset: std::os::raw::c_uint,
    pub overflow_arg_area: *mut std::os::raw::c_void,
    pub reg_save_area: *mut std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(
        std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__va_list_tag>())).gp_offset as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__va_list_tag>())).fp_offset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__va_list_tag>())).overflow_arg_area as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { &(*(std::ptr::null::<__va_list_tag>())).reg_save_area as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}
