use cuda_types::cublas::*;
use hip_runtime_sys::hipStream_t;
use rocblas_sys::*;
use std::mem;
use zluda_common::{from_cuda_object, ZludaObject};

pub struct Handle {
    handle: rocblas_handle,
}

impl Handle {
    fn new() -> Self {
        Self {
            handle: unsafe { mem::zeroed() },
        }
    }
}

impl ZludaObject for Handle {
    const COOKIE: usize = 0x57c3fdb0fd72b08e;

    type Error = cublasError_t;
    type CudaHandle = cublasHandle_t;

    fn drop_checked(&mut self) -> cublasStatus_t {
        Ok(())
    }
}

from_cuda_object!(Handle);

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cublasStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cublasStatus_t {
    cublasStatus_t::ERROR_NOT_SUPPORTED
}

pub(crate) fn create_v2(handle: &mut cublasHandle_t) -> cublasStatus_t {
    let mut zluda_blas_handle = Handle::new();
    unsafe { rocblas_create_handle(&mut zluda_blas_handle.handle) }?;
    *handle = Handle::wrap(zluda_blas_handle);
    Ok(())
}

pub(crate) fn get_status_name(status: cublasStatus_t) -> *const ::core::ffi::c_char {
    match status {
        cublasStatus_t::SUCCESS => "CUBLAS_STATUS_SUCCESS\0",
        cublasStatus_t::ERROR_NOT_INITIALIZED => "CUBLAS_STATUS_NOT_INITIALIZED\0",
        cublasStatus_t::ERROR_ALLOC_FAILED => "CUBLAS_STATUS_ALLOC_FAILED\0",
        cublasStatus_t::ERROR_INVALID_VALUE => "CUBLAS_STATUS_INVALID_VALUE\0",
        cublasStatus_t::ERROR_ARCH_MISMATCH => "CUBLAS_STATUS_ARCH_MISMATCH\0",
        cublasStatus_t::ERROR_MAPPING_ERROR => "CUBLAS_STATUS_MAPPING_ERROR\0",
        cublasStatus_t::ERROR_EXECUTION_FAILED => "CUBLAS_STATUS_EXECUTION_FAILED\0",
        cublasStatus_t::ERROR_INTERNAL_ERROR => "CUBLAS_STATUS_INTERNAL_ERROR\0",
        cublasStatus_t::ERROR_NOT_SUPPORTED => "CUBLAS_STATUS_NOT_SUPPORTED\0",
        cublasStatus_t::ERROR_LICENSE_ERROR => "CUBLAS_STATUS_LICENSE_ERROR\0",
        _ => "CUBLAS_STATUS_UNKNOWN\0",
    }
    .as_ptr() as *const ::core::ffi::c_char
}

pub(crate) fn get_status_string(status: cublasStatus_t) -> *const ::core::ffi::c_char {
    match status {
        cublasStatus_t::SUCCESS => "CUBLAS_STATUS_SUCCESS\0",
        cublasStatus_t::ERROR_NOT_INITIALIZED => "CUBLAS_STATUS_NOT_INITIALIZED\0",
        cublasStatus_t::ERROR_ALLOC_FAILED => "CUBLAS_STATUS_ALLOC_FAILED\0",
        cublasStatus_t::ERROR_INVALID_VALUE => "CUBLAS_STATUS_INVALID_VALUE\0",
        cublasStatus_t::ERROR_ARCH_MISMATCH => "CUBLAS_STATUS_ARCH_MISMATCH\0",
        cublasStatus_t::ERROR_MAPPING_ERROR => "CUBLAS_STATUS_MAPPING_ERROR\0",
        cublasStatus_t::ERROR_EXECUTION_FAILED => "CUBLAS_STATUS_EXECUTION_FAILED\0",
        cublasStatus_t::ERROR_INTERNAL_ERROR => "CUBLAS_STATUS_INTERNAL_ERROR\0",
        cublasStatus_t::ERROR_NOT_SUPPORTED => "CUBLAS_STATUS_NOT_SUPPORTED\0",
        cublasStatus_t::ERROR_LICENSE_ERROR => "CUBLAS_STATUS_LICENSE_ERROR\0",
        _ => "CUBLAS_STATUS_UNKNOWN\0",
    }
    .as_ptr() as *const ::core::ffi::c_char
}

