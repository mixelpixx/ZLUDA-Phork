use cuda_types::cufft::*;
use hipfft_sys::*;
use std::collections::HashMap;
use std::mem;
use std::sync::Mutex;
use std::sync::atomic::{AtomicI32, Ordering};

static NEXT_HANDLE: AtomicI32 = AtomicI32::new(1);
static HANDLE_MAP: Mutex<Option<HashMap<cufftHandle, hipfftHandle>>> = Mutex::new(None);

fn to_hipfft(handle: cufftHandle) -> Result<hipfftHandle, cufftError_t> {
    let map = HANDLE_MAP.lock().map_err(|_| cufftError_t::INTERNAL_ERROR)?;
    map.as_ref()
        .and_then(|m| m.get(&handle).copied())
        .ok_or(cufftError_t::INVALID_PLAN)
}

fn insert_handle(hip_handle: hipfftHandle) -> Result<cufftHandle, cufftError_t> {
    let id = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    let mut map = HANDLE_MAP.lock().map_err(|_| cufftError_t::INTERNAL_ERROR)?;
    map.get_or_insert_with(HashMap::new).insert(id, hip_handle);
    Ok(id)
}

fn remove_handle(handle: cufftHandle) -> Result<hipfftHandle, cufftError_t> {
    let mut map = HANDLE_MAP.lock().map_err(|_| cufftError_t::INTERNAL_ERROR)?;
    map.as_mut()
        .and_then(|m| m.remove(&handle))
        .ok_or(cufftError_t::INVALID_PLAN)
}

fn hipfft_result_to_cufft(result: hipfftResult) -> cufftResult {
    match result {
        hipfftResult_t::HIPFFT_SUCCESS => Ok(()),
        hipfftResult_t::HIPFFT_INVALID_PLAN => Err(cufftError_t::INVALID_PLAN),
        hipfftResult_t::HIPFFT_ALLOC_FAILED => Err(cufftError_t::ALLOC_FAILED),
        hipfftResult_t::HIPFFT_INVALID_VALUE => Err(cufftError_t::INVALID_VALUE),
        hipfftResult_t::HIPFFT_INTERNAL_ERROR => Err(cufftError_t::INTERNAL_ERROR),
        hipfftResult_t::HIPFFT_EXEC_FAILED => Err(cufftError_t::EXEC_FAILED),
        hipfftResult_t::HIPFFT_SETUP_FAILED => Err(cufftError_t::SETUP_FAILED),
        hipfftResult_t::HIPFFT_INVALID_SIZE => Err(cufftError_t::INVALID_SIZE),
        hipfftResult_t::HIPFFT_NOT_IMPLEMENTED => Err(cufftError_t::NOT_IMPLEMENTED),
        hipfftResult_t::HIPFFT_NOT_SUPPORTED => Err(cufftError_t::NOT_SUPPORTED),
        _ => Err(cufftError_t::INTERNAL_ERROR),
    }
}

