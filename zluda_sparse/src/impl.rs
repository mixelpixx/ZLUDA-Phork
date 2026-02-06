use cuda_types::cuda::CUstream;
use cuda_types::cusparse::*;
use hipsparse_sys::*;
use std::mem;

// Handle wrapper for hipSPARSE
#[repr(C)]
pub struct Handle {
    handle: hipsparseHandle_t,
}

// Matrix descriptor structure
#[repr(C)]
pub struct MatDescr {
    hip_descr: hipsparseMatDescr_t,
}

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cusparseStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cusparseStatus_t {
    cusparseStatus_t::ERROR_NOT_SUPPORTED
}

fn to_cusparse_error(status: hipsparseStatus_t) -> cusparseStatus_t {
    match status {
        Ok(()) => Ok(()),
        Err(e) => Err(match e {
            hipsparseError_t::NOT_INITIALIZED => cusparseError_t::NOT_INITIALIZED,
            hipsparseError_t::ALLOC_FAILED => cusparseError_t::ALLOC_FAILED,
            hipsparseError_t::INVALID_VALUE => cusparseError_t::INVALID_VALUE,
            hipsparseError_t::ARCH_MISMATCH => cusparseError_t::ARCH_MISMATCH,
            hipsparseError_t::MAPPING_ERROR => cusparseError_t::MAPPING_ERROR,
            hipsparseError_t::EXECUTION_FAILED => cusparseError_t::EXECUTION_FAILED,
            hipsparseError_t::INTERNAL_ERROR => cusparseError_t::INTERNAL_ERROR,
            hipsparseError_t::MATRIX_TYPE_NOT_SUPPORTED => cusparseError_t::MATRIX_TYPE_NOT_SUPPORTED,
            hipsparseError_t::ZERO_PIVOT => cusparseError_t::ZERO_PIVOT,
            hipsparseError_t::NOT_SUPPORTED => cusparseError_t::NOT_SUPPORTED,
            hipsparseError_t::INSUFFICIENT_RESOURCES => cusparseError_t::INSUFFICIENT_RESOURCES,
            _ => cusparseError_t::INTERNAL_ERROR,
        }),
    }
}

pub(crate) fn create(handle: *mut cusparseHandle_t) -> cusparseStatus_t {
    if handle.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    let mut hip_handle: hipsparseHandle_t = unsafe { mem::zeroed() };
    to_cusparse_error(unsafe { hipsparseCreate(&mut hip_handle) })?;
    let zluda_handle = Box::new(Handle { handle: hip_handle });
    unsafe { *handle = Box::into_raw(zluda_handle) as cusparseHandle_t };
    Ok(())
}

pub(crate) fn destroy(handle: cusparseHandle_t) -> cusparseStatus_t {
    if handle.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    let zluda_handle = unsafe { Box::from_raw(handle as *mut Handle) };
    to_cusparse_error(unsafe { hipsparseDestroy(zluda_handle.handle) })
}

fn get_handle(handle: cusparseHandle_t) -> Result<&'static Handle, cusparseError_t> {
    if handle.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    Ok(unsafe { &*(handle as *const Handle) })
}

pub(crate) fn set_stream(handle: cusparseHandle_t, stream: CUstream) -> cusparseStatus_t {
    let h = get_handle(handle)?;
    // CUstream and hipStream_t are both pointers, can safely transmute
    to_cusparse_error(unsafe { hipsparseSetStream(h.handle, mem::transmute(stream)) })
}

pub(crate) fn get_stream(handle: cusparseHandle_t, stream: *mut CUstream) -> cusparseStatus_t {
    if stream.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    let h = get_handle(handle)?;
    // CUstream and hipStream_t are both pointers, can safely transmute
    to_cusparse_error(unsafe { hipsparseGetStream(h.handle, stream as *mut _) })
}

pub(crate) fn set_pointer_mode(
    handle: cusparseHandle_t,
    mode: cusparsePointerMode_t,
) -> cusparseStatus_t {
    let h = get_handle(handle)?;
    to_cusparse_error(unsafe {
        hipsparseSetPointerMode(h.handle, mem::transmute(mode))
    })
}

pub(crate) fn get_pointer_mode(
    handle: cusparseHandle_t,
    mode: *mut cusparsePointerMode_t,
) -> cusparseStatus_t {
    if mode.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    let h = get_handle(handle)?;
    to_cusparse_error(unsafe {
        hipsparseGetPointerMode(h.handle, mode as *mut hipsparsePointerMode_t)
    })
}