pub(crate) fn xerbla(sr_name: *const ::core::ffi::c_char, info: ::core::ffi::c_int) -> () {
    // XERBLA is a BLAS/LAPACK error handler
    // In debug builds, print the error; in release, silently continue
    #[cfg(debug_assertions)]
    {
        let name = unsafe { std::ffi::CStr::from_ptr(sr_name) };
        eprintln!("XERBLA: Parameter {} had an illegal value ({})",
            name.to_string_lossy(), info);
    }
    let _ = (sr_name, info); // Suppress unused warnings in release
}

pub(crate) fn get_cudart_version() -> usize {
    // Return CUDA runtime version matching the driver (CUDA 13.0)
    cuda_types::cuda::CUDA_VERSION as usize
}

pub(crate) fn set_math_mode(handle: &Handle, mode: rocblas_math_mode) -> cublasStatus_t {
    unsafe { rocblas_set_math_mode(handle.handle, mode) }?;
    Ok(())
}

pub(crate) fn sgemm_strided_batched(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const f32,
    a: *const f32,
    lda: ::core::ffi::c_int,
    stride_a: ::core::ffi::c_longlong,
    b: *const f32,
    ldb: ::core::ffi::c_int,
    stride_b: ::core::ffi::c_longlong,
    beta: *const f32,
    c: *mut f32,
    ldc: ::core::ffi::c_int,
    stride_c: ::core::ffi::c_longlong,
    batch_count: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe {
        rocblas_sgemm_strided_batched(
            handle.handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a,
            lda,
            stride_a,
            b,
            ldb,
            stride_b,
            beta,
            c,
            ldc,
            stride_c,
            batch_count,
        )
    }?;
    Ok(())
}

pub(crate) fn sgemm_v2(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const f32,
    a: *const f32,
    lda: ::core::ffi::c_int,
    b: *const f32,
    ldb: ::core::ffi::c_int,
    beta: *const f32,
    c: *mut f32,
    ldc: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe {
        rocblas_sgemm(
            handle.handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a,
            lda,
            b,
            ldb,
            beta,
            c,
            ldc,
        )
    }?;
    Ok(())
}

pub(crate) fn dgemm_v2(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const f64,
    a: *const f64,
    lda: ::core::ffi::c_int,
    b: *const f64,
    ldb: ::core::ffi::c_int,
    beta: *const f64,
    c: *mut f64,
    ldc: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe {
        rocblas_dgemm(
            handle.handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a,
            lda,
            b,
            ldb,
            beta,
            c,
            ldc,
        )
    }?;
    Ok(())
}

pub(crate) fn dgemm_strided_batched(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const f64,
    a: *const f64,
    lda: ::core::ffi::c_int,
    stride_a: ::core::ffi::c_longlong,
    b: *const f64,
    ldb: ::core::ffi::c_int,
    stride_b: ::core::ffi::c_longlong,
    beta: *const f64,
    c: *mut f64,
    ldc: ::core::ffi::c_int,
    stride_c: ::core::ffi::c_longlong,
    batch_count: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe {
        rocblas_dgemm_strided_batched(
            handle.handle,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a,
            lda,
            stride_a,
            b,
            ldb,
            stride_b,
            beta,
            c,
            ldc,
            stride_c,
            batch_count,
        )
    }?;
    Ok(())
}

pub(crate) fn destroy_v2(handle: cublasHandle_t) -> cublasStatus_t {
    zluda_common::drop_checked::<Handle>(handle)
}

pub(crate) unsafe fn set_pointer_mode_v2(
    handle: &Handle,
    mode: rocblas_pointer_mode,
) -> rocblas_status {
    rocblas_set_pointer_mode(handle.handle, mode)
}

pub(crate) unsafe fn set_stream_v2(handle: &Handle, stream: hipStream_t) -> rocblas_status {
    rocblas_set_stream(handle.handle, stream)
}

pub(crate) unsafe fn set_workspace_v2(
    handle: &Handle,
    workspace: *mut ::core::ffi::c_void,
    size: usize,
) -> rocblas_status {
    rocblas_set_workspace(handle.handle, workspace, size)
}

pub(crate) unsafe fn get_math_mode(handle: &Handle, mode: &mut cublasMath_t) -> rocblas_status {
    let mut roc_mode = mem::zeroed();
    rocblas_get_math_mode(handle.handle, &mut roc_mode)?;
    *mode = zluda_common::FromCuda::from_cuda(&roc_mode)?;
    Ok(())
}