fn cufft_type_to_hipfft(t: cufftType) -> hipfftType {
    hipfftType(t.0)
}

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cufftResult {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cufftResult {
    Err(cufftError_t::NOT_SUPPORTED)
}

pub(crate) unsafe fn create(handle: *mut cufftHandle) -> cufftResult {
    if handle.is_null() {
        return Err(cufftError_t::INVALID_VALUE);
    }
    let mut hip_handle = hipfftHandle(std::ptr::null_mut());
    hipfft_result_to_cufft(hipfftCreate(&mut hip_handle))?;
    let id = insert_handle(hip_handle)?;
    *handle = id;
    Ok(())
}

pub(crate) unsafe fn destroy(plan: cufftHandle) -> cufftResult {
    let hip_handle = remove_handle(plan)?;
    hipfft_result_to_cufft(hipfftDestroy(hip_handle))
}

pub(crate) unsafe fn plan1d(
    plan: *mut cufftHandle,
    nx: ::core::ffi::c_int,
    type_: cufftType,
    batch: ::core::ffi::c_int,
) -> cufftResult {
    if plan.is_null() {
        return Err(cufftError_t::INVALID_VALUE);
    }
    let mut hip_handle = hipfftHandle(std::ptr::null_mut());
    hipfft_result_to_cufft(hipfftPlan1d(
        &mut hip_handle, nx, cufft_type_to_hipfft(type_), batch,
    ))?;
    let id = insert_handle(hip_handle)?;
    *plan = id;
    Ok(())
}

pub(crate) unsafe fn plan2d(
    plan: *mut cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    type_: cufftType,
) -> cufftResult {
    if plan.is_null() {
        return Err(cufftError_t::INVALID_VALUE);
    }
    let mut hip_handle = hipfftHandle(std::ptr::null_mut());
    hipfft_result_to_cufft(hipfftPlan2d(
        &mut hip_handle, nx, ny, cufft_type_to_hipfft(type_),
    ))?;
    let id = insert_handle(hip_handle)?;
    *plan = id;
    Ok(())
}

pub(crate) unsafe fn plan3d(
    plan: *mut cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    nz: ::core::ffi::c_int,
    type_: cufftType,
) -> cufftResult {
    if plan.is_null() {
        return Err(cufftError_t::INVALID_VALUE);
    }
    let mut hip_handle = hipfftHandle(std::ptr::null_mut());
    hipfft_result_to_cufft(hipfftPlan3d(
        &mut hip_handle, nx, ny, nz, cufft_type_to_hipfft(type_),
    ))?;
    let id = insert_handle(hip_handle)?;
    *plan = id;
    Ok(())
}

pub(crate) unsafe fn plan_many(
    plan: *mut cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_int,
    inembed: *mut ::core::ffi::c_int,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    onembed: *mut ::core::ffi::c_int,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: cufftType,
    batch: ::core::ffi::c_int,
) -> cufftResult {
    if plan.is_null() {
        return Err(cufftError_t::INVALID_VALUE);
    }
    let mut hip_handle = hipfftHandle(std::ptr::null_mut());
    hipfft_result_to_cufft(hipfftPlanMany(
        &mut hip_handle, rank, n, inembed, istride, idist,
        onembed, ostride, odist, cufft_type_to_hipfft(type_), batch,
    ))?;
    let id = insert_handle(hip_handle)?;
    *plan = id;
    Ok(())
}

pub(crate) unsafe fn make_plan1d(
    plan: cufftHandle,
    nx: ::core::ffi::c_int,
    type_: cufftType,
    batch: ::core::ffi::c_int,
    work_size: *mut usize,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftMakePlan1d(
        hip_handle, nx, cufft_type_to_hipfft(type_), batch, work_size,
    ))
}

pub(crate) unsafe fn make_plan2d(
    plan: cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    type_: cufftType,
    work_size: *mut usize,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftMakePlan2d(
        hip_handle, nx, ny, cufft_type_to_hipfft(type_), work_size,
    ))
}

pub(crate) unsafe fn make_plan3d(
    plan: cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    nz: ::core::ffi::c_int,
    type_: cufftType,
    work_size: *mut usize,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftMakePlan3d(
        hip_handle, nx, ny, nz, cufft_type_to_hipfft(type_), work_size,
    ))
}

pub(crate) unsafe fn make_plan_many(
    plan: cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_int,
    inembed: *mut ::core::ffi::c_int,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    onembed: *mut ::core::ffi::c_int,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: cufftType,
    batch: ::core::ffi::c_int,
    work_size: *mut usize,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftMakePlanMany(
        hip_handle, rank, n, inembed, istride, idist,
        onembed, ostride, odist, cufft_type_to_hipfft(type_), batch, work_size,
    ))
}