pub(crate) fn create_mat_descr(descr_a: *mut cusparseMatDescr_t) -> cusparseStatus_t {
    if descr_a.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    let mut hip_descr: hipsparseMatDescr_t = unsafe { mem::zeroed() };
    to_cusparse_error(unsafe { hipsparseCreateMatDescr(&mut hip_descr) })?;
    let mat_descr = Box::new(MatDescr { hip_descr });
    unsafe { *descr_a = Box::into_raw(mat_descr) as cusparseMatDescr_t };
    Ok(())
}

pub(crate) fn destroy_mat_descr(descr_a: cusparseMatDescr_t) -> cusparseStatus_t {
    if descr_a.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    let mat_descr = unsafe { Box::from_raw(descr_a as *mut MatDescr) };
    to_cusparse_error(unsafe { hipsparseDestroyMatDescr(mat_descr.hip_descr) })
}

fn get_mat_descr(descr_a: cusparseMatDescr_t) -> Option<&'static MatDescr> {
    if descr_a.is_null() {
        return None;
    }
    Some(unsafe { &*(descr_a as *const MatDescr) })
}

pub(crate) fn set_mat_type(
    descr_a: cusparseMatDescr_t,
    type_: cusparseMatrixType_t,
) -> cusparseStatus_t {
    let mat_descr = get_mat_descr(descr_a).ok_or(cusparseError_t::INVALID_VALUE)?;
    to_cusparse_error(unsafe { hipsparseSetMatType(mat_descr.hip_descr, mem::transmute(type_)) })
}

pub(crate) fn set_mat_fill_mode(
    descr_a: cusparseMatDescr_t,
    fill_mode: cusparseFillMode_t,
) -> cusparseStatus_t {
    let mat_descr = get_mat_descr(descr_a).ok_or(cusparseError_t::INVALID_VALUE)?;
    to_cusparse_error(unsafe { hipsparseSetMatFillMode(mat_descr.hip_descr, mem::transmute(fill_mode)) })
}

pub(crate) fn set_mat_diag_type(
    descr_a: cusparseMatDescr_t,
    diag_type: cusparseDiagType_t,
) -> cusparseStatus_t {
    let mat_descr = get_mat_descr(descr_a).ok_or(cusparseError_t::INVALID_VALUE)?;
    to_cusparse_error(unsafe { hipsparseSetMatDiagType(mat_descr.hip_descr, mem::transmute(diag_type)) })
}

pub(crate) fn set_mat_index_base(
    descr_a: cusparseMatDescr_t,
    base: cusparseIndexBase_t,
) -> cusparseStatus_t {
    let mat_descr = get_mat_descr(descr_a).ok_or(cusparseError_t::INVALID_VALUE)?;
    to_cusparse_error(unsafe { hipsparseSetMatIndexBase(mat_descr.hip_descr, mem::transmute(base)) })
}

pub(crate) fn get_error_name(status: cusparseStatus_t) -> *const ::core::ffi::c_char {
    match status {
        Ok(()) => "CUSPARSE_STATUS_SUCCESS\0",
        Err(cusparseError_t::NOT_INITIALIZED) => "CUSPARSE_STATUS_NOT_INITIALIZED\0",
        Err(cusparseError_t::ALLOC_FAILED) => "CUSPARSE_STATUS_ALLOC_FAILED\0",
        Err(cusparseError_t::INVALID_VALUE) => "CUSPARSE_STATUS_INVALID_VALUE\0",
        Err(cusparseError_t::ARCH_MISMATCH) => "CUSPARSE_STATUS_ARCH_MISMATCH\0",
        Err(cusparseError_t::MAPPING_ERROR) => "CUSPARSE_STATUS_MAPPING_ERROR\0",
        Err(cusparseError_t::EXECUTION_FAILED) => "CUSPARSE_STATUS_EXECUTION_FAILED\0",
        Err(cusparseError_t::INTERNAL_ERROR) => "CUSPARSE_STATUS_INTERNAL_ERROR\0",
        Err(cusparseError_t::MATRIX_TYPE_NOT_SUPPORTED) => "CUSPARSE_STATUS_MATRIX_TYPE_NOT_SUPPORTED\0",
        Err(cusparseError_t::ZERO_PIVOT) => "CUSPARSE_STATUS_ZERO_PIVOT\0",
        Err(cusparseError_t::NOT_SUPPORTED) => "CUSPARSE_STATUS_NOT_SUPPORTED\0",
        Err(cusparseError_t::INSUFFICIENT_RESOURCES) => "CUSPARSE_STATUS_INSUFFICIENT_RESOURCES\0",
        _ => "CUSPARSE_STATUS_UNKNOWN\0",
    }
    .as_ptr() as *const ::core::ffi::c_char
}

