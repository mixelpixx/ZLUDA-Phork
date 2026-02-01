use cuda_types::cusparse::*;

// Matrix descriptor structure to store matrix properties
#[repr(C)]
pub struct MatDescr {
    matrix_type: cusparseMatrixType_t,
    fill_mode: cusparseFillMode_t,
    diag_type: cusparseDiagType_t,
    index_base: cusparseIndexBase_t,
}

impl MatDescr {
    fn new() -> Self {
        Self {
            matrix_type: cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_GENERAL,
            fill_mode: cusparseFillMode_t::CUSPARSE_FILL_MODE_LOWER,
            diag_type: cusparseDiagType_t::CUSPARSE_DIAG_TYPE_NON_UNIT,
            index_base: cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ZERO,
        }
    }
}

pub(crate) fn create_mat_descr(
    descr_a: *mut cuda_types::cusparse::cusparseMatDescr_t,
) -> cusparseStatus_t {
    if descr_a.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    let mat_descr = Box::new(MatDescr::new());
    unsafe { *descr_a = Box::into_raw(mat_descr) as cuda_types::cusparse::cusparseMatDescr_t };
    Ok(())
}

pub(crate) fn destroy_mat_descr(
    descr_a: cuda_types::cusparse::cusparseMatDescr_t,
) -> cusparseStatus_t {
    if descr_a.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    unsafe { drop(Box::from_raw(descr_a as *mut MatDescr)) };
    Ok(())
}

pub(crate) fn set_mat_type(
    descr_a: cuda_types::cusparse::cusparseMatDescr_t,
    type_: cuda_types::cusparse::cusparseMatrixType_t,
) -> cusparseStatus_t {
    if descr_a.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    unsafe { (*(descr_a as *mut MatDescr)).matrix_type = type_ };
    Ok(())
}

pub(crate) fn set_mat_fill_mode(
    descr_a: cuda_types::cusparse::cusparseMatDescr_t,
    fill_mode: cuda_types::cusparse::cusparseFillMode_t,
) -> cusparseStatus_t {
    if descr_a.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    unsafe { (*(descr_a as *mut MatDescr)).fill_mode = fill_mode };
    Ok(())
}

pub(crate) fn set_mat_diag_type(
    descr_a: cuda_types::cusparse::cusparseMatDescr_t,
    diag_type: cuda_types::cusparse::cusparseDiagType_t,
) -> cusparseStatus_t {
    if descr_a.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    unsafe { (*(descr_a as *mut MatDescr)).diag_type = diag_type };
    Ok(())
}

pub(crate) fn set_mat_index_base(
    descr_a: cuda_types::cusparse::cusparseMatDescr_t,
    base: cuda_types::cusparse::cusparseIndexBase_t,
) -> cusparseStatus_t {
    if descr_a.is_null() {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    unsafe { (*(descr_a as *mut MatDescr)).index_base = base };
    Ok(())
}

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cusparseStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cusparseStatus_t {
    cusparseStatus_t::ERROR_NOT_SUPPORTED
}

pub(crate) fn get_error_name(
    status: cuda_types::cusparse::cusparseStatus_t,
) -> *const ::core::ffi::c_char {
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

pub(crate) fn get_error_string(
    status: cuda_types::cusparse::cusparseStatus_t,
) -> *const ::core::ffi::c_char {
    // For cuSPARSE, error string is the same as error name
    get_error_name(status)
}

pub(crate) fn get_mat_type(
    descr_a: cuda_types::cusparse::cusparseMatDescr_t,
) -> cuda_types::cusparse::cusparseMatrixType_t {
    if descr_a.is_null() {
        return cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_GENERAL;
    }
    unsafe { (*(descr_a as *const MatDescr)).matrix_type }
}

pub(crate) fn get_mat_fill_mode(
    descr_a: cuda_types::cusparse::cusparseMatDescr_t,
) -> cuda_types::cusparse::cusparseFillMode_t {
    if descr_a.is_null() {
        return cusparseFillMode_t::CUSPARSE_FILL_MODE_LOWER;
    }
    unsafe { (*(descr_a as *const MatDescr)).fill_mode }
}

pub(crate) fn get_mat_diag_type(
    descr_a: cuda_types::cusparse::cusparseMatDescr_t,
) -> cuda_types::cusparse::cusparseDiagType_t {
    if descr_a.is_null() {
        return cusparseDiagType_t::CUSPARSE_DIAG_TYPE_NON_UNIT;
    }
    unsafe { (*(descr_a as *const MatDescr)).diag_type }
}

pub(crate) fn get_mat_index_base(
    descr_a: cuda_types::cusparse::cusparseMatDescr_t,
) -> cuda_types::cusparse::cusparseIndexBase_t {
    if descr_a.is_null() {
        return cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ZERO;
    }
    unsafe { (*(descr_a as *const MatDescr)).index_base }
}