pub(crate) unsafe fn make_plan_many64(
    plan: cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_longlong,
    inembed: *mut ::core::ffi::c_longlong,
    istride: ::core::ffi::c_longlong,
    idist: ::core::ffi::c_longlong,
    onembed: *mut ::core::ffi::c_longlong,
    ostride: ::core::ffi::c_longlong,
    odist: ::core::ffi::c_longlong,
    type_: cufftType,
    batch: ::core::ffi::c_longlong,
    work_size: *mut usize,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftMakePlanMany64(
        hip_handle, rank, n, inembed, istride, idist,
        onembed, ostride, odist, cufft_type_to_hipfft(type_), batch, work_size,
    ))
}

pub(crate) unsafe fn get_size(
    handle: cufftHandle,
    work_size: *mut usize,
) -> cufftResult {
    let hip_handle = to_hipfft(handle)?;
    hipfft_result_to_cufft(hipfftGetSize(hip_handle, work_size))
}

pub(crate) unsafe fn set_work_area(
    plan: cufftHandle,
    work_area: *mut ::core::ffi::c_void,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftSetWorkArea(hip_handle, work_area))
}

pub(crate) unsafe fn set_auto_allocation(
    plan: cufftHandle,
    auto_allocate: ::core::ffi::c_int,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftSetAutoAllocation(hip_handle, auto_allocate))
}

pub(crate) unsafe fn exec_c2_c(
    plan: cufftHandle,
    idata: *mut cufftComplex,
    odata: *mut cufftComplex,
    direction: ::core::ffi::c_int,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftExecC2C(
        hip_handle,
        idata as *mut hipfftComplex,
        odata as *mut hipfftComplex,
        direction,
    ))
}

pub(crate) unsafe fn exec_r2_c(
    plan: cufftHandle,
    idata: *mut cufftReal,
    odata: *mut cufftComplex,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftExecR2C(
        hip_handle,
        idata as *mut hipfftReal,
        odata as *mut hipfftComplex,
    ))
}

pub(crate) unsafe fn exec_c2_r(
    plan: cufftHandle,
    idata: *mut cufftComplex,
    odata: *mut cufftReal,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftExecC2R(
        hip_handle,
        idata as *mut hipfftComplex,
        odata as *mut hipfftReal,
    ))
}

pub(crate) unsafe fn exec_z2_z(
    plan: cufftHandle,
    idata: *mut cufftDoubleComplex,
    odata: *mut cufftDoubleComplex,
    direction: ::core::ffi::c_int,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftExecZ2Z(
        hip_handle,
        idata as *mut hipfftDoubleComplex,
        odata as *mut hipfftDoubleComplex,
        direction,
    ))
}

pub(crate) unsafe fn exec_d2_z(
    plan: cufftHandle,
    idata: *mut cufftDoubleReal,
    odata: *mut cufftDoubleComplex,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftExecD2Z(
        hip_handle,
        idata as *mut hipfftDoubleReal,
        odata as *mut hipfftDoubleComplex,
    ))
}

pub(crate) unsafe fn exec_z2_d(
    plan: cufftHandle,
    idata: *mut cufftDoubleComplex,
    odata: *mut cufftDoubleReal,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftExecZ2D(
        hip_handle,
        idata as *mut hipfftDoubleComplex,
        odata as *mut hipfftDoubleReal,
    ))
}

pub(crate) unsafe fn set_stream(
    plan: cufftHandle,
    stream: cudaStream_t,
) -> cufftResult {
    let hip_handle = to_hipfft(plan)?;
    hipfft_result_to_cufft(hipfftSetStream(
        hip_handle,
        mem::transmute(stream),
    ))
}

pub(crate) unsafe fn get_version(
    version: *mut ::core::ffi::c_int,
) -> cufftResult {
    hipfft_result_to_cufft(hipfftGetVersion(version))
}

pub(crate) unsafe fn get_property(
    type_: libraryPropertyType,
    value: *mut ::core::ffi::c_int,
) -> cufftResult {
    hipfft_result_to_cufft(hipfftGetProperty(
        hipfftLibraryPropertyType(type_.0),
        value,
    ))
}