pub(crate) unsafe fn gemm_ex(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const ::core::ffi::c_void,
    a: *const ::core::ffi::c_void,
    a_type: rocblas_datatype,
    lda: ::core::ffi::c_int,
    b: *const ::core::ffi::c_void,
    b_type: rocblas_datatype,
    ldb: ::core::ffi::c_int,
    beta: *const ::core::ffi::c_void,
    c: *mut ::core::ffi::c_void,
    c_type: rocblas_datatype,
    ldc: ::core::ffi::c_int,
    compute_type: rocblas_datatype,
    algo: rocblas_gemm_algo,
) -> rocblas_status {
    rocblas_gemm_ex(
        handle.handle,
        transa,
        transb,
        m,
        n,
        k,
        alpha,
        a,
        a_type,
        lda,
        b,
        b_type,
        ldb,
        beta,
        c,
        c_type,
        ldc,
        c,
        c_type,
        ldc,
        compute_type,
        algo,
        0,
        0,
    )
}

pub(crate) unsafe fn hgemm(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const cuda_types::cublas::__half,
    a: *const cuda_types::cublas::__half,
    lda: ::core::ffi::c_int,
    b: *const cuda_types::cublas::__half,
    ldb: ::core::ffi::c_int,
    beta: *const cuda_types::cublas::__half,
    c: *mut cuda_types::cublas::__half,
    ldc: ::core::ffi::c_int,
) -> rocblas_status {
    rocblas_hgemm(
        handle.handle,
        transa,
        transb,
        m,
        n,
        k,
        alpha.cast(),
        a.cast(),
        lda,
        b.cast(),
        ldb,
        beta.cast(),
        c.cast(),
        ldc,
    )
}

pub(crate) unsafe fn gemm_batched_ex(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const ::core::ffi::c_void,
    a_array: *const *const ::core::ffi::c_void,
    a_type: rocblas_datatype,
    lda: ::core::ffi::c_int,
    b_array: *const *const ::core::ffi::c_void,
    b_type: rocblas_datatype,
    ldb: ::core::ffi::c_int,
    beta: *const ::core::ffi::c_void,
    c_array: *const *mut ::core::ffi::c_void,
    c_type: rocblas_datatype,
    ldc: ::core::ffi::c_int,
    batch_count: ::core::ffi::c_int,
    compute_type: rocblas_datatype,
    algo: rocblas_gemm_algo,
) -> rocblas_status {
    rocblas_gemm_batched_ex(
        handle.handle,
        transa,
        transb,
        m,
        n,
        k,
        alpha,
        a_array.cast(),
        a_type,
        lda,
        b_array.cast(),
        b_type,
        ldb,
        beta,
        c_array.cast(),
        c_type,
        ldc,
        c_array.cast_mut().cast(),
        c_type,
        ldc,
        batch_count,
        compute_type,
        algo,
        -1,
        0,
    )
}

pub(crate) unsafe fn gemm_strided_batched_ex(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const ::core::ffi::c_void,
    a: *const ::core::ffi::c_void,
    a_type: rocblas_datatype,
    lda: ::core::ffi::c_int,
    stride_a: ::core::ffi::c_longlong,
    b: *const ::core::ffi::c_void,
    b_type: rocblas_datatype,
    ldb: ::core::ffi::c_int,
    stride_b: ::core::ffi::c_longlong,
    beta: *const ::core::ffi::c_void,
    c: *mut ::core::ffi::c_void,
    c_type: rocblas_datatype,
    ldc: ::core::ffi::c_int,
    stride_c: ::core::ffi::c_longlong,
    batch_count: ::core::ffi::c_int,
    compute_type: rocblas_datatype,
    algo: rocblas_gemm_algo,
) -> rocblas_status {
    rocblas_gemm_strided_batched_ex(
        handle.handle,
        transa,
        transb,
        m,
        n,
        k,
        alpha,
        a,
        a_type,
        lda,
        stride_a,
        b,
        b_type,
        ldb,
        stride_b,
        beta,
        c,
        c_type,
        ldc,
        stride_c,
        c,
        c_type,
        ldc,
        stride_c,
        batch_count,
        compute_type,
        algo,
        -1,
        0,
    )
}

pub(crate) unsafe fn set_vector(
    n: ::core::ffi::c_int,
    elem_size: ::core::ffi::c_int,
    x: *const ::core::ffi::c_void,
    incx: ::core::ffi::c_int,
    device_ptr: *mut ::core::ffi::c_void,
    incy: ::core::ffi::c_int,
) -> rocblas_status {
    rocblas_set_vector(n, elem_size, x, incx, device_ptr, incy)
}

// BLAS Level 1 operations

pub(crate) fn sscal_v2(
    handle: &Handle,
    n: ::core::ffi::c_int,
    alpha: *const f32,
    x: *mut f32,
    incx: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_sscal(handle.handle, n, alpha, x, incx) }?;
    Ok(())
}