pub(crate) fn get_error_string(status: cusparseStatus_t) -> *const ::core::ffi::c_char {
    get_error_name(status)
}

pub(crate) fn get_mat_type(descr_a: cusparseMatDescr_t) -> cusparseMatrixType_t {
    if let Some(mat_descr) = get_mat_descr(descr_a) {
        unsafe { mem::transmute(hipsparseGetMatType(mat_descr.hip_descr)) }
    } else {
        cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_GENERAL
    }
}

pub(crate) fn get_mat_fill_mode(descr_a: cusparseMatDescr_t) -> cusparseFillMode_t {
    if let Some(mat_descr) = get_mat_descr(descr_a) {
        unsafe { mem::transmute(hipsparseGetMatFillMode(mat_descr.hip_descr)) }
    } else {
        cusparseFillMode_t::CUSPARSE_FILL_MODE_LOWER
    }
}

pub(crate) fn get_mat_diag_type(descr_a: cusparseMatDescr_t) -> cusparseDiagType_t {
    if let Some(mat_descr) = get_mat_descr(descr_a) {
        unsafe { mem::transmute(hipsparseGetMatDiagType(mat_descr.hip_descr)) }
    } else {
        cusparseDiagType_t::CUSPARSE_DIAG_TYPE_NON_UNIT
    }
}

pub(crate) fn get_mat_index_base(descr_a: cusparseMatDescr_t) -> cusparseIndexBase_t {
    if let Some(mat_descr) = get_mat_descr(descr_a) {
        unsafe { mem::transmute(hipsparseGetMatIndexBase(mat_descr.hip_descr)) }
    } else {
        cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ZERO
    }
}

// Generic Sparse API - Sparse Vector Descriptor
pub(crate) fn create_sp_vec(
    sp_vec_descr: *mut cusparseSpVecDescr_t,
    size: i64,
    nnz: i64,
    indices: *mut ::core::ffi::c_void,
    values: *mut ::core::ffi::c_void,
    idx_type: cusparseIndexType_t,
    idx_base: cusparseIndexBase_t,
    value_type: cuda_types::cuda::cudaDataType,
) -> cusparseStatus_t {
    if sp_vec_descr.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    // hipsparseSpVecDescr_t is a newtype around *mut c_void
    // cusparseSpVecDescr_t is a struct pointer
    // The hipSPARSE function writes to our pointer location
    let mut hip_descr: hipsparseSpVecDescr_t = hipsparseSpVecDescr_t(std::ptr::null_mut());
    to_cusparse_error(unsafe {
        hipsparseCreateSpVec(
            &mut hip_descr,
            size,
            nnz,
            indices,
            values,
            mem::transmute(idx_type),
            mem::transmute(idx_base),
            mem::transmute(value_type),
        )
    })?;
    // Store the hip descriptor pointer directly as the cusparse handle
    unsafe { *(sp_vec_descr as *mut *mut ::core::ffi::c_void) = hip_descr.0 };
    Ok(())
}

pub(crate) fn destroy_sp_vec(sp_vec_descr: cusparseConstSpVecDescr_t) -> cusparseStatus_t {
    let hip_descr = hipsparseConstSpVecDescr_t(sp_vec_descr as *const ::core::ffi::c_void);
    to_cusparse_error(unsafe { hipsparseDestroySpVec(hip_descr) })
}

// Generic Sparse API - Dense Vector Descriptor
pub(crate) fn create_dn_vec(
    dn_vec_descr: *mut cusparseDnVecDescr_t,
    size: i64,
    values: *mut ::core::ffi::c_void,
    value_type: cuda_types::cuda::cudaDataType,
) -> cusparseStatus_t {
    if dn_vec_descr.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    let mut hip_descr: hipsparseDnVecDescr_t = hipsparseDnVecDescr_t(std::ptr::null_mut());
    to_cusparse_error(unsafe {
        hipsparseCreateDnVec(
            &mut hip_descr,
            size,
            values,
            mem::transmute(value_type),
        )
    })?;
    unsafe { *(dn_vec_descr as *mut *mut ::core::ffi::c_void) = hip_descr.0 };
    Ok(())
}

