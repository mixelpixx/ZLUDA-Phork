#![allow(non_camel_case_types)]

use hip_runtime_sys::*;

pub type hipfftReal = f32;
pub type hipfftDoubleReal = f64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipfftComplex {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipfftDoubleComplex {
    pub x: f64,
    pub y: f64,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipfftHandle(pub *mut ::core::ffi::c_void);

unsafe impl Send for hipfftHandle {}
unsafe impl Sync for hipfftHandle {}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipfftType(pub ::core::ffi::c_uint);

impl hipfftType {
    pub const HIPFFT_R2C: hipfftType = hipfftType(0x2a);
    pub const HIPFFT_C2R: hipfftType = hipfftType(0x2c);
    pub const HIPFFT_C2C: hipfftType = hipfftType(0x29);
    pub const HIPFFT_D2Z: hipfftType = hipfftType(0x6a);
    pub const HIPFFT_Z2D: hipfftType = hipfftType(0x6c);
    pub const HIPFFT_Z2Z: hipfftType = hipfftType(0x69);
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipfftResult_t(pub ::core::ffi::c_uint);

impl hipfftResult_t {
    pub const HIPFFT_SUCCESS: hipfftResult_t = hipfftResult_t(0);
    pub const HIPFFT_INVALID_PLAN: hipfftResult_t = hipfftResult_t(1);
    pub const HIPFFT_ALLOC_FAILED: hipfftResult_t = hipfftResult_t(2);
    pub const HIPFFT_INVALID_TYPE: hipfftResult_t = hipfftResult_t(3);
    pub const HIPFFT_INVALID_VALUE: hipfftResult_t = hipfftResult_t(4);
    pub const HIPFFT_INTERNAL_ERROR: hipfftResult_t = hipfftResult_t(5);
    pub const HIPFFT_EXEC_FAILED: hipfftResult_t = hipfftResult_t(6);
    pub const HIPFFT_SETUP_FAILED: hipfftResult_t = hipfftResult_t(7);
    pub const HIPFFT_INVALID_SIZE: hipfftResult_t = hipfftResult_t(8);
    pub const HIPFFT_UNALIGNED_DATA: hipfftResult_t = hipfftResult_t(9);
    pub const HIPFFT_INCOMPLETE_PARAMETER_LIST: hipfftResult_t = hipfftResult_t(10);
    pub const HIPFFT_INVALID_DEVICE: hipfftResult_t = hipfftResult_t(11);
    pub const HIPFFT_PARSE_ERROR: hipfftResult_t = hipfftResult_t(12);
    pub const HIPFFT_NO_WORKSPACE: hipfftResult_t = hipfftResult_t(13);
    pub const HIPFFT_NOT_IMPLEMENTED: hipfftResult_t = hipfftResult_t(14);
    pub const HIPFFT_NOT_SUPPORTED: hipfftResult_t = hipfftResult_t(16);
}

pub type hipfftResult = hipfftResult_t;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipfftLibraryPropertyType(pub ::core::ffi::c_uint);

impl hipfftLibraryPropertyType {
    pub const HIPFFT_MAJOR_VERSION: hipfftLibraryPropertyType = hipfftLibraryPropertyType(0);
    pub const HIPFFT_MINOR_VERSION: hipfftLibraryPropertyType = hipfftLibraryPropertyType(1);
    pub const HIPFFT_PATCH_LEVEL: hipfftLibraryPropertyType = hipfftLibraryPropertyType(2);
}

#[cfg_attr(windows, link(name = "hipfft", kind = "raw-dylib"))]
extern "C" {
    pub fn hipfftCreate(plan: *mut hipfftHandle) -> hipfftResult;
    pub fn hipfftDestroy(plan: hipfftHandle) -> hipfftResult;

    pub fn hipfftPlan1d(
        plan: *mut hipfftHandle,
        nx: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
    ) -> hipfftResult;

    pub fn hipfftPlan2d(
        plan: *mut hipfftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        type_: hipfftType,
    ) -> hipfftResult;

    pub fn hipfftPlan3d(
        plan: *mut hipfftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        nz: ::core::ffi::c_int,
        type_: hipfftType,
    ) -> hipfftResult;

    pub fn hipfftPlanMany(
        plan: *mut hipfftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_int,
        inembed: *mut ::core::ffi::c_int,
        istride: ::core::ffi::c_int,
        idist: ::core::ffi::c_int,
        onembed: *mut ::core::ffi::c_int,
        ostride: ::core::ffi::c_int,
        odist: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
    ) -> hipfftResult;

    pub fn hipfftMakePlan1d(
        plan: hipfftHandle,
        nx: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftMakePlan2d(
        plan: hipfftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        type_: hipfftType,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftMakePlan3d(
        plan: hipfftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        nz: ::core::ffi::c_int,
        type_: hipfftType,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftMakePlanMany(
        plan: hipfftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_int,
        inembed: *mut ::core::ffi::c_int,
        istride: ::core::ffi::c_int,
        idist: ::core::ffi::c_int,
        onembed: *mut ::core::ffi::c_int,
        ostride: ::core::ffi::c_int,
        odist: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftMakePlanMany64(
        plan: hipfftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_longlong,
        inembed: *mut ::core::ffi::c_longlong,
        istride: ::core::ffi::c_longlong,
        idist: ::core::ffi::c_longlong,
        onembed: *mut ::core::ffi::c_longlong,
        ostride: ::core::ffi::c_longlong,
        odist: ::core::ffi::c_longlong,
        type_: hipfftType,
        batch: ::core::ffi::c_longlong,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftEstimate1d(
        nx: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftEstimate2d(
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        type_: hipfftType,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftEstimate3d(
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        nz: ::core::ffi::c_int,
        type_: hipfftType,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftEstimateMany(
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_int,
        inembed: *mut ::core::ffi::c_int,
        istride: ::core::ffi::c_int,
        idist: ::core::ffi::c_int,
        onembed: *mut ::core::ffi::c_int,
        ostride: ::core::ffi::c_int,
        odist: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftGetSize1d(
        plan: hipfftHandle,
        nx: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftGetSize2d(
        plan: hipfftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        type_: hipfftType,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftGetSize3d(
        plan: hipfftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        nz: ::core::ffi::c_int,
        type_: hipfftType,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftGetSizeMany(
        plan: hipfftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_int,
        inembed: *mut ::core::ffi::c_int,
        istride: ::core::ffi::c_int,
        idist: ::core::ffi::c_int,
        onembed: *mut ::core::ffi::c_int,
        ostride: ::core::ffi::c_int,
        odist: ::core::ffi::c_int,
        type_: hipfftType,
        batch: ::core::ffi::c_int,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftGetSizeMany64(
        plan: hipfftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_longlong,
        inembed: *mut ::core::ffi::c_longlong,
        istride: ::core::ffi::c_longlong,
        idist: ::core::ffi::c_longlong,
        onembed: *mut ::core::ffi::c_longlong,
        ostride: ::core::ffi::c_longlong,
        odist: ::core::ffi::c_longlong,
        type_: hipfftType,
        batch: ::core::ffi::c_longlong,
        work_size: *mut usize,
    ) -> hipfftResult;

    pub fn hipfftGetSize(plan: hipfftHandle, work_size: *mut usize) -> hipfftResult;

    pub fn hipfftSetAutoAllocation(
        plan: hipfftHandle,
        auto_allocate: ::core::ffi::c_int,
    ) -> hipfftResult;

    pub fn hipfftSetWorkArea(
        plan: hipfftHandle,
        work_area: *mut ::core::ffi::c_void,
    ) -> hipfftResult;

    pub fn hipfftExecC2C(
        plan: hipfftHandle,
        idata: *mut hipfftComplex,
        odata: *mut hipfftComplex,
        direction: ::core::ffi::c_int,
    ) -> hipfftResult;

    pub fn hipfftExecR2C(
        plan: hipfftHandle,
        idata: *mut hipfftReal,
        odata: *mut hipfftComplex,
    ) -> hipfftResult;

    pub fn hipfftExecC2R(
        plan: hipfftHandle,
        idata: *mut hipfftComplex,
        odata: *mut hipfftReal,
    ) -> hipfftResult;

    pub fn hipfftExecZ2Z(
        plan: hipfftHandle,
        idata: *mut hipfftDoubleComplex,
        odata: *mut hipfftDoubleComplex,
        direction: ::core::ffi::c_int,
    ) -> hipfftResult;

    pub fn hipfftExecD2Z(
        plan: hipfftHandle,
        idata: *mut hipfftDoubleReal,
        odata: *mut hipfftDoubleComplex,
    ) -> hipfftResult;

    pub fn hipfftExecZ2D(
        plan: hipfftHandle,
        idata: *mut hipfftDoubleComplex,
        odata: *mut hipfftDoubleReal,
    ) -> hipfftResult;

    pub fn hipfftSetStream(plan: hipfftHandle, stream: hipStream_t) -> hipfftResult;

    pub fn hipfftGetVersion(version: *mut ::core::ffi::c_int) -> hipfftResult;

    pub fn hipfftGetProperty(
        type_: hipfftLibraryPropertyType,
        value: *mut ::core::ffi::c_int,
    ) -> hipfftResult;
}