pub(crate) fn dscal_v2(
    handle: &Handle,
    n: ::core::ffi::c_int,
    alpha: *const f64,
    x: *mut f64,
    incx: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_dscal(handle.handle, n, alpha, x, incx) }?;
    Ok(())
}

pub(crate) fn saxpy_v2(
    handle: &Handle,
    n: ::core::ffi::c_int,
    alpha: *const f32,
    x: *const f32,
    incx: ::core::ffi::c_int,
    y: *mut f32,
    incy: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_saxpy(handle.handle, n, alpha, x, incx, y, incy) }?;
    Ok(())
}

pub(crate) fn daxpy_v2(
    handle: &Handle,
    n: ::core::ffi::c_int,
    alpha: *const f64,
    x: *const f64,
    incx: ::core::ffi::c_int,
    y: *mut f64,
    incy: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_daxpy(handle.handle, n, alpha, x, incx, y, incy) }?;
    Ok(())
}

pub(crate) fn sdot_v2(
    handle: &Handle,
    n: ::core::ffi::c_int,
    x: *const f32,
    incx: ::core::ffi::c_int,
    y: *const f32,
    incy: ::core::ffi::c_int,
    result: *mut f32,
) -> cublasStatus_t {
    unsafe { rocblas_sdot(handle.handle, n, x, incx, y, incy, result) }?;
    Ok(())
}

pub(crate) fn ddot_v2(
    handle: &Handle,
    n: ::core::ffi::c_int,
    x: *const f64,
    incx: ::core::ffi::c_int,
    y: *const f64,
    incy: ::core::ffi::c_int,
    result: *mut f64,
) -> cublasStatus_t {
    unsafe { rocblas_ddot(handle.handle, n, x, incx, y, incy, result) }?;
    Ok(())
}

pub(crate) fn snrm2_v2(
    handle: &Handle,
    n: ::core::ffi::c_int,
    x: *const f32,
    incx: ::core::ffi::c_int,
    result: *mut f32,
) -> cublasStatus_t {
    unsafe { rocblas_snrm2(handle.handle, n, x, incx, result) }?;
    Ok(())
}

pub(crate) fn dnrm2_v2(
    handle: &Handle,
    n: ::core::ffi::c_int,
    x: *const f64,
    incx: ::core::ffi::c_int,
    result: *mut f64,
) -> cublasStatus_t {
    unsafe { rocblas_dnrm2(handle.handle, n, x, incx, result) }?;
    Ok(())
}

pub(crate) fn sasum_v2(
    handle: &Handle,
    n: ::core::ffi::c_int,
    x: *const f32,
    incx: ::core::ffi::c_int,
    result: *mut f32,
) -> cublasStatus_t {
    unsafe { rocblas_sasum(handle.handle, n, x, incx, result) }?;
    Ok(())
}

pub(crate) fn dasum_v2(
    handle: &Handle,
    n: ::core::ffi::c_int,
    x: *const f64,
    incx: ::core::ffi::c_int,
    result: *mut f64,
) -> cublasStatus_t {
    unsafe { rocblas_dasum(handle.handle, n, x, incx, result) }?;
    Ok(())
}

pub(crate) fn scopy_v2(
    handle: &Handle,
    n: ::core::ffi::c_int,
    x: *const f32,
    incx: ::core::ffi::c_int,
    y: *mut f32,
    incy: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_scopy(handle.handle, n, x, incx, y, incy) }?;
    Ok(())
}

pub(crate) fn dcopy_v2(
    handle: &Handle,
    n: ::core::ffi::c_int,
    x: *const f64,
    incx: ::core::ffi::c_int,
    y: *mut f64,
    incy: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_dcopy(handle.handle, n, x, incx, y, incy) }?;
    Ok(())
}

pub(crate) unsafe fn get_vector(
    n: ::core::ffi::c_int,
    elem_size: ::core::ffi::c_int,
    x: *const ::core::ffi::c_void,
    incx: ::core::ffi::c_int,
    y: *mut ::core::ffi::c_void,
    incy: ::core::ffi::c_int,
) -> rocblas_status {
    rocblas_get_vector(n, elem_size, x, incx, y, incy)
}

// BLAS Level 2 operations

pub(crate) fn sgemv_v2(
    handle: &Handle,
    trans: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    alpha: *const f32,
    a: *const f32,
    lda: ::core::ffi::c_int,
    x: *const f32,
    incx: ::core::ffi::c_int,
    beta: *const f32,
    y: *mut f32,
    incy: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_sgemv(handle.handle, trans, m, n, alpha, a, lda, x, incx, beta, y, incy) }?;
    Ok(())
}