pub(crate) fn destroy_dn_vec(dn_vec_descr: cusparseConstDnVecDescr_t) -> cusparseStatus_t {
    let hip_descr = hipsparseConstDnVecDescr_t(dn_vec_descr as *const ::core::ffi::c_void);
    to_cusparse_error(unsafe { hipsparseDestroyDnVec(hip_descr) })
}

// Generic Sparse API - Dense Matrix Descriptor
pub(crate) fn create_dn_mat(
    dn_mat_descr: *mut cusparseDnMatDescr_t,
    rows: i64,
    cols: i64,
    ld: i64,
    values: *mut ::core::ffi::c_void,
    value_type: cuda_types::cuda::cudaDataType,
    order: cusparseOrder_t,
) -> cusparseStatus_t {
    if dn_mat_descr.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    let mut hip_descr: hipsparseDnMatDescr_t = hipsparseDnMatDescr_t(std::ptr::null_mut());
    to_cusparse_error(unsafe {
        hipsparseCreateDnMat(
            &mut hip_descr,
            rows,
            cols,
            ld,
            values,
            mem::transmute(value_type),
            mem::transmute(order),
        )
    })?;
    unsafe { *(dn_mat_descr as *mut *mut ::core::ffi::c_void) = hip_descr.0 };
    Ok(())
}

pub(crate) fn destroy_dn_mat(dn_mat_descr: cusparseConstDnMatDescr_t) -> cusparseStatus_t {
    let hip_descr = hipsparseConstDnMatDescr_t(dn_mat_descr as *const ::core::ffi::c_void);
    to_cusparse_error(unsafe { hipsparseDestroyDnMat(hip_descr) })
}

// Generic Sparse API - CSR Sparse Matrix Descriptor
pub(crate) fn create_csr(
    sp_mat_descr: *mut cusparseSpMatDescr_t,
    rows: i64,
    cols: i64,
    nnz: i64,
    csr_row_offsets: *mut ::core::ffi::c_void,
    csr_col_ind: *mut ::core::ffi::c_void,
    csr_values: *mut ::core::ffi::c_void,
    csr_row_offsets_type: cusparseIndexType_t,
    csr_col_ind_type: cusparseIndexType_t,
    idx_base: cusparseIndexBase_t,
    value_type: cuda_types::cuda::cudaDataType,
) -> cusparseStatus_t {
    if sp_mat_descr.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    let mut hip_descr: hipsparseSpMatDescr_t = hipsparseSpMatDescr_t(std::ptr::null_mut());
    to_cusparse_error(unsafe {
        hipsparseCreateCsr(
            &mut hip_descr,
            rows,
            cols,
            nnz,
            csr_row_offsets,
            csr_col_ind,
            csr_values,
            mem::transmute(csr_row_offsets_type),
            mem::transmute(csr_col_ind_type),
            mem::transmute(idx_base),
            mem::transmute(value_type),
        )
    })?;
    unsafe { *(sp_mat_descr as *mut *mut ::core::ffi::c_void) = hip_descr.0 };
    Ok(())
}

pub(crate) fn destroy_sp_mat(sp_mat_descr: cusparseConstSpMatDescr_t) -> cusparseStatus_t {
    let hip_descr = hipsparseConstSpMatDescr_t(sp_mat_descr as *const ::core::ffi::c_void);
    to_cusparse_error(unsafe { hipsparseDestroySpMat(hip_descr) })
}

// SpMV - Sparse Matrix-Vector Multiplication
pub(crate) fn sp_m_v_buffer_size(
    handle: cusparseHandle_t,
    op_a: cusparseOperation_t,
    alpha: *const ::core::ffi::c_void,
    mat_a: cusparseConstSpMatDescr_t,
    vec_x: cusparseConstDnVecDescr_t,
    beta: *const ::core::ffi::c_void,
    vec_y: cusparseDnVecDescr_t,
    compute_type: cuda_types::cuda::cudaDataType,
    alg: cusparseSpMVAlg_t,
    buffer_size: *mut usize,
) -> cusparseStatus_t {
    if buffer_size.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    let h = get_handle(handle)?;
    let hip_mat_a = hipsparseConstSpMatDescr_t(mat_a as *const ::core::ffi::c_void);
    let hip_vec_x = hipsparseConstDnVecDescr_t(vec_x as *const ::core::ffi::c_void);
    let hip_vec_y = hipsparseDnVecDescr_t(vec_y as *mut ::core::ffi::c_void);
    to_cusparse_error(unsafe {
        hipsparseSpMV_bufferSize(
            h.handle,
            mem::transmute(op_a),
            alpha,
            hip_mat_a,
            hip_vec_x,
            beta,
            hip_vec_y,
            mem::transmute(compute_type),
            mem::transmute(alg),
            buffer_size,
        )
    })
}