pub(crate) unsafe fn estimate1d(
    nx: ::core::ffi::c_int,
    type_: cufftType,
    batch: ::core::ffi::c_int,
    work_size: *mut usize,
) -> cufftResult {
    hipfft_result_to_cufft(hipfftEstimate1d(
        nx, cufft_type_to_hipfft(type_), batch, work_size,
    ))
}

pub(crate) unsafe fn estimate2d(
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    type_: cufftType,
    work_size: *mut usize,
) -> cufftResult {
    hipfft_result_to_cufft(hipfftEstimate2d(
        nx, ny, cufft_type_to_hipfft(type_), work_size,
    ))
}

pub(crate) unsafe fn estimate3d(
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    nz: ::core::ffi::c_int,
    type_: cufftType,
    work_size: *mut usize,
) -> cufftResult {
    hipfft_result_to_cufft(hipfftEstimate3d(
        nx, ny, nz, cufft_type_to_hipfft(type_), work_size,
    ))
}

pub(crate) unsafe fn estimate_many(
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_int,
    inembed: *mut ::core::ffi::c_int,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    onembed: *mut ::core::ffi::c_int,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: cufftType,
    batch: ::core::ffi::c_int,
    work_size: *mut usize,
) -> cufftResult {
    hipfft_result_to_cufft(hipfftEstimateMany(
        rank, n, inembed, istride, idist,
        onembed, ostride, odist, cufft_type_to_hipfft(type_), batch, work_size,
    ))
}

pub(crate) unsafe fn get_size1d(
    handle: cufftHandle,
    nx: ::core::ffi::c_int,
    type_: cufftType,
    batch: ::core::ffi::c_int,
    work_size: *mut usize,
) -> cufftResult {
    let hip_handle = to_hipfft(handle)?;
    hipfft_result_to_cufft(hipfftGetSize1d(
        hip_handle, nx, cufft_type_to_hipfft(type_), batch, work_size,
    ))
}

pub(crate) unsafe fn get_size2d(
    handle: cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    type_: cufftType,
    work_size: *mut usize,
) -> cufftResult {
    let hip_handle = to_hipfft(handle)?;
    hipfft_result_to_cufft(hipfftGetSize2d(
        hip_handle, nx, ny, cufft_type_to_hipfft(type_), work_size,
    ))
}

pub(crate) unsafe fn get_size3d(
    handle: cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    nz: ::core::ffi::c_int,
    type_: cufftType,
    work_size: *mut usize,
) -> cufftResult {
    let hip_handle = to_hipfft(handle)?;
    hipfft_result_to_cufft(hipfftGetSize3d(
        hip_handle, nx, ny, nz, cufft_type_to_hipfft(type_), work_size,
    ))
}

pub(crate) unsafe fn get_size_many(
    handle: cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_int,
    inembed: *mut ::core::ffi::c_int,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    onembed: *mut ::core::ffi::c_int,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: cufftType,
    batch: ::core::ffi::c_int,
    work_area: *mut usize,
) -> cufftResult {
    let hip_handle = to_hipfft(handle)?;
    hipfft_result_to_cufft(hipfftGetSizeMany(
        hip_handle, rank, n, inembed, istride, idist,
        onembed, ostride, odist, cufft_type_to_hipfft(type_), batch, work_area,
    ))
}

pub(crate) unsafe fn get_size_many64(
    handle: cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_longlong,
    inembed: *mut ::core::ffi::c_longlong,
    istride: ::core::ffi::c_longlong,
    idist: ::core::ffi::c_longlong,
    onembed: *mut ::core::ffi::c_longlong,
    ostride: ::core::ffi::c_longlong,
    odist: ::core::ffi::c_longlong,
    type_: cufftType,
    batch: ::core::ffi::c_longlong,
    work_size: *mut usize,
) -> cufftResult {
    let hip_handle = to_hipfft(handle)?;
    hipfft_result_to_cufft(hipfftGetSizeMany64(
        hip_handle, rank, n, inembed, istride, idist,
        onembed, ostride, odist, cufft_type_to_hipfft(type_), batch, work_size,
    ))
}