pub(crate) fn dgemv_v2(
    handle: &Handle,
    trans: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    alpha: *const f64,
    a: *const f64,
    lda: ::core::ffi::c_int,
    x: *const f64,
    incx: ::core::ffi::c_int,
    beta: *const f64,
    y: *mut f64,
    incy: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_dgemv(handle.handle, trans, m, n, alpha, a, lda, x, incx, beta, y, incy) }?;
    Ok(())
}

// BLAS Level 3 operations (beyond GEMM)

pub(crate) fn strsm_v2(
    handle: &Handle,
    side: rocblas_side,
    uplo: rocblas_fill,
    trans: rocblas_operation,
    diag: rocblas_diagonal,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    alpha: *const f32,
    a: *const f32,
    lda: ::core::ffi::c_int,
    b: *mut f32,
    ldb: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_strsm(handle.handle, side, uplo, trans, diag, m, n, alpha, a, lda, b, ldb) }?;
    Ok(())
}

pub(crate) fn dtrsm_v2(
    handle: &Handle,
    side: rocblas_side,
    uplo: rocblas_fill,
    trans: rocblas_operation,
    diag: rocblas_diagonal,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    alpha: *const f64,
    a: *const f64,
    lda: ::core::ffi::c_int,
    b: *mut f64,
    ldb: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_dtrsm(handle.handle, side, uplo, trans, diag, m, n, alpha, a, lda, b, ldb) }?;
    Ok(())
}

pub(crate) fn ssymm_v2(
    handle: &Handle,
    side: rocblas_side,
    uplo: rocblas_fill,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    alpha: *const f32,
    a: *const f32,
    lda: ::core::ffi::c_int,
    b: *const f32,
    ldb: ::core::ffi::c_int,
    beta: *const f32,
    c: *mut f32,
    ldc: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_ssymm(handle.handle, side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc) }?;
    Ok(())
}

pub(crate) fn dsymm_v2(
    handle: &Handle,
    side: rocblas_side,
    uplo: rocblas_fill,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    alpha: *const f64,
    a: *const f64,
    lda: ::core::ffi::c_int,
    b: *const f64,
    ldb: ::core::ffi::c_int,
    beta: *const f64,
    c: *mut f64,
    ldc: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_dsymm(handle.handle, side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc) }?;
    Ok(())
}

pub(crate) fn ssyrk_v2(
    handle: &Handle,
    uplo: rocblas_fill,
    trans: rocblas_operation,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const f32,
    a: *const f32,
    lda: ::core::ffi::c_int,
    beta: *const f32,
    c: *mut f32,
    ldc: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_ssyrk(handle.handle, uplo, trans, n, k, alpha, a, lda, beta, c, ldc) }?;
    Ok(())
}

pub(crate) fn dsyrk_v2(
    handle: &Handle,
    uplo: rocblas_fill,
    trans: rocblas_operation,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const f64,
    a: *const f64,
    lda: ::core::ffi::c_int,
    beta: *const f64,
    c: *mut f64,
    ldc: ::core::ffi::c_int,
) -> cublasStatus_t {
    unsafe { rocblas_dsyrk(handle.handle, uplo, trans, n, k, alpha, a, lda, beta, c, ldc) }?;
    Ok(())
}

// Half-precision batched operations

pub(crate) unsafe fn hgemm_strided_batched(
    handle: &Handle,
    transa: rocblas_operation,
    transb: rocblas_operation,
    m: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    k: ::core::ffi::c_int,
    alpha: *const cuda_types::cublas::__half,
    a: *const cuda_types::cublas::__half,
    lda: ::core::ffi::c_int,
    stride_a: ::core::ffi::c_longlong,
    b: *const cuda_types::cublas::__half,
    ldb: ::core::ffi::c_int,
    stride_b: ::core::ffi::c_longlong,
    beta: *const cuda_types::cublas::__half,
    c: *mut cuda_types::cublas::__half,
    ldc: ::core::ffi::c_int,
    stride_c: ::core::ffi::c_longlong,
    batch_count: ::core::ffi::c_int,
) -> rocblas_status {
    rocblas_hgemm_strided_batched(
        handle.handle,
        transa,
        transb,
        m,
        n,
        k,
        alpha.cast(),
        a.cast(),
        lda,
        stride_a,
        b.cast(),
        ldb,
        stride_b,
        beta.cast(),
        c.cast(),
        ldc,
        stride_c,
        batch_count,
    )
}