pub(crate) fn sp_m_v(
    handle: cusparseHandle_t,
    op_a: cusparseOperation_t,
    alpha: *const ::core::ffi::c_void,
    mat_a: cusparseConstSpMatDescr_t,
    vec_x: cusparseConstDnVecDescr_t,
    beta: *const ::core::ffi::c_void,
    vec_y: cusparseDnVecDescr_t,
    compute_type: cuda_types::cuda::cudaDataType,
    alg: cusparseSpMVAlg_t,
    external_buffer: *mut ::core::ffi::c_void,
) -> cusparseStatus_t {
    let h = get_handle(handle)?;
    let hip_mat_a = hipsparseConstSpMatDescr_t(mat_a as *const ::core::ffi::c_void);
    let hip_vec_x = hipsparseConstDnVecDescr_t(vec_x as *const ::core::ffi::c_void);
    let hip_vec_y = hipsparseDnVecDescr_t(vec_y as *mut ::core::ffi::c_void);
    to_cusparse_error(unsafe {
        hipsparseSpMV(
            h.handle,
            mem::transmute(op_a),
            alpha,
            hip_mat_a,
            hip_vec_x,
            beta,
            hip_vec_y,
            mem::transmute(compute_type),
            mem::transmute(alg),
            external_buffer,
        )
    })
}

// SpMM - Sparse Matrix-Matrix Multiplication
pub(crate) fn sp_m_m_buffer_size(
    handle: cusparseHandle_t,
    op_a: cusparseOperation_t,
    op_b: cusparseOperation_t,
    alpha: *const ::core::ffi::c_void,
    mat_a: cusparseConstSpMatDescr_t,
    mat_b: cusparseConstDnMatDescr_t,
    beta: *const ::core::ffi::c_void,
    mat_c: cusparseDnMatDescr_t,
    compute_type: cuda_types::cuda::cudaDataType,
    alg: cusparseSpMMAlg_t,
    buffer_size: *mut usize,
) -> cusparseStatus_t {
    if buffer_size.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    let h = get_handle(handle)?;
    let hip_mat_a = hipsparseConstSpMatDescr_t(mat_a as *const ::core::ffi::c_void);
    let hip_mat_b = hipsparseConstDnMatDescr_t(mat_b as *const ::core::ffi::c_void);
    let hip_mat_c = hipsparseDnMatDescr_t(mat_c as *mut ::core::ffi::c_void);
    to_cusparse_error(unsafe {
        hipsparseSpMM_bufferSize(
            h.handle,
            mem::transmute(op_a),
            mem::transmute(op_b),
            alpha,
            hip_mat_a,
            hip_mat_b,
            beta,
            hip_mat_c,
            mem::transmute(compute_type),
            mem::transmute(alg),
            buffer_size,
        )
    })
}

pub(crate) fn sp_m_m(
    handle: cusparseHandle_t,
    op_a: cusparseOperation_t,
    op_b: cusparseOperation_t,
    alpha: *const ::core::ffi::c_void,
    mat_a: cusparseConstSpMatDescr_t,
    mat_b: cusparseConstDnMatDescr_t,
    beta: *const ::core::ffi::c_void,
    mat_c: cusparseDnMatDescr_t,
    compute_type: cuda_types::cuda::cudaDataType,
    alg: cusparseSpMMAlg_t,
    external_buffer: *mut ::core::ffi::c_void,
) -> cusparseStatus_t {
    let h = get_handle(handle)?;
    let hip_mat_a = hipsparseConstSpMatDescr_t(mat_a as *const ::core::ffi::c_void);
    let hip_mat_b = hipsparseConstDnMatDescr_t(mat_b as *const ::core::ffi::c_void);
    let hip_mat_c = hipsparseDnMatDescr_t(mat_c as *mut ::core::ffi::c_void);
    to_cusparse_error(unsafe {
        hipsparseSpMM(
            h.handle,
            mem::transmute(op_a),
            mem::transmute(op_b),
            alpha,
            hip_mat_a,
            hip_mat_b,
            beta,
            hip_mat_c,
            mem::transmute(compute_type),
            mem::transmute(alg),
            external_buffer,
        )
    })
}
