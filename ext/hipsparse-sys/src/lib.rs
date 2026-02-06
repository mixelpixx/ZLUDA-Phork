// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct float2 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct double2 {
    pub x: f64,
    pub y: f64,
}
pub type hipFloatComplex = float2;
pub type hipDoubleComplex = double2;
pub type hipComplex = hipFloatComplex;
impl hipDataType {
    pub const HIP_R_32F: hipDataType = hipDataType(0);
}
impl hipDataType {
    pub const HIP_R_64F: hipDataType = hipDataType(1);
}
impl hipDataType {
    pub const HIP_R_16F: hipDataType = hipDataType(2);
}
impl hipDataType {
    pub const HIP_R_8I: hipDataType = hipDataType(3);
}
impl hipDataType {
    pub const HIP_C_32F: hipDataType = hipDataType(4);
}
impl hipDataType {
    pub const HIP_C_64F: hipDataType = hipDataType(5);
}
impl hipDataType {
    pub const HIP_C_16F: hipDataType = hipDataType(6);
}
impl hipDataType {
    pub const HIP_C_8I: hipDataType = hipDataType(7);
}
impl hipDataType {
    pub const HIP_R_8U: hipDataType = hipDataType(8);
}
impl hipDataType {
    pub const HIP_C_8U: hipDataType = hipDataType(9);
}
impl hipDataType {
    pub const HIP_R_32I: hipDataType = hipDataType(10);
}
impl hipDataType {
    pub const HIP_C_32I: hipDataType = hipDataType(11);
}
impl hipDataType {
    pub const HIP_R_32U: hipDataType = hipDataType(12);
}
impl hipDataType {
    pub const HIP_C_32U: hipDataType = hipDataType(13);
}
impl hipDataType {
    pub const HIP_R_16BF: hipDataType = hipDataType(14);
}
impl hipDataType {
    pub const HIP_C_16BF: hipDataType = hipDataType(15);
}
impl hipDataType {
    pub const HIP_R_4I: hipDataType = hipDataType(16);
}
impl hipDataType {
    pub const HIP_C_4I: hipDataType = hipDataType(17);
}
impl hipDataType {
    pub const HIP_R_4U: hipDataType = hipDataType(18);
}
impl hipDataType {
    pub const HIP_C_4U: hipDataType = hipDataType(19);
}
impl hipDataType {
    pub const HIP_R_16I: hipDataType = hipDataType(20);
}
impl hipDataType {
    pub const HIP_C_16I: hipDataType = hipDataType(21);
}
impl hipDataType {
    pub const HIP_R_16U: hipDataType = hipDataType(22);
}
impl hipDataType {
    pub const HIP_C_16U: hipDataType = hipDataType(23);
}
impl hipDataType {
    pub const HIP_R_64I: hipDataType = hipDataType(24);
}
impl hipDataType {
    pub const HIP_C_64I: hipDataType = hipDataType(25);
}
impl hipDataType {
    pub const HIP_R_64U: hipDataType = hipDataType(26);
}
impl hipDataType {
    pub const HIP_C_64U: hipDataType = hipDataType(27);
}
impl hipDataType {
    pub const HIP_R_8F_E4M3_FNUZ: hipDataType = hipDataType(1000);
}
impl hipDataType {
    pub const HIP_R_8F_E5M2_FNUZ: hipDataType = hipDataType(1001);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipDataType(pub ::core::ffi::c_uint);
/// \cond DO_NOT_DOCUMENT
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bsrsv2Info {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bsrsm2Info {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bsrilu02Info {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bsric02Info {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csrsv2Info {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csrsm2Info {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csrilu02Info {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csric02Info {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csrgemm2Info {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pruneInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csru2csrInfo {
    _unused: [u8; 0],
}
/** \ingroup types_module
  \brief Handle to the hipSPARSE library context queue.

  \details
  The hipSPARSE handle is a structure holding the hipSPARSE library context. It must
  be initialized using hipsparseCreate() and the returned handle must be passed to all
  subsequent library function calls. It should be destroyed at the end using
  hipsparseDestroy().*/
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseHandle_t(pub *mut ::core::ffi::c_void);
/** \ingroup types_module
  \brief Descriptor of the matrix.

  \details
  The hipSPARSE matrix descriptor is a structure holding all properties of a matrix.
  It must be initialized using hipsparseCreateMatDescr() and the returned descriptor
  must be passed to all subsequent library calls that involve the matrix. It should be
  destroyed at the end using hipsparseDestroyMatDescr().*/
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseMatDescr_t(pub *mut ::core::ffi::c_void);
/** \ingroup types_module
  \brief HYB matrix storage format.

  \details
  The hipSPARSE HYB matrix structure holds the HYB matrix. It must be initialized using
  hipsparseCreateHybMat() and the returned HYB matrix must be passed to all subsequent
  library calls that involve the matrix. It should be destroyed at the end using
  hipsparseDestroyHybMat().*/
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseHybMat_t(pub *mut ::core::ffi::c_void);
/** \ingroup types_module
  \brief Pointer type to opaque structure holding coloring info.

  \details
  The hipSPARSE ColorInfo structure holds the coloring information. It must be
  initialized using hipsparseCreateColorInfo() and the returned structure must be
  passed to all subsequent library calls that involve the coloring. It should be
  destroyed at the end using hipsparseDestroyColorInfo().*/
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseColorInfo_t(pub *mut ::core::ffi::c_void);
/** \ingroup types_module
  \brief Pointer type to opaque structure holding bsrsv2 info.

  \details
  The hipSPARSE bsrsv2 structure holds the information used by hipsparseXbsrsv2_zeroPivot(),
  hipsparseXbsrsv2_bufferSize(), hipsparseXbsrsv2_bufferSizeExt(), hipsparseXbsrsv2_analysis(),
  and hipsparseXbsrsv2_solve(). It must be initialized using hipsparseCreateBsrsv2Info() and
  the returned structure must be passed to all subsequent library calls that involve bsrsv2.
  It should be destroyed at the end using hipsparseDestroyBsrsv2Info().*/
pub type bsrsv2Info_t = *mut bsrsv2Info;
/** \ingroup types_module
  \brief Pointer type to opaque structure holding bsrsm2 info.

  \details
  The hipSPARSE bsrsm2 structure holds the information used by hipsparseXbsrsm2_zeroPivot(), hipsparseXbsrsm2_bufferSize(),
  hipsparseXbsrsm2_analysis(), and hipsparseXbsrsm2_solve(). It must be initialized using
  hipsparseCreateBsrsm2Info() and the returned structure must be
  passed to all subsequent library calls that involve bsrsm2. It should be
  destroyed at the end using hipsparseDestroyBsrsm2Info().*/
pub type bsrsm2Info_t = *mut bsrsm2Info;
/** \ingroup types_module
  \brief Pointer type to opaque structure holding bsrilu02 info.

  \details
  The hipSPARSE bsrilu02 structure holds the information used by hipsparseXbsrilu02_zeroPivot(),
  hipsparseXbsrilu02_numericBoost(), hipsparseXbsrilu02_bufferSize(), hipsparseXbsrilu02_analysis(),
  and hipsparseXbsrilu02(). It must be initialized using hipsparseCreateBsrilu02Info() and the
  returned structure must be passed to all subsequent library calls that involve bsrilu02. It should be
  destroyed at the end using hipsparseDestroyBsrilu02Info().*/
pub type bsrilu02Info_t = *mut bsrilu02Info;
/** \ingroup types_module
  \brief Pointer type to opaque structure holding bsric02 info.

  \details
  The hipSPARSE bsric02 structure holds the information used by hipsparseXbsric02_zeroPivot(), hipsparseXbsric02_bufferSize(),
  hipsparseXbsric02_analysis(), and hipsparseXbsric02(). It must be initialized using
  hipsparseCreateBsric02Info() and the returned structure must be
  passed to all subsequent library calls that involve bsric02. It should be
  destroyed at the end using hipsparseDestroyBsric02Info().*/
pub type bsric02Info_t = *mut bsric02Info;
/** \ingroup types_module
  \brief Pointer type to opaque structure holding csrsv2 info.

  \details
  The hipSPARSE csrsv2 structure holds the information used by hipsparseXcsrsv2_zeroPivot(), hipsparseXcsrsv2_bufferSize(),
  hipsparseXcsrsv2_analysis(), and hipsparseXcsrsv2(). It must be initialized using
  hipsparseCreateCsrsv2Info() and the returned structure must be
  passed to all subsequent library calls that involve csrsv2. It should be
  destroyed at the end using hipsparseDestroyCsrsv2Info().*/
pub type csrsv2Info_t = *mut csrsv2Info;
/** \ingroup types_module
  \brief Pointer type to opaque structure holding csrsm2 info.

  \details
  The hipSPARSE csrsm2 structure holds the information used by hipsparseXcsrsm2_zeroPivot(), hipsparseXcsrsm2_bufferSize(),
  hipsparseXcsrsm2_analysis(), and hipsparseXcsrsm2(). It must be initialized using
  hipsparseCreateCsrsm2Info() and the returned structure must be
  passed to all subsequent library calls that involve csrsm2. It should be
  destroyed at the end using hipsparseDestroyCsrsm2Info().*/
pub type csrsm2Info_t = *mut csrsm2Info;
/** \ingroup types_module
  \brief Pointer type to opaque structure holding csrilu02 info.

  \details
  The hipSPARSE csrilu02 structure holds the information used by hipsparseXcsrilu02_zeroPivot(),
  hipsparseXcsrilu02_numericBoost(), hipsparseXcsrilu02_bufferSize(), hipsparseXcsrilu02_analysis(),
  and hipsparseXcsrilu02(). It must be initialized using hipsparseCreateCsrilu02Info() and the
  returned structure must be passed to all subsequent library calls that involve csrilu02. It should be
  destroyed at the end using hipsparseDestroyCsrilu02Info().*/
pub type csrilu02Info_t = *mut csrilu02Info;
/** \ingroup types_module
  \brief Pointer type to opaque structure holding csric02 info.

  \details
  The hipSPARSE csric02 structure holds the information used by hipsparseXcsric02_zeroPivot(),
  hipsparseXcsric02_bufferSize(), hipsparseXcsric02_analysis(), and hipsparseXcsric02(). It must be
  initialized using hipsparseCreateCsric02Info() and the returned structure must be passed to all
  subsequent library calls that involve csric02. It should be destroyed at the end using
  hipsparseDestroyCsric02Info().*/
pub type csric02Info_t = *mut csric02Info;
/** \ingroup types_module
  \brief Pointer type to opaque structure holding csrgemm2 info.

  \details
  The hipSPARSE csrgemm2 structure holds the information used by hipsparseXcsrgemm2_bufferSizeExt(),
  hipsparseXcsrgemm2Nnz(), and hipsparseXcsrgemm2(). It must be initialized using
  hipsparseCreateCsrgemm2Info() and the returned structure must be passed to all subsequent
  library calls that involve csrgemm2. It should be destroyed at the end using
  hipsparseDestroyCsrgemm2Info().*/
pub type csrgemm2Info_t = *mut csrgemm2Info;
/** \ingroup types_module
  \brief Pointer type to opaque structure holding prune info.

  \details
  The hipSPARSE prune structure holds the information used by hipsparseXpruneDense2csrByPercentage_bufferSize(),
  hipsparseXpruneDense2csrByPercentage_bufferSizeExt(), hipsparseXpruneCsr2csrByPercentage_bufferSize(),
  hipsparseXpruneCsr2csrByPercentage_bufferSizeExt(), hipsparseXpruneDense2csrNnzByPercentage(),
  hipsparseXpruneCsr2csrNnzByPercentage(), hipsparseXpruneDense2csrByPercentage(), and
  hipsparseXpruneCsr2csrByPercentage(). It must be initialized using hipsparseCreatePruneInfo() and the
  returned structure must be passed to all subsequent library calls that involve prune. It should be
  destroyed at the end using hipsparseDestroyPruneInfo().*/
pub type pruneInfo_t = *mut pruneInfo;
/** \ingroup types_module
  \brief Pointer type to opaque structure holding csru2csr info.

  \details
  The hipSPARSE csru2csr structure holds the information used by hipsparseXcsru2csr_bufferSizeExt(),
  hipsparseXcsru2csr(), and hipsparseXcsr2csru(). It must be initialized using hipsparseCreateCsru2csrInfo()
  and the returned structure must be passed to all subsequent library calls that involve csru2csr. It should be
  destroyed at the end using hipsparseDestroyCsru2csrInfo().*/
pub type csru2csrInfo_t = *mut csru2csrInfo;
impl hipsparsePointerMode_t {
    ///< Scalar pointers are in host memory
    pub const HIPSPARSE_POINTER_MODE_HOST: hipsparsePointerMode_t = hipsparsePointerMode_t(
        0,
    );
}
impl hipsparsePointerMode_t {
    ///< Scalar pointers are in device memory
    pub const HIPSPARSE_POINTER_MODE_DEVICE: hipsparsePointerMode_t = hipsparsePointerMode_t(
        1,
    );
}
#[repr(transparent)]
/** \ingroup types_module
  \brief Indicates if the pointer is device pointer or host pointer.

  \details
  The \ref hipsparsePointerMode_t indicates whether scalar values are passed by
  reference on the host or device. The \ref hipsparsePointerMode_t can be changed by
  hipsparseSetPointerMode(). The currently used pointer mode can be obtained by
  hipsparseGetPointerMode().*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparsePointerMode_t(pub ::core::ffi::c_uint);
impl hipsparseAction_t {
    ///< Operate only on indices
    pub const HIPSPARSE_ACTION_SYMBOLIC: hipsparseAction_t = hipsparseAction_t(0);
}
impl hipsparseAction_t {
    ///< Operate on data and indices
    pub const HIPSPARSE_ACTION_NUMERIC: hipsparseAction_t = hipsparseAction_t(1);
}
#[repr(transparent)]
/** \ingroup types_module
  \brief Specify where the operation is performed on.

  \details
  The \ref hipsparseAction_t indicates whether the operation is performed on the full
  matrix, or only on the sparsity pattern of the matrix.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseAction_t(pub ::core::ffi::c_uint);
impl hipsparseMatrixType_t {
    ///< General matrix type
    pub const HIPSPARSE_MATRIX_TYPE_GENERAL: hipsparseMatrixType_t = hipsparseMatrixType_t(
        0,
    );
}
impl hipsparseMatrixType_t {
    ///< Symmetric matrix type
    pub const HIPSPARSE_MATRIX_TYPE_SYMMETRIC: hipsparseMatrixType_t = hipsparseMatrixType_t(
        1,
    );
}
impl hipsparseMatrixType_t {
    ///< Hermitian matrix type
    pub const HIPSPARSE_MATRIX_TYPE_HERMITIAN: hipsparseMatrixType_t = hipsparseMatrixType_t(
        2,
    );
}
impl hipsparseMatrixType_t {
    ///< Triangular matrix type
    pub const HIPSPARSE_MATRIX_TYPE_TRIANGULAR: hipsparseMatrixType_t = hipsparseMatrixType_t(
        3,
    );
}
#[repr(transparent)]
/** \ingroup types_module
  \brief Specify the matrix type.

  \details
  The \ref hipsparseMatrixType_t indices the type of a matrix. For a given
  \ref hipsparseMatDescr_t, the \ref hipsparseMatrixType_t can be set using
  hipsparseSetMatType(). The current \ref hipsparseMatrixType_t of a matrix can be
  obtained by hipsparseGetMatType().*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseMatrixType_t(pub ::core::ffi::c_uint);
impl hipsparseFillMode_t {
    ///< Lower triangular part is stored
    pub const HIPSPARSE_FILL_MODE_LOWER: hipsparseFillMode_t = hipsparseFillMode_t(0);
}
impl hipsparseFillMode_t {
    ///< Upper triangular part is stored
    pub const HIPSPARSE_FILL_MODE_UPPER: hipsparseFillMode_t = hipsparseFillMode_t(1);
}
#[repr(transparent)]
/** \ingroup types_module
  \brief Specify the matrix fill mode.

  \details
  The \ref hipsparseFillMode_t indicates whether the lower or the upper part is stored
  in a sparse triangular matrix. For a given \ref hipsparseMatDescr_t, the
  \ref hipsparseFillMode_t can be set using hipsparseSetMatFillMode(). The current
  \ref hipsparseFillMode_t of a matrix can be obtained by hipsparseGetMatFillMode().*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseFillMode_t(pub ::core::ffi::c_uint);
impl hipsparseDiagType_t {
    ///< Diagonal entries are non-unity
    pub const HIPSPARSE_DIAG_TYPE_NON_UNIT: hipsparseDiagType_t = hipsparseDiagType_t(0);
}
impl hipsparseDiagType_t {
    ///< Diagonal entries are unity
    pub const HIPSPARSE_DIAG_TYPE_UNIT: hipsparseDiagType_t = hipsparseDiagType_t(1);
}
#[repr(transparent)]
/** \ingroup types_module
  \brief Indicates if the diagonal entries are unity.

  \details
  The \ref hipsparseDiagType_t indicates whether the diagonal entries of a matrix are
  unity or not. If \ref HIPSPARSE_DIAG_TYPE_UNIT is specified, all present diagonal
  values will be ignored. For a given \ref hipsparseMatDescr_t, the
  \ref hipsparseDiagType_t can be set using hipsparseSetMatDiagType(). The current
  \ref hipsparseDiagType_t of a matrix can be obtained by hipsparseGetMatDiagType().*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseDiagType_t(pub ::core::ffi::c_uint);
impl hipsparseIndexBase_t {
    ///< Zero based indexing
    pub const HIPSPARSE_INDEX_BASE_ZERO: hipsparseIndexBase_t = hipsparseIndexBase_t(0);
}
impl hipsparseIndexBase_t {
    ///< One based indexing
    pub const HIPSPARSE_INDEX_BASE_ONE: hipsparseIndexBase_t = hipsparseIndexBase_t(1);
}
#[repr(transparent)]
/** \ingroup types_module
  \brief Specify the matrix index base.

  \details
  The \ref hipsparseIndexBase_t indicates the index base of the indices. For a
  given \ref hipsparseMatDescr_t, the \ref hipsparseIndexBase_t can be set using
  hipsparseSetMatIndexBase(). The current \ref hipsparseIndexBase_t of a matrix
  can be obtained by hipsparseGetMatIndexBase().*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseIndexBase_t(pub ::core::ffi::c_uint);
impl hipsparseOperation_t {
    ///< Operate with matrix
    pub const HIPSPARSE_OPERATION_NON_TRANSPOSE: hipsparseOperation_t = hipsparseOperation_t(
        0,
    );
}
impl hipsparseOperation_t {
    ///< Operate with transpose
    pub const HIPSPARSE_OPERATION_TRANSPOSE: hipsparseOperation_t = hipsparseOperation_t(
        1,
    );
}
impl hipsparseOperation_t {
    ///< Operate with conj. transpose
    pub const HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE: hipsparseOperation_t = hipsparseOperation_t(
        2,
    );
}
#[repr(transparent)]
/** \ingroup types_module
  \brief Specify whether the matrix is to be transposed or not.

  \details
  The \ref hipsparseOperation_t indicates the operation performed with the given matrix.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseOperation_t(pub ::core::ffi::c_uint);
impl hipsparseHybPartition_t {
    ///< Automatically decide on ELL nnz per row
    pub const HIPSPARSE_HYB_PARTITION_AUTO: hipsparseHybPartition_t = hipsparseHybPartition_t(
        0,
    );
}
impl hipsparseHybPartition_t {
    ///< User given ELL nnz per row
    pub const HIPSPARSE_HYB_PARTITION_USER: hipsparseHybPartition_t = hipsparseHybPartition_t(
        1,
    );
}
impl hipsparseHybPartition_t {
    ///< Max ELL nnz per row, no COO part
    pub const HIPSPARSE_HYB_PARTITION_MAX: hipsparseHybPartition_t = hipsparseHybPartition_t(
        2,
    );
}
#[repr(transparent)]
/** \ingroup types_module
  \brief HYB matrix partitioning type.

  \details
  The \ref hipsparseHybPartition_t type indicates how the hybrid format partitioning
  between COO and ELL storage formats is performed.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseHybPartition_t(pub ::core::ffi::c_uint);
impl hipsparseSolvePolicy_t {
    ///< No level information generated
    pub const HIPSPARSE_SOLVE_POLICY_NO_LEVEL: hipsparseSolvePolicy_t = hipsparseSolvePolicy_t(
        0,
    );
}
impl hipsparseSolvePolicy_t {
    ///< Generate level information
    pub const HIPSPARSE_SOLVE_POLICY_USE_LEVEL: hipsparseSolvePolicy_t = hipsparseSolvePolicy_t(
        1,
    );
}
#[repr(transparent)]
/** \ingroup types_module
  \brief Specify policy in triangular solvers and factorizations.

  \details
  The \ref hipsparseSolvePolicy_t type indicates the solve policy for the triangular
  solve.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSolvePolicy_t(pub ::core::ffi::c_uint);
impl hipsparseSideMode_t {
    pub const HIPSPARSE_SIDE_LEFT: hipsparseSideMode_t = hipsparseSideMode_t(0);
}
impl hipsparseSideMode_t {
    pub const HIPSPARSE_SIDE_RIGHT: hipsparseSideMode_t = hipsparseSideMode_t(1);
}
#[repr(transparent)]
/// \cond DO_NOT_DOCUMENT
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSideMode_t(pub ::core::ffi::c_uint);
impl hipsparseDirection_t {
    ///< Parse the matrix by rows
    pub const HIPSPARSE_DIRECTION_ROW: hipsparseDirection_t = hipsparseDirection_t(0);
}
impl hipsparseDirection_t {
    ///< Parse the matrix by columns
    pub const HIPSPARSE_DIRECTION_COLUMN: hipsparseDirection_t = hipsparseDirection_t(1);
}
#[repr(transparent)]
/** \ingroup types_module
  \brief Specify the matrix direction.

  \details
  The \ref hipsparseDirection_t indicates whether a dense matrix should be parsed by
  rows or by columns, assuming column-major storage.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseDirection_t(pub ::core::ffi::c_uint);
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a hipsparse handle

  \details
  \p hipsparseCreate creates the hipSPARSE library context. It must be
  initialized before any other hipSPARSE API function is invoked and must be passed to
  all subsequent library function calls. The handle should be destroyed at the end
  using hipsparseDestroy().*/
    pub fn hipsparseCreate(handle: *mut hipsparseHandle_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a hipsparse handle

  \details
  \p hipsparseDestroy destroys the hipSPARSE library context and releases all
  resources used by the hipSPARSE library.*/
    pub fn hipsparseDestroy(handle: hipsparseHandle_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    /** \ingroup aux_module
  \brief Return the string representation of a hipSPARSE status's matching backend status enum name

  \details
  \p hipsparseGetErrorName takes a hipSPARSE status as input and first converts it to the matching backend
  status (either rocsparse_status or cusparseStatus_t). It then returns the string representation of this status
  enum name. If the status is not recognized, the function returns "Unrecognized status code".

  For example, hipsparseGetErrorName(HIPSPARSE_STATUS_SUCCESS) on a system with a rocSPARSE backend will
  return "rocsparse_status_success". On a system with a cuSPARSE backend this function would return
  "CUSPARSE_STATUS_SUCCESS".*/
    pub fn hipsparseGetErrorName(
        status: hipsparseStatus_t,
    ) -> *const ::core::ffi::c_char;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    /** \ingroup aux_module
  \brief Return the hipSPARSE status's matching backend status description as a string

  \details
  \p hipsparseGetErrorString takes a hipSPARSE status as input and first converts it to the matching backend
  status (either rocsparse_status or cusparseStatus_t). It then returns the string description of this status.
  If the status is not recognized, the function returns "Unrecognized status code".*/
    pub fn hipsparseGetErrorString(
        status: hipsparseStatus_t,
    ) -> *const ::core::ffi::c_char;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Get hipSPARSE version

  \details
  \p hipsparseGetVersion gets the hipSPARSE library version number.
  - patch = version % 100
  - minor = version / 100 % 1000
  - major = version / 100000*/
    pub fn hipsparseGetVersion(
        handle: hipsparseHandle_t,
        version: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Get hipSPARSE git revision

  \details
  \p hipsparseGetGitRevision gets the hipSPARSE library git commit revision (SHA-1).*/
    pub fn hipsparseGetGitRevision(
        handle: hipsparseHandle_t,
        rev: *mut ::core::ffi::c_char,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Specify user defined HIP stream

  \details
  \p hipsparseSetStream specifies the stream to be used by the hipSPARSE library
  context and all subsequent function calls.*/
    pub fn hipsparseSetStream(
        handle: hipsparseHandle_t,
        streamId: hip_runtime_sys::hipStream_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Get current stream from library context

  \details
  \p hipsparseGetStream gets the hipSPARSE library context stream which is currently
  used for all subsequent function calls.*/
    pub fn hipsparseGetStream(
        handle: hipsparseHandle_t,
        streamId: *mut hip_runtime_sys::hipStream_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Specify pointer mode

  \details
  \p hipsparseSetPointerMode specifies the pointer mode to be used by the hipSPARSE
  library context and all subsequent function calls. By default, all values are passed
  by reference on the host. Valid pointer modes are \ref HIPSPARSE_POINTER_MODE_HOST
  or \ref HIPSPARSE_POINTER_MODE_DEVICE.*/
    pub fn hipsparseSetPointerMode(
        handle: hipsparseHandle_t,
        mode: hipsparsePointerMode_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Get current pointer mode from library context

  \details
  \p hipsparseGetPointerMode gets the hipSPARSE library context pointer mode which
  is currently used for all subsequent function calls.*/
    pub fn hipsparseGetPointerMode(
        handle: hipsparseHandle_t,
        mode: *mut hipsparsePointerMode_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a matrix descriptor
  \details
  \p hipsparseCreateMatDescr creates a matrix descriptor. It initializes
  \ref hipsparseMatrixType_t to \ref HIPSPARSE_MATRIX_TYPE_GENERAL and
  \ref hipsparseIndexBase_t to \ref HIPSPARSE_INDEX_BASE_ZERO. It should be destroyed
  at the end using hipsparseDestroyMatDescr().*/
    pub fn hipsparseCreateMatDescr(
        descrA: *mut hipsparseMatDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a matrix descriptor

  \details
  \p hipsparseDestroyMatDescr destroys a matrix descriptor and releases all
  resources used by the descriptor.*/
    pub fn hipsparseDestroyMatDescr(descrA: hipsparseMatDescr_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Copy a matrix descriptor
  \details
  \p hipsparseCopyMatDescr copies a matrix descriptor. Both, source and destination
  matrix descriptors must be initialized prior to calling \p hipsparseCopyMatDescr.*/
    pub fn hipsparseCopyMatDescr(
        dest: hipsparseMatDescr_t,
        src: hipsparseMatDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Specify the matrix type of a matrix descriptor

  \details
  \p hipsparseSetMatType sets the matrix type of a matrix descriptor. Valid
  matrix types are \ref HIPSPARSE_MATRIX_TYPE_GENERAL,
  \ref HIPSPARSE_MATRIX_TYPE_SYMMETRIC, \ref HIPSPARSE_MATRIX_TYPE_HERMITIAN or
  \ref HIPSPARSE_MATRIX_TYPE_TRIANGULAR.*/
    pub fn hipsparseSetMatType(
        descrA: hipsparseMatDescr_t,
        type_: hipsparseMatrixType_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    /** \ingroup aux_module
  \brief Get the matrix type of a matrix descriptor

  \details
  \p hipsparseGetMatType returns the matrix type of a matrix descriptor.*/
    pub fn hipsparseGetMatType(descrA: hipsparseMatDescr_t) -> hipsparseMatrixType_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Specify the matrix fill mode of a matrix descriptor

  \details
  \p hipsparseSetMatFillMode sets the matrix fill mode of a matrix descriptor.
  Valid fill modes are \ref HIPSPARSE_FILL_MODE_LOWER or
  \ref HIPSPARSE_FILL_MODE_UPPER.*/
    pub fn hipsparseSetMatFillMode(
        descrA: hipsparseMatDescr_t,
        fillMode: hipsparseFillMode_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    /** \ingroup aux_module
  \brief Get the matrix fill mode of a matrix descriptor

  \details
  \p hipsparseGetMatFillMode returns the matrix fill mode of a matrix descriptor.*/
    pub fn hipsparseGetMatFillMode(descrA: hipsparseMatDescr_t) -> hipsparseFillMode_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Specify the matrix diagonal type of a matrix descriptor

  \details
  \p hipsparseSetMatDiagType sets the matrix diagonal type of a matrix
  descriptor. Valid diagonal types are \ref HIPSPARSE_DIAG_TYPE_UNIT or
  \ref HIPSPARSE_DIAG_TYPE_NON_UNIT.*/
    pub fn hipsparseSetMatDiagType(
        descrA: hipsparseMatDescr_t,
        diagType: hipsparseDiagType_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    /** \ingroup aux_module
  \brief Get the matrix diagonal type of a matrix descriptor

  \details
  \p hipsparseGetMatDiagType returns the matrix diagonal type of a matrix
  descriptor.*/
    pub fn hipsparseGetMatDiagType(descrA: hipsparseMatDescr_t) -> hipsparseDiagType_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Specify the index base of a matrix descriptor

  \details
  \p hipsparseSetMatIndexBase sets the index base of a matrix descriptor. Valid
  options are \ref HIPSPARSE_INDEX_BASE_ZERO or \ref HIPSPARSE_INDEX_BASE_ONE.*/
    pub fn hipsparseSetMatIndexBase(
        descrA: hipsparseMatDescr_t,
        base: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    /** \ingroup aux_module
  \brief Get the index base of a matrix descriptor

  \details
  \p hipsparseGetMatIndexBase returns the index base of a matrix descriptor.*/
    pub fn hipsparseGetMatIndexBase(descrA: hipsparseMatDescr_t) -> hipsparseIndexBase_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a \p HYB matrix structure

  \details
  \p hipsparseCreateHybMat creates a structure that holds the matrix in \p HYB
  storage format. It should be destroyed at the end using hipsparseDestroyHybMat().*/
    pub fn hipsparseCreateHybMat(hybA: *mut hipsparseHybMat_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a \p HYB matrix structure

  \details
  \p hipsparseDestroyHybMat destroys a \p HYB structure.*/
    pub fn hipsparseDestroyHybMat(hybA: hipsparseHybMat_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a bsrsv2 info structure

  \details
  \p hipsparseCreateBsrsv2Info creates a structure that holds the bsrsv2 info data
  that is gathered during the analysis routines available. It should be destroyed
  at the end using hipsparseDestroyBsrsv2Info().*/
    pub fn hipsparseCreateBsrsv2Info(info: *mut bsrsv2Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a bsrsv2 info structure

  \details
  \p hipsparseDestroyBsrsv2Info destroys a bsrsv2 info structure.*/
    pub fn hipsparseDestroyBsrsv2Info(info: bsrsv2Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a bsrsm2 info structure

  \details
  \p hipsparseCreateBsrsm2Info creates a structure that holds the bsrsm2 info data
  that is gathered during the analysis routines available. It should be destroyed
  at the end using hipsparseDestroyBsrsm2Info().*/
    pub fn hipsparseCreateBsrsm2Info(info: *mut bsrsm2Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a bsrsm2 info structure

  \details
  \p hipsparseDestroyBsrsm2Info destroys a bsrsm2 info structure.*/
    pub fn hipsparseDestroyBsrsm2Info(info: bsrsm2Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a bsrilu02 info structure

  \details
  \p hipsparseCreateBsrilu02Info creates a structure that holds the bsrilu02 info data
  that is gathered during the analysis routines available. It should be destroyed
  at the end using hipsparseDestroyBsrilu02Info().*/
    pub fn hipsparseCreateBsrilu02Info(info: *mut bsrilu02Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a bsrilu02 info structure

  \details
  \p hipsparseDestroyBsrilu02Info destroys a bsrilu02 info structure.*/
    pub fn hipsparseDestroyBsrilu02Info(info: bsrilu02Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a bsric02 info structure

  \details
  \p hipsparseCreateBsric02Info creates a structure that holds the bsric02 info data
  that is gathered during the analysis routines available. It should be destroyed
  at the end using hipsparseDestroyBsric02Info().*/
    pub fn hipsparseCreateBsric02Info(info: *mut bsric02Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a bsric02 info structure

  \details
  \p hipsparseDestroyBsric02Info destroys a bsric02 info structure.*/
    pub fn hipsparseDestroyBsric02Info(info: bsric02Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a csrsv2 info structure

  \details
  \p hipsparseCreateCsrsv2Info creates a structure that holds the csrsv2 info data
  that is gathered during the analysis routines available. It should be destroyed
  at the end using hipsparseDestroyCsrsv2Info().*/
    pub fn hipsparseCreateCsrsv2Info(info: *mut csrsv2Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a csrsv2 info structure

  \details
  \p hipsparseDestroyCsrsv2Info destroys a csrsv2 info structure.*/
    pub fn hipsparseDestroyCsrsv2Info(info: csrsv2Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a csrsm2 info structure

  \details
  \p hipsparseCreateCsrsm2Info creates a structure that holds the csrsm2 info data
  that is gathered during the analysis routines available. It should be destroyed
  at the end using hipsparseDestroyCsrsm2Info().*/
    pub fn hipsparseCreateCsrsm2Info(info: *mut csrsm2Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a csrsm2 info structure

  \details
  \p hipsparseDestroyCsrsm2Info destroys a csrsm2 info structure.*/
    pub fn hipsparseDestroyCsrsm2Info(info: csrsm2Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a csrilu02 info structure

  \details
  \p hipsparseCreateCsrilu02Info creates a structure that holds the csrilu02 info data
  that is gathered during the analysis routines available. It should be destroyed
  at the end using hipsparseDestroyCsrilu02Info().*/
    pub fn hipsparseCreateCsrilu02Info(info: *mut csrilu02Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a csrilu02 info structure

  \details
  \p hipsparseDestroyCsrilu02Info destroys a csrilu02 info structure.*/
    pub fn hipsparseDestroyCsrilu02Info(info: csrilu02Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a csric02 info structure

  \details
  \p hipsparseCreateCsric02Info creates a structure that holds the csric02 info data
  that is gathered during the analysis routines available. It should be destroyed
  at the end using hipsparseDestroyCsric02Info().*/
    pub fn hipsparseCreateCsric02Info(info: *mut csric02Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a csric02 info structure

  \details
  \p hipsparseDestroyCsric02Info destroys a csric02 info structure.*/
    pub fn hipsparseDestroyCsric02Info(info: csric02Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a csru2csr info structure

  \details
  \p hipsparseCreateCsru2csrInfo creates a structure that holds the csru2csr info data
  that is gathered during the analysis routines available. It should be destroyed
  at the end using hipsparseDestroyCsru2csrInfo().*/
    pub fn hipsparseCreateCsru2csrInfo(info: *mut csru2csrInfo_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a csru2csr info structure

  \details
  \p hipsparseDestroyCsru2csrInfo destroys a csru2csr info structure.*/
    pub fn hipsparseDestroyCsru2csrInfo(info: csru2csrInfo_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a color info structure

  \details
  \p hipsparseCreateColorInfo creates a structure that holds the color info data
  that is gathered during the analysis routines available. It should be destroyed
  at the end using hipsparseDestroyColorInfo().*/
    pub fn hipsparseCreateColorInfo(
        info: *mut hipsparseColorInfo_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a color info structure

  \details
  \p hipsparseDestroyColorInfo destroys a color info structure.*/
    pub fn hipsparseDestroyColorInfo(info: hipsparseColorInfo_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a csrgemm2 info structure

  \details
  \p hipsparseCreateCsrgemm2Info creates a structure that holds the csrgemm2 info data
  that is gathered during the analysis routines available. It should be destroyed
  at the end using hipsparseDestroyCsrgemm2Info().*/
    pub fn hipsparseCreateCsrgemm2Info(info: *mut csrgemm2Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a csrgemm2 info structure

  \details
  \p hipsparseDestroyCsrgemm2Info destroys a csrgemm2 info structure.*/
    pub fn hipsparseDestroyCsrgemm2Info(info: csrgemm2Info_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Create a prune info structure

  \details
  \p hipsparseCreatePruneInfo creates a structure that holds the prune info data
  that is gathered during the analysis routines available. It should be destroyed
  at the end using hipsparseDestroyPruneInfo().*/
    pub fn hipsparseCreatePruneInfo(info: *mut pruneInfo_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup aux_module
  \brief Destroy a prune info structure

  \details
  \p hipsparseDestroyPruneInfo destroys a prune info structure.*/
    pub fn hipsparseDestroyPruneInfo(info: pruneInfo_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level1_module\n  \\brief Scale a sparse vector and add it to a dense vector.\n\n  \\details\n  \\p hipsparseXaxpyi multiplies the sparse vector \\f$x\\f$ with scalar \\f$\\alpha\\f$ and\n  adds the result to the dense vector \\f$y\\f$, such that\n\n  \\f[\n      y := y + \\alpha \\cdot x\n  \\f]\n\n  \\code{.c}\n      for(i = 0; i < nnz; ++i)\n      {\n          y[x_ind[i]] = y[x_ind[i]] + alpha * x_val[i];\n      }\n  \\endcode\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\par Example\n  \\code{.c}\n      // Number of non-zeros of the sparse vector\n      int nnz = 3;\n\n      // Sparse index vector\n      int hx_ind[3] = {0, 3, 5};\n\n      // Sparse value vector\n      double hx_val[3] = {1.0, 2.0, 3.0};\n\n      // Dense vector\n      double hy[9] = {1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0};\n\n      // Scalar alpha\n      double alpha = 3.7;\n\n      // Index base\n      hipsparseIndexBase_t idx_base = HIPSPARSE_INDEX_BASE_ZERO;\n\n      // Offload data to device\n      int* dx_ind;\n      double*        dx_val;\n      double*        dy;\n\n      hipMalloc((void**)&dx_ind, sizeof(int) * nnz);\n      hipMalloc((void**)&dx_val, sizeof(double) * nnz);\n      hipMalloc((void**)&dy, sizeof(double) * 9);\n\n      hipMemcpy(dx_ind, hx_ind, sizeof(int) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dx_val, hx_val, sizeof(double) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dy, hy, sizeof(double) * 9, hipMemcpyHostToDevice);\n\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      // Call daxpyi to perform y = y + alpha * x\n      hipsparseDaxpyi(handle, nnz, &alpha, dx_val, dx_ind, dy, idx_base);\n\n      // Copy result back to host\n      hipMemcpy(hy, dy, sizeof(double) * 9, hipMemcpyDeviceToHost);\n\n      // Clear hipSPARSE\n      hipsparseDestroy(handle);\n\n      // Clear device memory\n      hipFree(dx_ind);\n      hipFree(dx_val);\n      hipFree(dy);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseSaxpyi(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        alpha: *const f32,
        xVal: *const f32,
        xInd: *const ::core::ffi::c_int,
        y: *mut f32,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDaxpyi(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        alpha: *const f64,
        xVal: *const f64,
        xInd: *const ::core::ffi::c_int,
        y: *mut f64,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCaxpyi(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        alpha: *const hipComplex,
        xVal: *const hipComplex,
        xInd: *const ::core::ffi::c_int,
        y: *mut hipComplex,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZaxpyi(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        xVal: *const hipDoubleComplex,
        xInd: *const ::core::ffi::c_int,
        y: *mut hipDoubleComplex,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level1_module\n  \\brief Compute the dot product of a sparse vector with a dense vector.\n\n  \\details\n  \\p hipsparseXdoti computes the dot product of the sparse vector \\f$x\\f$ with the\n  dense vector \\f$y\\f$, such that\n  \\f[\n    \\text{result} := y^T x\n  \\f]\n\n  \\code{.c}\n      for(i = 0; i < nnz; ++i)\n      {\n          result += x_val[i] * y[x_ind[i]];\n      }\n  \\endcode\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\par Example\n  \\code{.c}\n      // Number of non-zeros of the sparse vector\n      int nnz = 3;\n\n      // Sparse index vector\n      int hx_ind[3] = {0, 3, 5};\n\n      // Sparse value vector\n      float hx_val[3] = {1.0f, 2.0f, 3.0f};\n\n      // Dense vector\n      float hy[9] = {1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f, 7.0f, 8.0f, 9.0f};\n\n      // Index base\n      hipsparseIndexBase_t idx_base = HIPSPARSE_INDEX_BASE_ZERO;\n\n      // Offload data to device\n      int* dx_ind;\n      float*        dx_val;\n      float*        dy;\n\n      hipMalloc((void**)&dx_ind, sizeof(int) * nnz);\n      hipMalloc((void**)&dx_val, sizeof(float) * nnz);\n      hipMalloc((void**)&dy, sizeof(float) * 9);\n\n      hipMemcpy(dx_ind, hx_ind, sizeof(int) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dx_val, hx_val, sizeof(float) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dy, hy, sizeof(float) * 9, hipMemcpyHostToDevice);\n\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      // Call sdoti to compute the dot product\n      float dot;\n      hipsparseSdoti(handle, nnz, dx_val, dx_ind, dy, &dot, idx_base);\n\n      // Clear hipSPARSE\n      hipsparseDestroy(handle);\n\n      // Clear device memory\n      hipFree(dx_ind);\n      hipFree(dx_val);\n      hipFree(dy);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseSdoti(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        xVal: *const f32,
        xInd: *const ::core::ffi::c_int,
        y: *const f32,
        result: *mut f32,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDdoti(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        xVal: *const f64,
        xInd: *const ::core::ffi::c_int,
        y: *const f64,
        result: *mut f64,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCdoti(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        xVal: *const hipComplex,
        xInd: *const ::core::ffi::c_int,
        y: *const hipComplex,
        result: *mut hipComplex,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZdoti(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        xVal: *const hipDoubleComplex,
        xInd: *const ::core::ffi::c_int,
        y: *const hipDoubleComplex,
        result: *mut hipDoubleComplex,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level1_module\n  \\brief Compute the dot product of a complex conjugate sparse vector with a dense\n  vector.\n\n  \\details\n  \\p hipsparseXdotci computes the dot product of the complex conjugate sparse vector\n  \\f$x\\f$ with the dense vector \\f$y\\f$, such that\n  \\f[\n    \\text{result} := \\bar{x}^H y\n  \\f]\n\n  \\code{.c}\n      for(i = 0; i < nnz; ++i)\n      {\n          result += conj(x_val[i]) * y[x_ind[i]];\n      }\n  \\endcode\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseCdotci(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        xVal: *const hipComplex,
        xInd: *const ::core::ffi::c_int,
        y: *const hipComplex,
        result: *mut hipComplex,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZdotci(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        xVal: *const hipDoubleComplex,
        xInd: *const ::core::ffi::c_int,
        y: *const hipDoubleComplex,
        result: *mut hipDoubleComplex,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level1_module\n  \\brief Gather elements from a dense vector and store them into a sparse vector.\n\n  \\details\n  \\p hipsparseXgthr gathers the elements that are listed in \\p x_ind from the dense\n  vector \\f$y\\f$ and stores them in the sparse vector \\f$x\\f$.\n\n  \\code{.c}\n      for(i = 0; i < nnz; ++i)\n      {\n          x_val[i] = y[x_ind[i]];\n      }\n  \\endcode\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\par Example\n  \\code{.c}\n      // Number of non-zeros of the sparse vector\n      int nnz = 3;\n\n      // Sparse index vector\n      int hx_ind[3] = {0, 3, 5};\n\n      // Sparse value vector\n      float hx_val[3];\n\n      // Dense vector\n      float hy[9] = {1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0};\n\n      // Index base\n      hipsparseIndexBase_t idx_base = HIPSPARSE_INDEX_BASE_ZERO;\n\n      // Offload data to device\n      int* dx_ind;\n      float*         dx_val;\n      float*         dy;\n\n      hipMalloc((void**)&dx_ind, sizeof(int) * nnz);\n      hipMalloc((void**)&dx_val, sizeof(float) * nnz);\n      hipMalloc((void**)&dy, sizeof(float) * 9);\n\n      hipMemcpy(dx_ind, hx_ind, sizeof(int) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dy, hy, sizeof(float) * 9, hipMemcpyHostToDevice);\n\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      // Call sgthr\n      hipsparseSgthr(handle, nnz, dy, dx_val, dx_ind, idx_base);\n\n      // Copy result back to host\n      hipMemcpy(hx_val, dx_val, sizeof(float) * nnz, hipMemcpyDeviceToHost);\n\n      // Clear hipSPARSE\n      hipsparseDestroy(handle);\n\n      // Clear device memory\n      hipFree(dx_ind);\n      hipFree(dx_val);\n      hipFree(dy);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseSgthr(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        y: *const f32,
        xVal: *mut f32,
        xInd: *const ::core::ffi::c_int,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgthr(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        y: *const f64,
        xVal: *mut f64,
        xInd: *const ::core::ffi::c_int,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgthr(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        y: *const hipComplex,
        xVal: *mut hipComplex,
        xInd: *const ::core::ffi::c_int,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgthr(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        y: *const hipDoubleComplex,
        xVal: *mut hipDoubleComplex,
        xInd: *const ::core::ffi::c_int,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level1_module\n  \\brief Gather and zero out elements from a dense vector and store them into a sparse\n  vector.\n\n  \\details\n  \\p hipsparseXgthrz gathers the elements that are listed in \\p x_ind from the dense\n  vector \\f$y\\f$ and stores them in the sparse vector \\f$x\\f$. The gathered elements\n  in \\f$y\\f$ are replaced by zero.\n\n  \\code{.c}\n      for(i = 0; i < nnz; ++i)\n      {\n          x_val[i]    = y[x_ind[i]];\n          y[x_ind[i]] = 0;\n      }\n  \\endcode\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSgthrz(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        y: *mut f32,
        xVal: *mut f32,
        xInd: *const ::core::ffi::c_int,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgthrz(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        y: *mut f64,
        xVal: *mut f64,
        xInd: *const ::core::ffi::c_int,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgthrz(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        y: *mut hipComplex,
        xVal: *mut hipComplex,
        xInd: *const ::core::ffi::c_int,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgthrz(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        y: *mut hipDoubleComplex,
        xVal: *mut hipDoubleComplex,
        xInd: *const ::core::ffi::c_int,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level1_module\n  \\brief Apply Givens rotation to a dense and a sparse vector.\n\n  \\details\n  \\p hipsparseXroti applies the Givens rotation matrix \\f$G\\f$ to the sparse vector\n  \\f$x\\f$ and the dense vector \\f$y\\f$, where\n  \\f[\n    G = \\begin{pmatrix} c & s \\\\ -s & c \\end{pmatrix}\n  \\f]\n\n  \\code{.c}\n      for(i = 0; i < nnz; ++i)\n      {\n          x_tmp = x_val[i];\n          y_tmp = y[x_ind[i]];\n\n          x_val[i]    = c * x_tmp + s * y_tmp;\n          y[x_ind[i]] = c * y_tmp - s * x_tmp;\n      }\n  \\endcode\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\par Example\n  \\code{.c}\n      // Number of non-zeros of the sparse vector\n      int nnz = 3;\n\n      // Sparse index vector\n      int hx_ind[3] = {0, 3, 5};\n\n      // Sparse value vector\n      float hx_val[3] = {1.0f, 2.0f, 3.0f};\n\n      // Dense vector\n      float hy[9] = {1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f, 7.0f, 8.0f, 9.0f};\n\n      // c and s\n      float c = 3.7;\n      float s = 1.3;\n\n      // Index base\n      hipsparseIndexBase_t idx_base = HIPSPARSE_INDEX_BASE_ZERO;\n\n      // Offload data to device\n      int* dx_ind;\n      float*        dx_val;\n      float*        dy;\n\n      hipMalloc((void**)&dx_ind, sizeof(int) * nnz);\n      hipMalloc((void**)&dx_val, sizeof(float) * nnz);\n      hipMalloc((void**)&dy, sizeof(float) * 9);\n\n      hipMemcpy(dx_ind, hx_ind, sizeof(int) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dx_val, hx_val, sizeof(float) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dy, hy, sizeof(float) * 9, hipMemcpyHostToDevice);\n\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      // Call sroti\n      hipsparseSroti(handle, nnz, dx_val, dx_ind, dy, &c, &s, idx_base);\n\n      // Copy result back to host\n      hipMemcpy(hx_val, dx_val, sizeof(float) * nnz, hipMemcpyDeviceToHost);\n      hipMemcpy(hy, dy, sizeof(float) * 9, hipMemcpyDeviceToHost);\n\n      // Clear hipSPARSE\n      hipsparseDestroy(handle);\n\n      // Clear device memory\n      hipFree(dx_ind);\n      hipFree(dx_val);\n      hipFree(dy);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseSroti(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        xVal: *mut f32,
        xInd: *const ::core::ffi::c_int,
        y: *mut f32,
        c: *const f32,
        s: *const f32,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDroti(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        xVal: *mut f64,
        xInd: *const ::core::ffi::c_int,
        y: *mut f64,
        c: *const f64,
        s: *const f64,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level1_module\n  \\brief Scatter elements from a dense vector across a sparse vector.\n\n  \\details\n  \\p hipsparseXsctr scatters the elements that are listed in \\p x_ind from the sparse\n  vector \\f$x\\f$ into the dense vector \\f$y\\f$. Indices of \\f$y\\f$ that are not listed\n  in \\p x_ind remain unchanged.\n\n  \\code{.c}\n      for(i = 0; i < nnz; ++i)\n      {\n          y[x_ind[i]] = x_val[i];\n      }\n  \\endcode\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\par Example\n  \\code{.c}\n      // Number of non-zeros of the sparse vector\n      int nnz = 3;\n\n      // Sparse index vector\n      int hx_ind[3] = {0, 3, 5};\n\n      // Sparse value vector\n      float hx_val[3] = {9.0, 2.0, 3.0};\n\n      // Dense vector\n      float hy[9] = {1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0};\n\n      // Index base\n      hipsparseIndexBase_t idx_base = HIPSPARSE_INDEX_BASE_ZERO;\n\n      // Offload data to device\n      int* dx_ind;\n      float*         dx_val;\n      float*         dy;\n\n      hipMalloc((void**)&dx_ind, sizeof(int) * nnz);\n      hipMalloc((void**)&dx_val, sizeof(float) * nnz);\n      hipMalloc((void**)&dy, sizeof(float) * 9);\n\n      hipMemcpy(dx_ind, hx_ind, sizeof(int) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dx_val, hx_val, sizeof(float) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dy, hy, sizeof(float) * 9, hipMemcpyHostToDevice);\n\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      // Call ssctr\n      hipsparseSsctr(handle, nnz, dx_val, dx_ind, dy, idx_base);\n\n      // Copy result back to host\n      hipMemcpy(hy, dy, sizeof(float) * 9, hipMemcpyDeviceToHost);\n\n      // Clear hipSPARSE\n      hipsparseDestroy(handle);\n\n      // Clear device memory\n      hipFree(dx_ind);\n      hipFree(dx_val);\n      hipFree(dy);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseSsctr(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        xVal: *const f32,
        xInd: *const ::core::ffi::c_int,
        y: *mut f32,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDsctr(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        xVal: *const f64,
        xInd: *const ::core::ffi::c_int,
        y: *mut f64,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCsctr(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        xVal: *const hipComplex,
        xInd: *const ::core::ffi::c_int,
        y: *mut hipComplex,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZsctr(
        handle: hipsparseHandle_t,
        nnz: ::core::ffi::c_int,
        xVal: *const hipDoubleComplex,
        xInd: *const ::core::ffi::c_int,
        y: *mut hipDoubleComplex,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Sparse matrix vector multiplication using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrmv multiplies the scalar \\f$\\alpha\\f$ with a sparse \\f$m \\times n\\f$\n  matrix, defined in CSR storage format, and the dense vector \\f$x\\f$ and adds the\n  result to the dense vector \\f$y\\f$ that is multiplied by the scalar \\f$\\beta\\f$,\n  such that\n  \\f[\n    y := \\alpha \\cdot op(A) \\cdot x + \\beta \\cdot y,\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        A^T, & \\text{if trans == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        A^H, & \\text{if trans == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n\n  \\code{.c}\n      for(i = 0; i < m; ++i)\n      {\n          y[i] = beta * y[i];\n\n          for(j = csr_row_ptr[i]; j < csr_row_ptr[i + 1]; ++j)\n          {\n              y[i] = y[i] + alpha * csr_val[j] * x[csr_col_ind[j]];\n          }\n      }\n  \\endcode\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\note\n  Currently, only \\p trans == \\ref HIPSPARSE_OPERATION_NON_TRANSPOSE is supported.\n\n  \\par Example\n  \\code{.c}\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      // alpha * ( 1.0  0.0  2.0 ) * ( 1.0 ) + beta * ( 4.0 ) = (  31.1 )\n      //         ( 3.0  0.0  4.0 ) * ( 2.0 )          ( 5.0 ) = (  62.0 )\n      //         ( 5.0  6.0  0.0 ) * ( 3.0 )          ( 6.0 ) = (  70.7 )\n      //         ( 7.0  0.0  8.0 ) *                  ( 7.0 ) = ( 123.8 )\n\n      int m = 4;\n      int n = 3;\n      int nnz = 8;\n\n      // CSR row pointers\n      int hcsr_row_ptr[5] = {0, 2, 4, 6, 8};\n\n      // CSR column indices\n      int hcsr_col_ind[8] = {0, 2, 0, 2, 0, 1, 0, 2};\n\n      // CSR values\n      double hcsr_val[8] = {1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0};\n\n      // Transposition of the matrix\n      hipsparseOperation_t trans = HIPSPARSE_OPERATION_NON_TRANSPOSE;\n\n      // Scalar alpha and beta\n      double alpha = 3.7;\n      double beta  = 1.3;\n\n      // x and y\n      double hx[3] = {1.0, 2.0, 3.0};\n      double hy[4] = {4.0, 5.0, 6.0, 7.0};\n\n      // Matrix descriptor\n      hipsparseMatDescr_t descr;\n      hipsparseCreateMatDescr(&descr);\n\n      // Offload data to device\n      int* dcsr_row_ptr;\n      int* dcsr_col_ind;\n      double*        dcsr_val;\n      double*        dx;\n      double*        dy;\n\n      hipMalloc((void**)&dcsr_row_ptr, sizeof(int) * (m + 1));\n      hipMalloc((void**)&dcsr_col_ind, sizeof(int) * nnz);\n      hipMalloc((void**)&dcsr_val, sizeof(double) * nnz);\n      hipMalloc((void**)&dx, sizeof(double) * n);\n      hipMalloc((void**)&dy, sizeof(double) * m);\n\n      hipMemcpy(dcsr_row_ptr, hcsr_row_ptr, sizeof(int) * (m + 1), hipMemcpyHostToDevice);\n      hipMemcpy(dcsr_col_ind, hcsr_col_ind, sizeof(int) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dcsr_val, hcsr_val, sizeof(double) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dx, hx, sizeof(double) * n, hipMemcpyHostToDevice);\n      hipMemcpy(dy, hy, sizeof(double) * m, hipMemcpyHostToDevice);\n\n      // Call dcsrmv to perform y = alpha * A x + beta * y\n      hipsparseDcsrmv(handle,\n                      trans,\n                      m,\n                      n,\n                      nnz,\n                      &alpha,\n                      descr,\n                      dcsr_val,\n                      dcsr_row_ptr,\n                      dcsr_col_ind,\n                      dx,\n                      &beta,\n                      dy);\n\n      // Copy result back to host\n      hipMemcpy(hy, dy, sizeof(double) * m, hipMemcpyDeviceToHost);\n\n      // Clear hipSPARSE\n      hipsparseDestroyMatDescr(descr);\n      hipsparseDestroy(handle);\n\n      // Clear device memory\n      hipFree(dcsr_row_ptr);\n      hipFree(dcsr_col_ind);\n      hipFree(dcsr_val);\n      hipFree(dx);\n      hipFree(dy);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseScsrmv(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrmv(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrmv(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        x: *const hipComplex,
        beta: *const hipComplex,
        y: *mut hipComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrmv(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        x: *const hipDoubleComplex,
        beta: *const hipDoubleComplex,
        y: *mut hipDoubleComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup level2_module
  \brief Sparse triangular solve using CSR storage format

  \details
  \p hipsparseXcsrsv2_zeroPivot returns \ref HIPSPARSE_STATUS_ZERO_PIVOT, if either a
  structural or numerical zero has been found during hipsparseScsrsv2_solve(),
  hipsparseDcsrsv2_solve(), hipsparseCcsrsv2_solve() or hipsparseZcsrsv2_solve()
  computation. The first zero pivot \f$j\f$ at \f$A_{j,j}\f$ is stored in \p position,
  using same index base as the CSR matrix.

  \p position can be in host or device memory. If no zero pivot has been found,
  \p position is set to -1 and \ref HIPSPARSE_STATUS_SUCCESS is returned instead.

  \note \p hipsparseXcsrsv2_zeroPivot is a blocking function. It might influence
  performance negatively.*/
    pub fn hipsparseXcsrsv2_zeroPivot(
        handle: hipsparseHandle_t,
        info: csrsv2Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Sparse triangular solve using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrsv2_bufferSize returns the size of the temporary storage buffer in bytes\n  that is required by hipsparseScsrsv2_analysis(), hipsparseDcsrsv2_analysis(),\n  hipsparseCcsrsv2_analysis(), hipsparseZcsrsv2_analysis(), hipsparseScsrsv2_solve(),\n  hipsparseDcsrsv2_solve(), hipsparseCcsrsv2_solve() and hipsparseZcsrsv2_solve(). The\n  temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseScsrsv2_bufferSize(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrsv2_bufferSize(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrsv2_bufferSize(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrsv2_bufferSize(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Sparse triangular solve using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrsv2_bufferSizeExt returns the size of the temporary storage buffer in bytes\n  that is required by hipsparseScsrsv2_analysis(), hipsparseDcsrsv2_analysis(),\n  hipsparseCcsrsv2_analysis(), hipsparseZcsrsv2_analysis(), hipsparseScsrsv2_solve(),\n  hipsparseDcsrsv2_solve(), hipsparseCcsrsv2_solve() and hipsparseZcsrsv2_solve(). The\n  temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseScsrsv2_bufferSizeExt(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrsv2_bufferSizeExt(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrsv2_bufferSizeExt(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrsv2_bufferSizeExt(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Sparse triangular solve using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrsv2_analysis performs the analysis step for hipsparseScsrsv2_solve(),\n  hipsparseDcsrsv2_solve(), hipsparseCcsrsv2_solve() and hipsparseZcsrsv2_solve().\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseScsrsv2_analysis(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrsv2_analysis(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrsv2_analysis(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrsv2_analysis(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Sparse triangular solve using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrsv2_solve solves a sparse triangular linear system of a sparse\n  \\f$m \\times m\\f$ matrix, defined in CSR storage format, a dense solution vector\n  \\f$y\\f$ and the right-hand side \\f$x\\f$ that is multiplied by \\f$\\alpha\\f$, such that\n  \\f[\n    op(A) \\cdot y = \\alpha \\cdot x,\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        A^T, & \\text{if trans == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        A^H, & \\text{if trans == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n\n  \\p hipsparseXcsrsv2_solve requires a user allocated temporary buffer. Its size is\n  returned by hipsparseXcsrsv2_bufferSize() or hipsparseXcsrsv2_bufferSizeExt().\n  Furthermore, analysis meta data is required. It can be obtained by\n  hipsparseXcsrsv2_analysis(). \\p hipsparseXcsrsv2_solve reports the first zero pivot\n  (either numerical or structural zero). The zero pivot status can be checked calling\n  hipsparseXcsrsv2_zeroPivot(). If\n  \\ref hipsparseDiagType_t == \\ref HIPSPARSE_DIAG_TYPE_UNIT, no zero pivot will be\n  reported, even if \\f$A_{j,j} = 0\\f$ for some \\f$j\\f$.\n\n  \\note\n  The sparse CSR matrix has to be sorted. This can be achieved by calling\n  hipsparseXcsrsort().\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\note\n  Currently, only \\p trans == \\ref HIPSPARSE_OPERATION_NON_TRANSPOSE and\n  \\p trans == \\ref HIPSPARSE_OPERATION_TRANSPOSE is supported.\n\n  \\par Example\n  \\code{.c}\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      // alpha * ( 1.0  0.0  2.0  0.0 ) * ( x_0 ) = ( 32.0 )\n      //         ( 3.0  2.0  4.0  1.0 ) * ( x_1 ) = ( 14.7 )\n      //         ( 5.0  6.0  1.0  3.0 ) * ( x_2 ) = ( 33.6 )\n      //         ( 7.0  0.0  8.0  0.6 ) * ( x_3 ) = ( 10.0 )\n\n      int m = 4;\n      int nnz = 13;\n\n      // CSR row pointers\n      int hcsr_row_ptr[5] = {0, 2, 6, 10, 13};\n\n      // CSR column indices\n      int hcsr_col_ind[13] = {0, 2, 0, 1, 2, 3, 0, 1, 2, 3, 0, 2, 3};\n\n      // CSR values\n      double hcsr_val[13] = {1.0, 2.0, 3.0, 2.0, 4.0, 1.0, 5.0, 6.0, 1.0, 3.0, 7.0, 8.0, 0.6};\n\n      // Transposition of the matrix\n      hipsparseOperation_t trans = HIPSPARSE_OPERATION_NON_TRANSPOSE;\n      hipsparseSolvePolicy_t policy = HIPSPARSE_SOLVE_POLICY_USE_LEVEL;\n\n      // Scalar alpha\n      double alpha = 1.0;\n\n      // f and x\n      double hf[4] = {32.0, 14.7, 33.6, 10.0};\n      double hx[4];\n\n      // Matrix descriptor\n      hipsparseMatDescr_t descr;\n      hipsparseCreateMatDescr(&descr);\n\n      // Set index base on descriptor\n      hipsparseSetMatIndexBase(descr, HIPSPARSE_INDEX_BASE_ZERO);\n\n      // Set fill mode on descriptor\n      hipsparseSetMatFillMode(descr, HIPSPARSE_FILL_MODE_LOWER);\n\n      // Set diag type on descriptor\n      hipsparseSetMatDiagType(descr, HIPSPARSE_DIAG_TYPE_UNIT);\n\n      // Csrsv info\n      csrsv2Info_t info;\n      hipsparseCreateCsrsv2Info(&info);\n\n      // Offload data to device\n      int* dcsr_row_ptr;\n      int* dcsr_col_ind;\n      double*        dcsr_val;\n      double*        df;\n      double*        dx;\n\n      hipMalloc((void**)&dcsr_row_ptr, sizeof(int) * (m + 1));\n      hipMalloc((void**)&dcsr_col_ind, sizeof(int) * nnz);\n      hipMalloc((void**)&dcsr_val, sizeof(double) * nnz);\n      hipMalloc((void**)&df, sizeof(double) * m);\n      hipMalloc((void**)&dx, sizeof(double) * m);\n\n      hipMemcpy(dcsr_row_ptr, hcsr_row_ptr, sizeof(int) * (m + 1), hipMemcpyHostToDevice);\n      hipMemcpy(dcsr_col_ind, hcsr_col_ind, sizeof(int) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dcsr_val, hcsr_val, sizeof(double) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(df, hf, sizeof(double) * m, hipMemcpyHostToDevice);\n\n      int bufferSize = 0;\n      hipsparseDcsrsv2_bufferSize(handle,\n                                  trans,\n                                  m,\n                                  nnz,\n                                  descr,\n                                  dcsr_val,\n                                  dcsr_row_ptr,\n                                  dcsr_col_ind,\n                                  info,\n                                  &bufferSize);\n\n      void* dbuffer = nullptr;\n      hipMalloc((void**)&dbuffer, bufferSize);\n\n      hipsparseDcsrsv2_analysis(handle,\n                                trans,\n                                m,\n                                nnz,\n                                descr,\n                                dcsr_val,\n                                dcsr_row_ptr,\n                                dcsr_col_ind,\n                                info,\n                                policy,\n                                dbuffer);\n\n      // Call dcsrsv to perform alpha * A * x = f\n      hipsparseDcsrsv2_solve(handle,\n                             trans,\n                             m,\n                             nnz,\n                             &alpha,\n                             descr,\n                             dcsr_val,\n                             dcsr_row_ptr,\n                             dcsr_col_ind,\n                             info,\n                             df,\n                             dx,\n                             policy,\n                             dbuffer);\n\n      // Copy result back to host\n      hipMemcpy(hx, dx, sizeof(double) * m, hipMemcpyDeviceToHost);\n\n      // Clear hipSPARSE\n      hipsparseDestroyMatDescr(descr);\n      hipsparseDestroyCsrsv2Info(info);\n      hipsparseDestroy(handle);\n\n      // Clear device memory\n      hipFree(dcsr_row_ptr);\n      hipFree(dcsr_col_ind);\n      hipFree(dcsr_val);\n      hipFree(df);\n      hipFree(dx);\n      hipFree(dbuffer);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseScsrsv2_solve(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        f: *const f32,
        x: *mut f32,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrsv2_solve(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        f: *const f64,
        x: *mut f64,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrsv2_solve(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        f: *const hipComplex,
        x: *mut hipComplex,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrsv2_solve(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrsv2Info_t,
        f: *const hipDoubleComplex,
        x: *mut hipDoubleComplex,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Sparse matrix vector multiplication using HYB storage format\n\n  \\details\n  \\p hipsparseXhybmv multiplies the scalar \\f$\\alpha\\f$ with a sparse \\f$m \\times n\\f$\n  matrix, defined in HYB storage format, and the dense vector \\f$x\\f$ and adds the\n  result to the dense vector \\f$y\\f$ that is multiplied by the scalar \\f$\\beta\\f$,\n  such that\n  \\f[\n    y := \\alpha \\cdot op(A) \\cdot x + \\beta \\cdot y,\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        A^T, & \\text{if trans == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        A^H, & \\text{if trans == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\note\n  Currently, only \\p trans == \\ref HIPSPARSE_OPERATION_NON_TRANSPOSE is supported.\n\n  \\par Example\n  \\code{.c}\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      // A sparse matrix\n      // 1 0 3 4\n      // 0 0 5 1\n      // 0 2 0 0\n      // 4 0 0 8\n      int hAptr[5] = {0, 3, 5, 6, 8};\n      int hAcol[8] = {0, 2, 3, 2, 3, 1, 0, 3};\n      double hAval[8] = {1.0, 3.0, 4.0, 5.0, 1.0, 2.0, 4.0, 8.0};\n\n      int m = 4;\n      int n = 4;\n      int nnz = 8;\n\n      double halpha = 1.0;\n      double hbeta  = 0.0;\n\n      double  hx[4] = {1.0, 2.0, 3.0, 4.0};\n      double  hy[4] = {4.0, 5.0, 6.0, 7.0};\n\n      // Matrix descriptor\n      hipsparseMatDescr_t descrA;\n      hipsparseCreateMatDescr(&descrA);\n\n      // Offload data to device\n      int* dAptr = NULL;\n      int* dAcol = NULL;\n      double*        dAval = NULL;\n      double*        dx    = NULL;\n      double*        dy    = NULL;\n\n      hipMalloc((void**)&dAptr, sizeof(int) * (m + 1));\n      hipMalloc((void**)&dAcol, sizeof(int) * nnz);\n      hipMalloc((void**)&dAval, sizeof(double) * nnz);\n      hipMalloc((void**)&dx, sizeof(double) * n);\n      hipMalloc((void**)&dy, sizeof(double) * m);\n\n      hipMemcpy(dAptr, hAptr, sizeof(int) * (m + 1), hipMemcpyHostToDevice);\n      hipMemcpy(dAcol, hAcol, sizeof(int) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dAval, hAval, sizeof(double) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dx, hx, sizeof(double) * n, hipMemcpyHostToDevice);\n\n      // Convert CSR matrix to HYB format\n      hipsparseHybMat_t hybA;\n      hipsparseCreateHybMat(&hybA);\n\n      hipsparseDcsr2hyb(handle, m, n, descrA, dAval, dAptr, dAcol, hybA, 0, HIPSPARSE_HYB_PARTITION_AUTO);\n\n      // Clean up CSR structures\n      hipFree(dAptr);\n      hipFree(dAcol);\n      hipFree(dAval);\n\n      // Call hipsparse hybmv\n      hipsparseDhybmv(handle, HIPSPARSE_OPERATION_NON_TRANSPOSE, &halpha, descrA, hybA, dx, &hbeta, dy);\n\n      // Copy result back to host\n      hipMemcpy(hy, dy, sizeof(double) * m, hipMemcpyDeviceToHost);\n\n      // Clear up on device\n      hipsparseDestroyHybMat(hybA);\n      hipsparseDestroyMatDescr(descrA);\n      hipsparseDestroy(handle);\n\n      hipFree(dx);\n      hipFree(dy);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseShybmv(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        hybA: hipsparseHybMat_t,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDhybmv(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        hybA: hipsparseHybMat_t,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseChybmv(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        hybA: hipsparseHybMat_t,
        x: *const hipComplex,
        beta: *const hipComplex,
        y: *mut hipComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZhybmv(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        hybA: hipsparseHybMat_t,
        x: *const hipDoubleComplex,
        beta: *const hipDoubleComplex,
        y: *mut hipDoubleComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Sparse matrix vector multiplication using BSR storage format\n\n  \\details\n  \\p hipsparseXbsrmv multiplies the scalar \\f$\\alpha\\f$ with a sparse\n  \\f$(mb \\cdot \\text{block_dim}) \\times (nb \\cdot \\text{block_dim})\\f$\n  matrix, defined in BSR storage format, and the dense vector \\f$x\\f$ and adds the\n  result to the dense vector \\f$y\\f$ that is multiplied by the scalar \\f$\\beta\\f$,\n  such that\n  \\f[\n    y := \\alpha \\cdot op(A) \\cdot x + \\beta \\cdot y,\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        A^T, & \\text{if trans == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        A^H, & \\text{if trans == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\note\n  Currently, only \\p trans == \\ref HIPSPARSE_OPERATION_NON_TRANSPOSE is supported.\n\n  \\par Example\n  \\code{.c}\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      // alpha * ( 1.0  0.0  2.0 ) * ( 1.0 ) + beta * ( 4.0 ) = (  31.1 )\n      //         ( 3.0  0.0  4.0 ) * ( 2.0 )          ( 5.0 ) = (  62.0 )\n      //         ( 5.0  6.0  0.0 ) * ( 3.0 )          ( 6.0 ) = (  70.7 )\n      //         ( 7.0  0.0  8.0 ) *                  ( 7.0 ) = ( 123.8 )\n\n      // BSR block dimension\n      int bsr_dim = 2;\n\n      // Number of block rows and columns\n      int mb = 2;\n      int nb = 2;\n\n      // Number of non-zero blocks\n      int nnzb = 4;\n\n      // BSR row pointers\n      int hbsr_row_ptr[3] = {0, 2, 4};\n\n      // BSR column indices\n      int hbsr_col_ind[4] = {0, 1, 0, 1};\n\n      // BSR values\n      double hbsr_val[16]\n        = {1.0, 3.0, 0.0, 0.0, 2.0, 4.0, 0.0, 0.0, 5.0, 7.0, 6.0, 0.0, 0.0, 8.0, 0.0, 0.0};\n\n      // Block storage in column major\n      hipsparseDirection_t dir = HIPSPARSE_DIRECTION_COLUMN;\n\n      // Transposition of the matrix\n      hipsparseOperation_t trans = HIPSPARSE_OPERATION_NON_TRANSPOSE;\n\n      // Scalar alpha and beta\n      double alpha = 3.7;\n      double beta  = 1.3;\n\n      // x and y\n      double hx[4] = {1.0, 2.0, 3.0, 0.0};\n      double hy[4] = {4.0, 5.0, 6.0, 7.0};\n\n      // Matrix descriptor\n      hipsparseMatDescr_t descr;\n      hipsparseCreateMatDescr(&descr);\n\n      // Offload data to device\n      int* dbsr_row_ptr;\n      int* dbsr_col_ind;\n      double*        dbsr_val;\n      double*        dx;\n      double*        dy;\n\n      hipMalloc((void**)&dbsr_row_ptr, sizeof(int) * (mb + 1));\n      hipMalloc((void**)&dbsr_col_ind, sizeof(int) * nnzb);\n      hipMalloc((void**)&dbsr_val, sizeof(double) * nnzb * bsr_dim * bsr_dim);\n      hipMalloc((void**)&dx, sizeof(double) * nb * bsr_dim);\n      hipMalloc((void**)&dy, sizeof(double) * mb * bsr_dim);\n\n      hipMemcpy(dbsr_row_ptr, hbsr_row_ptr, sizeof(int) * (mb + 1), hipMemcpyHostToDevice);\n      hipMemcpy(dbsr_col_ind, hbsr_col_ind, sizeof(int) * nnzb, hipMemcpyHostToDevice);\n      hipMemcpy(dbsr_val, hbsr_val, sizeof(double) * nnzb * bsr_dim * bsr_dim, hipMemcpyHostToDevice);\n      hipMemcpy(dx, hx, sizeof(double) * nb * bsr_dim, hipMemcpyHostToDevice);\n      hipMemcpy(dy, hy, sizeof(double) * mb * bsr_dim, hipMemcpyHostToDevice);\n\n      // Call dbsrmv to perform y = alpha * A x + beta * y\n      hipsparseDbsrmv(handle,\n                      dir,\n                      trans,\n                      mb,\n                      nb,\n                      nnzb,\n                      &alpha,\n                      descr,\n                      dbsr_val,\n                      dbsr_row_ptr,\n                      dbsr_col_ind,\n                      bsr_dim,\n                      dx,\n                      &beta,\n                      dy);\n\n      // Copy result back to host\n      hipMemcpy(hy, dy, sizeof(double) * mb * bsr_dim, hipMemcpyDeviceToHost);\n\n      // Clear hipSPARSE\n      hipsparseDestroyMatDescr(descr);\n      hipsparseDestroy(handle);\n\n      // Clear device memory\n      hipFree(dbsr_row_ptr);\n      hipFree(dbsr_col_ind);\n      hipFree(dbsr_val);\n      hipFree(dx);\n      hipFree(dy);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseSbsrmv(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrmv(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrmv(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const hipComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const hipComplex,
        beta: *const hipComplex,
        y: *mut hipComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrmv(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const hipDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const hipDoubleComplex,
        beta: *const hipDoubleComplex,
        y: *mut hipDoubleComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Sparse matrix vector multiplication with mask operation using BSR storage format\n\n  \\details\n  \\p hipsparseXbsrxmv multiplies the scalar \\f$\\alpha\\f$ with a sparse\n  \\f$(mb \\cdot \\text{block_dim}) \\times (nb \\cdot \\text{block_dim})\\f$\n  modified matrix, defined in BSR storage format, and the dense vector \\f$x\\f$ and adds the\n  result to the dense vector \\f$y\\f$ that is multiplied by the scalar \\f$\\beta\\f$,\n  such that\n  \\f[\n    y := \\left( \\alpha \\cdot op(A) \\cdot x + \\beta \\cdot y \\right)\\left( \\text{mask} \\right),\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        A^T, & \\text{if trans == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        A^H, & \\text{if trans == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n\n  The \\f$\\text{mask}\\f$ is defined as an array of block row indices.\n  The input sparse matrix is defined with a modified BSR storage format where the beginning and the end of each row\n  is defined with two arrays, \\p bsr_row_ptr and \\p bsr_end_ptr (both of size \\p mb), rather the usual \\p bsr_row_ptr of size \\p mb + 1.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\note\n  Currently, only \\p trans == \\ref HIPSPARSE_OPERATION_NON_TRANSPOSE is supported.\n  Currently, \\p block_dim == 1 is not supported.\n/\n/**@{"]
    pub fn hipsparseSbsrxmv(
        handle: hipsparseHandle_t,
        dir: hipsparseDirection_t,
        trans: hipsparseOperation_t,
        sizeOfMask: ::core::ffi::c_int,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f32,
        descr: hipsparseMatDescr_t,
        bsrVal: *const f32,
        bsrMaskPtr: *const ::core::ffi::c_int,
        bsrRowPtr: *const ::core::ffi::c_int,
        bsrEndPtr: *const ::core::ffi::c_int,
        bsrColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrxmv(
        handle: hipsparseHandle_t,
        dir: hipsparseDirection_t,
        trans: hipsparseOperation_t,
        sizeOfMask: ::core::ffi::c_int,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f64,
        descr: hipsparseMatDescr_t,
        bsrVal: *const f64,
        bsrMaskPtr: *const ::core::ffi::c_int,
        bsrRowPtr: *const ::core::ffi::c_int,
        bsrEndPtr: *const ::core::ffi::c_int,
        bsrColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrxmv(
        handle: hipsparseHandle_t,
        dir: hipsparseDirection_t,
        trans: hipsparseOperation_t,
        sizeOfMask: ::core::ffi::c_int,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descr: hipsparseMatDescr_t,
        bsrVal: *const hipComplex,
        bsrMaskPtr: *const ::core::ffi::c_int,
        bsrRowPtr: *const ::core::ffi::c_int,
        bsrEndPtr: *const ::core::ffi::c_int,
        bsrColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const hipComplex,
        beta: *const hipComplex,
        y: *mut hipComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrxmv(
        handle: hipsparseHandle_t,
        dir: hipsparseDirection_t,
        trans: hipsparseOperation_t,
        sizeOfMask: ::core::ffi::c_int,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descr: hipsparseMatDescr_t,
        bsrVal: *const hipDoubleComplex,
        bsrMaskPtr: *const ::core::ffi::c_int,
        bsrRowPtr: *const ::core::ffi::c_int,
        bsrEndPtr: *const ::core::ffi::c_int,
        bsrColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const hipDoubleComplex,
        beta: *const hipDoubleComplex,
        y: *mut hipDoubleComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup level2_module
  \brief Sparse triangular solve using BSR storage format

  \details
  \p hipsparseXbsrsv2_zeroPivot returns \ref HIPSPARSE_STATUS_ZERO_PIVOT, if either a
  structural or numerical zero has been found during hipsparseXbsrsv2_analysis() or
  hipsparseXbsrsv2_solve() computation. The first zero pivot \f$j\f$ at \f$A_{j,j}\f$
  is stored in \p position, using same index base as the BSR matrix.

  \p position can be in host or device memory. If no zero pivot has been found,
  \p position is set to -1 and \ref HIPSPARSE_STATUS_SUCCESS is returned instead.

  \note \p hipsparseXbsrsv2_zeroPivot is a blocking function. It might influence
  performance negatively.*/
    pub fn hipsparseXbsrsv2_zeroPivot(
        handle: hipsparseHandle_t,
        info: bsrsv2Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Sparse triangular solve using BSR storage format\n\n  \\details\n  \\p hipsparseXbsrsv2_bufferSize returns the size of the temporary storage buffer in bytes\n  that is required by hipsparseXbsrsv2_analysis() and hipsparseXbsrsv2_solve(). The\n  temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSbsrsv2_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrsv2_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrsv2_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut hipComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrsv2_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut hipDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Sparse triangular solve using BSR storage format\n\n  \\details\n  \\p hipsparseXbsrsv2_bufferSizeExt returns the size of the temporary storage buffer in bytes\n  that is required by hipsparseXbsrsv2_analysis() and hipsparseXbsrsv2_solve(). The\n  temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSbsrsv2_bufferSizeExt(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrsv2_bufferSizeExt(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrsv2_bufferSizeExt(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut hipComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrsv2_bufferSizeExt(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut hipDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Sparse triangular solve using BSR storage format\n\n  \\details\n  \\p hipsparseXbsrsv2_analysis performs the analysis step for hipsparseXbsrsv2_solve().\n\n  \\note\n  If the matrix sparsity pattern changes, the gathered information will become invalid.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSbsrsv2_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrsv2_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrsv2_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const hipComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrsv2_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const hipDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Sparse triangular solve using BSR storage format\n\n  \\details\n  \\p hipsparseXbsrsv2_solve solves a sparse triangular linear system of a sparse\n  \\f$m \\times m\\f$ matrix, defined in BSR storage format, a dense solution vector\n  \\f$y\\f$ and the right-hand side \\f$x\\f$ that is multiplied by \\f$\\alpha\\f$, such that\n  \\f[\n    op(A) \\cdot y = \\alpha \\cdot x,\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        A^T, & \\text{if trans == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        A^H, & \\text{if trans == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n\n  \\p hipsparseXbsrsv2_solve requires a user allocated temporary buffer. Its size is\n  returned by hipsparseXbsrsv2_bufferSize() or hipsparseXbsrsv2_bufferSizeExt().\n  Furthermore, analysis meta data is required. It can be obtained by\n  hipsparseXbsrsv2_analysis(). \\p hipsparseXbsrsv2_solve reports the first zero pivot\n  (either numerical or structural zero). The zero pivot status can be checked calling\n  hipsparseXbsrsv2_zeroPivot(). If\n  \\ref hipsparseDiagType_t == \\ref HIPSPARSE_DIAG_TYPE_UNIT, no zero pivot will be\n  reported, even if \\f$A_{j,j} = 0\\f$ for some \\f$j\\f$.\n\n  \\note\n  The sparse BSR matrix has to be sorted.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\note\n  Currently, only \\p trans == \\ref HIPSPARSE_OPERATION_NON_TRANSPOSE and\n  \\p trans == \\ref HIPSPARSE_OPERATION_TRANSPOSE is supported.\n\n  \\par Example\n  \\code{.c}\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      // A = ( 1.0  0.0  0.0  0.0 )\n      //     ( 2.0  3.0  0.0  0.0 )\n      //     ( 4.0  5.0  6.0  0.0 )\n      //     ( 7.0  0.0  8.0  9.0 )\n      //\n      // with bsr_dim = 2\n      //\n      //      -------------------\n      //   = | 1.0 0.0 | 0.0 0.0 |\n      //     | 2.0 3.0 | 0.0 0.0 |\n      //      -------------------\n      //     | 4.0 5.0 | 6.0 0.0 |\n      //     | 7.0 0.0 | 8.0 9.0 |\n      //      -------------------\n\n      // Number of rows and columns\n      int m = 4;\n\n      // Number of block rows and block columns\n      int mb = 2;\n      int nb = 2;\n\n      // BSR block dimension\n      int bsr_dim = 2;\n\n      // Number of non-zero blocks\n      int nnzb = 3;\n\n      // BSR row pointers\n      int hbsr_row_ptr[3] = {0, 1, 3};\n\n      // BSR column indices\n      int hbsr_col_ind[3] = {0, 0, 1};\n\n      // BSR values\n      double hbsr_val[12] = {1.0, 2.0, 0.0, 3.0, 4.0, 7.0, 5.0, 0.0, 6.0, 8.0, 0.0, 9.0};\n\n      // Storage scheme of the BSR blocks\n      hipsparseDirection_t dir = HIPSPARSE_DIRECTION_COLUMN;\n\n      // Transposition of the matrix and rhs matrix\n      hipsparseOperation_t trans = HIPSPARSE_OPERATION_NON_TRANSPOSE;\n\n      // Solve policy\n      hipsparseSolvePolicy_t solve_policy = HIPSPARSE_SOLVE_POLICY_USE_LEVEL;\n\n      // Scalar alpha and beta\n      double alpha = 3.7;\n\n      double hx[4] = {1, 2, 3, 4};\n      double hy[4];\n\n      // Offload data to device\n      int* dbsr_row_ptr;\n      int* dbsr_col_ind;\n      double* dbsr_val;\n      double* dx;\n      double* dy;\n\n      hipMalloc((void**)&dbsr_row_ptr, sizeof(int) * (mb + 1));\n      hipMalloc((void**)&dbsr_col_ind, sizeof(int) * nnzb);\n      hipMalloc((void**)&dbsr_val, sizeof(double) * nnzb * bsr_dim * bsr_dim);\n      hipMalloc((void**)&dx, sizeof(double) * nb * bsr_dim);\n      hipMalloc((void**)&dy, sizeof(double) * mb * bsr_dim);\n\n      hipMemcpy(dbsr_row_ptr, hbsr_row_ptr, sizeof(int) * (mb + 1), hipMemcpyHostToDevice);\n      hipMemcpy(dbsr_col_ind, hbsr_col_ind, sizeof(int) * nnzb, hipMemcpyHostToDevice);\n      hipMemcpy(dbsr_val, hbsr_val, sizeof(double) * nnzb * bsr_dim * bsr_dim, hipMemcpyHostToDevice);\n      hipMemcpy(dx, hx, sizeof(double) * nb * bsr_dim, hipMemcpyHostToDevice);\n\n      // Matrix descriptor\n      hipsparseMatDescr_t descr;\n      hipsparseCreateMatDescr(&descr);\n\n      // Matrix fill mode\n      hipsparseSetMatFillMode(descr, HIPSPARSE_FILL_MODE_LOWER);\n\n      // Matrix diagonal type\n      hipsparseSetMatDiagType(descr, HIPSPARSE_DIAG_TYPE_UNIT);\n\n      // Matrix info structure\n      bsrsv2Info_t info;\n      hipsparseCreateBsrsv2Info(&info);\n\n      // Obtain required buffer size\n      int buffer_size;\n      hipsparseDbsrsv2_bufferSize(handle,\n                                  dir,\n                                  trans,\n                                  mb,\n                                  nnzb,\n                                  descr,\n                                  dbsr_val,\n                                  dbsr_row_ptr,\n                                  dbsr_col_ind,\n                                  bsr_dim,\n                                  info,\n                                  &buffer_size);\n\n      // Allocate temporary buffer\n      void* dbuffer;\n      hipMalloc(&dbuffer, buffer_size);\n\n      // Perform analysis step\n      hipsparseDbsrsv2_analysis(handle,\n                                dir,\n                                trans,\n                                mb,\n                                nnzb,\n                                descr,\n                                dbsr_val,\n                                dbsr_row_ptr,\n                                dbsr_col_ind,\n                                bsr_dim,\n                                info,\n                                solve_policy,\n                                dbuffer);\n\n      // Call dbsrsm to perform lower triangular solve LX = B\n      hipsparseDbsrsv2_solve(handle,\n                             dir,\n                             trans,\n                             mb,\n                             nnzb,\n                             &alpha,\n                             descr,\n                             dbsr_val,\n                             dbsr_row_ptr,\n                             dbsr_col_ind,\n                             bsr_dim,\n                             info,\n                             dx,\n                             dy,\n                             solve_policy,\n                             dbuffer);\n\n      // Check for zero pivots\n      int    pivot;\n      hipsparseStatus_t status = hipsparseXbsrsv2_zeroPivot(handle, info, &pivot);\n\n      if(status == HIPSPARSE_STATUS_ZERO_PIVOT)\n      {\n          std::cout << \"Found zero pivot in matrix row \" << pivot << std::endl;\n      }\n\n      // Copy results back to the host\n      hipMemcpy(hy, dy, sizeof(double) * mb * bsr_dim, hipMemcpyDeviceToHost);\n\n      // Clear hipSPARSE\n      hipsparseDestroyBsrsv2Info(info);\n      hipsparseDestroyMatDescr(descr);\n      hipsparseDestroy(handle);\n\n      // Clear device memory\n      hipFree(dbsr_row_ptr);\n      hipFree(dbsr_col_ind);\n      hipFree(dbsr_val);\n      hipFree(dx);\n      hipFree(dy);\n      hipFree(dbuffer);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseSbsrsv2_solve(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        f: *const f32,
        x: *mut f32,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrsv2_solve(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        f: *const f64,
        x: *mut f64,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrsv2_solve(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const hipComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        f: *const hipComplex,
        x: *mut hipComplex,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrsv2_solve(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const hipDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsv2Info_t,
        f: *const hipDoubleComplex,
        x: *mut hipDoubleComplex,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Dense matrix sparse vector multiplication\n\n  \\details\n  \\p hipsparseXgemvi_bufferSize returns the size of the temporary storage buffer in bytes\n  required by hipsparseXgemvi(). The temporary storage buffer must be allocated by the\n  user.\n/\n/**@{"]
    pub fn hipsparseSgemvi_bufferSize(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgemvi_bufferSize(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgemvi_bufferSize(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgemvi_bufferSize(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level2_module\n  \\brief Dense matrix sparse vector multiplication\n\n  \\details\n  \\p hipsparseXgemvi multiplies the scalar \\f$\\alpha\\f$ with a dense \\f$m \\times n\\f$\n  matrix \\f$A\\f$ and the sparse vector \\f$x\\f$ and adds the result to the dense vector\n  \\f$y\\f$ that is multiplied by the scalar \\f$\\beta\\f$, such that\n  \\f[\n    y := \\alpha \\cdot op(A) \\cdot x + \\beta \\cdot y,\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        A^T, & \\text{if trans == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        A^H, & \\text{if trans == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n\n  \\p hipsparseXgemvi requires a user allocated temporary buffer. Its size is returned\n  by hipsparseXgemvi_bufferSize().\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\note\n  Currently, only \\p trans == \\ref HIPSPARSE_OPERATION_NON_TRANSPOSE is supported.\n/\n/**@{"]
    pub fn hipsparseSgemvi(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        x: *const f32,
        xInd: *const ::core::ffi::c_int,
        beta: *const f32,
        y: *mut f32,
        idxBase: hipsparseIndexBase_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgemvi(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        x: *const f64,
        xInd: *const ::core::ffi::c_int,
        beta: *const f64,
        y: *mut f64,
        idxBase: hipsparseIndexBase_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgemvi(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const hipComplex,
        A: *const hipComplex,
        lda: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        x: *const hipComplex,
        xInd: *const ::core::ffi::c_int,
        beta: *const hipComplex,
        y: *mut hipComplex,
        idxBase: hipsparseIndexBase_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgemvi(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        A: *const hipDoubleComplex,
        lda: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        x: *const hipDoubleComplex,
        xInd: *const ::core::ffi::c_int,
        beta: *const hipDoubleComplex,
        y: *mut hipDoubleComplex,
        idxBase: hipsparseIndexBase_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level3_module\n  \\brief Sparse matrix dense matrix multiplication using BSR storage format\n\n  \\details\n  \\p hipsparseXbsrmm multiplies the scalar \\f$\\alpha\\f$ with a sparse \\f$mb \\times kb\\f$\n  matrix \\f$A\\f$, defined in BSR storage format, and the dense \\f$k \\times n\\f$\n  matrix \\f$B\\f$ (where \\f$k = block\\_dim \\times kb\\f$) and adds the result to the dense\n  \\f$m \\times n\\f$ matrix \\f$C\\f$ (where \\f$m = block\\_dim \\times mb\\f$) that\n  is multiplied by the scalar \\f$\\beta\\f$, such that\n  \\f[\n    C := \\alpha \\cdot op(A) \\cdot op(B) + \\beta \\cdot C,\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans_A == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n    \\end{array}\n    \\right.\n  \\f]\n  and\n  \\f[\n    op(B) = \\left\\{\n    \\begin{array}{ll}\n        B,   & \\text{if trans_B == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        B^T, & \\text{if trans_B == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n    \\end{array}\n    \\right.\n  \\f]\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\note\n  Currently, only \\p trans_A == \\ref HIPSPARSE_OPERATION_NON_TRANSPOSE is supported.\n\n  \\par Example\n  \\code{.c}\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      //     1 2 0 3 0 0\n      // A = 0 4 5 0 0 0\n      //     0 0 0 7 8 0\n      //     0 0 1 2 4 1\n\n      int block_dim = 2;\n      int mb   = 2;\n      int kb   = 3;\n      int nnzb = 4;\n      hipsparseDirection_t dir = HIPSPARSE_DIRECTION_ROW;\n\n      int hbsr_row_ptr[2 + 1]   = {0, 2, 4};\n      int hbsr_col_ind[4]       = {0, 1, 1, 2};\n      float hbsr_val[4 * 2 * 2] = {1, 2, 0, 4, 0, 3, 5, 0, 0, 7, 1, 2, 8, 0, 4, 1};\n\n      // Set dimension n of B\n      int n = 3;\n      int m = mb * block_dim;\n      int k = kb * block_dim;\n\n      // Allocate and generate dense matrix B (k x n)\n      float hB[6 * 3] = {1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f, 7.0f, 8.0f, 9.0f, 10.0f,\n                      11.0f, 12.0f, 13.0f, 14.0f, 15.0f, 16.0f, 17.0f, 18.0f};\n\n      int* dbsr_row_ptr = NULL;\n      int* dbsr_col_ind = NULL;\n      float* dbsr_val = NULL;\n      hipMalloc((void**)&dbsr_row_ptr, sizeof(int) * (mb + 1));\n      hipMalloc((void**)&dbsr_col_ind, sizeof(int) * nnzb);\n      hipMalloc((void**)&dbsr_val, sizeof(float) * nnzb * block_dim * block_dim);\n      hipMemcpy(dbsr_row_ptr, hbsr_row_ptr, sizeof(int) * (mb + 1), hipMemcpyHostToDevice);\n      hipMemcpy(dbsr_col_ind, hbsr_col_ind, sizeof(int) * nnzb, hipMemcpyHostToDevice);\n      hipMemcpy(dbsr_val, hbsr_val, sizeof(float) * nnzb * block_dim * block_dim, hipMemcpyHostToDevice);\n\n      // Copy B to the device\n      float* dB;\n      hipMalloc((void**)&dB, sizeof(float) * k * n);\n      hipMemcpy(dB, hB, sizeof(float) * k * n, hipMemcpyHostToDevice);\n\n      // alpha and beta\n      float alpha = 1.0f;\n      float beta  = 0.0f;\n\n      // Allocate memory for the resulting matrix C\n      float* dC;\n      hipMalloc((void**)&dC, sizeof(float) * m * n);\n\n      // Matrix descriptor\n      hipsparseMatDescr_t descr;\n      hipsparseCreateMatDescr(&descr);\n\n      // Perform the matrix multiplication\n      hipsparseSbsrmm(handle,\n                      dir,\n                      HIPSPARSE_OPERATION_NON_TRANSPOSE,\n                      HIPSPARSE_OPERATION_NON_TRANSPOSE,\n                      mb,\n                      n,\n                      kb,\n                      nnzb,\n                      &alpha,\n                      descr,\n                      dbsr_val,\n                      dbsr_row_ptr,\n                      dbsr_col_ind,\n                      block_dim,\n                      dB,\n                      k,\n                      &beta,\n                      dC,\n                      m);\n\n      // Copy results to host\n      float hC[6 * 3];\n      hipMemcpy(hC, dC, sizeof(float) * m * n, hipMemcpyDeviceToHost);\n\n      hipFree(dbsr_row_ptr);\n      hipFree(dbsr_col_ind);\n      hipFree(dbsr_val);\n      hipFree(dB);\n      hipFree(dC);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseSbsrmm(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        kb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const f32,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrmm(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        kb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const f64,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrmm(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        kb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const hipComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        B: *const hipComplex,
        ldb: ::core::ffi::c_int,
        beta: *const hipComplex,
        C: *mut hipComplex,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrmm(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        kb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const hipDoubleComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        B: *const hipDoubleComplex,
        ldb: ::core::ffi::c_int,
        beta: *const hipDoubleComplex,
        C: *mut hipDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level3_module\n  \\brief Sparse matrix dense matrix multiplication using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrmm multiplies the scalar \\f$\\alpha\\f$ with a sparse \\f$m \\times k\\f$\n  matrix \\f$A\\f$, defined in CSR storage format, and the dense \\f$k \\times n\\f$\n  matrix \\f$B\\f$ and adds the result to the dense \\f$m \\times n\\f$ matrix \\f$C\\f$ that\n  is multiplied by the scalar \\f$\\beta\\f$, such that\n  \\f[\n    C := \\alpha \\cdot op(A) \\cdot B + \\beta \\cdot C,\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans_A == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        A^T, & \\text{if trans_A == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        A^H, & \\text{if trans_A == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n\n  \\code{.c}\n      for(i = 0; i < ldc; ++i)\n      {\n          for(j = 0; j < n; ++j)\n          {\n              C[i][j] = beta * C[i][j];\n\n              for(k = csr_row_ptr[i]; k < csr_row_ptr[i + 1]; ++k)\n              {\n                  C[i][j] += alpha * csr_val[k] * B[csr_col_ind[k]][j];\n              }\n          }\n      }\n  \\endcode\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\par Example\n  \\code{.c}\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      //     1 2 0 3 0 0\n      // A = 0 4 5 0 0 0\n      //     0 0 0 7 8 0\n      //     0 0 1 2 4 1\n\n      int m   = 4;\n      int k   = 6;\n      int nnz = 11;\n      hipsparseDirection_t dir = HIPSPARSE_DIRECTION_ROW;\n\n      int hcsr_row_ptr[4 + 1] = {0, 3, 5, 7, 11};\n      int hcsr_col_ind[11]    = {0, 1, 3, 1, 2, 3, 4, 2, 3, 4, 5};\n      float hcsr_val[11]      = {1, 2, 3, 4, 5, 7, 8, 1, 2, 4, 1};\n\n      // Set dimension n of B\n      int n = 3;\n\n      // Allocate and generate dense matrix B (k x n)\n      float hB[6 * 3] = {1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f, 7.0f, 8.0f, 9.0f, 10.0f,\n                         11.0f, 12.0f, 13.0f, 14.0f, 15.0f, 16.0f, 17.0f, 18.0f};\n\n      int* dcsr_row_ptr = NULL;\n      int* dcsr_col_ind = NULL;\n      float* dcsr_val = NULL;\n      hipMalloc((void**)&dcsr_row_ptr, sizeof(int) * (m + 1));\n      hipMalloc((void**)&dcsr_col_ind, sizeof(int) * nnz);\n      hipMalloc((void**)&dcsr_val, sizeof(float) * nnz);\n      hipMemcpy(dcsr_row_ptr, hcsr_row_ptr, sizeof(int) * (m + 1), hipMemcpyHostToDevice);\n      hipMemcpy(dcsr_col_ind, hcsr_col_ind, sizeof(int) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dcsr_val, hcsr_val, sizeof(float) * nnz, hipMemcpyHostToDevice);\n\n      // Copy B to the device\n      float* dB;\n      hipMalloc((void**)&dB, sizeof(float) * k * n);\n      hipMemcpy(dB, hB, sizeof(float) * k * n, hipMemcpyHostToDevice);\n\n      // alpha and beta\n      float alpha = 1.0f;\n      float beta  = 0.0f;\n\n      // Allocate memory for the resulting matrix C\n      float* dC;\n      hipMalloc((void**)&dC, sizeof(float) * m * n);\n\n      // Matrix descriptor\n      hipsparseMatDescr_t descr;\n      hipsparseCreateMatDescr(&descr);\n\n      // Perform the matrix multiplication\n      hipsparseScsrmm(handle,\n                      HIPSPARSE_OPERATION_NON_TRANSPOSE,\n                      m,\n                      n,\n                      k,\n                      nnz,\n                      &alpha,\n                      descr,\n                      dcsr_val,\n                      dcsr_row_ptr,\n                      dcsr_col_ind,\n                      dB,\n                      k,\n                      &beta,\n                      dC,\n                      m);\n\n      // Copy results to host\n      float hC[6 * 3];\n      hipMemcpy(hC, dC, sizeof(float) * m * n, hipMemcpyDeviceToHost);\n\n      hipFree(dcsr_row_ptr);\n      hipFree(dcsr_col_ind);\n      hipFree(dcsr_val);\n      hipFree(dB);\n      hipFree(dC);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseScsrmm(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrmm(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrmm(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const hipComplex,
        ldb: ::core::ffi::c_int,
        beta: *const hipComplex,
        C: *mut hipComplex,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrmm(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const hipDoubleComplex,
        ldb: ::core::ffi::c_int,
        beta: *const hipDoubleComplex,
        C: *mut hipDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level3_module\n  \\brief Sparse matrix dense matrix multiplication using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrmm2 multiplies the scalar \\f$\\alpha\\f$ with a sparse \\f$m \\times k\\f$\n  matrix \\f$A\\f$, defined in CSR storage format, and the dense \\f$k \\times n\\f$\n  matrix \\f$B\\f$ and adds the result to the dense \\f$m \\times n\\f$ matrix \\f$C\\f$ that\n  is multiplied by the scalar \\f$\\beta\\f$, such that\n  \\f[\n    C := \\alpha \\cdot op(A) \\cdot op(B) + \\beta \\cdot C,\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans_A == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        A^T, & \\text{if trans_A == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        A^H, & \\text{if trans_A == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n  and\n  \\f[\n    op(B) = \\left\\{\n    \\begin{array}{ll}\n        B,   & \\text{if trans_B == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        B^T, & \\text{if trans_B == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        B^H, & \\text{if trans_B == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n\n  \\code{.c}\n      for(i = 0; i < ldc; ++i)\n      {\n          for(j = 0; j < n; ++j)\n          {\n              C[i][j] = beta * C[i][j];\n\n              for(k = csr_row_ptr[i]; k < csr_row_ptr[i + 1]; ++k)\n              {\n                  C[i][j] += alpha * csr_val[k] * B[csr_col_ind[k]][j];\n              }\n          }\n      }\n  \\endcode\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseScsrmm2(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrmm2(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrmm2(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const hipComplex,
        ldb: ::core::ffi::c_int,
        beta: *const hipComplex,
        C: *mut hipComplex,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrmm2(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const hipDoubleComplex,
        ldb: ::core::ffi::c_int,
        beta: *const hipDoubleComplex,
        C: *mut hipDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup level3_module
  \brief Sparse triangular system solve using BSR storage format

  \details
  \p hipsparseXbsrsm2_zeroPivot returns \ref HIPSPARSE_STATUS_ZERO_PIVOT, if either a
  structural or numerical zero has been found during hipsparseXbsrsm2_analysis() or
  hipsparseXbsrsm2_solve() computation. The first zero pivot \f$j\f$ at \f$A_{j,j}\f$
  is stored in \p position, using same index base as the BSR matrix.

  \p position can be in host or device memory. If no zero pivot has been found,
  \p position is set to -1 and \ref HIPSPARSE_STATUS_SUCCESS is returned instead.

  \note \p hipsparseXbsrsm2_zeroPivot is a blocking function. It might influence
  performance negatively.*/
    pub fn hipsparseXbsrsm2_zeroPivot(
        handle: hipsparseHandle_t,
        info: bsrsm2Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level3_module\n  \\brief Sparse triangular system solve using BSR storage format\n\n  \\details\n  \\p hipsparseXbsrsm2_buffer_size returns the size of the temporary storage buffer in bytes\n  that is required by hipsparseXbsrsm2_analysis() and hipsparseXbsrsm2_solve(). The\n  temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSbsrsm2_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transX: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsm2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrsm2_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transX: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsm2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrsm2_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transX: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut hipComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsm2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrsm2_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transX: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut hipDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsm2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level3_module\n  \\brief Sparse triangular system solve using BSR storage format\n\n  \\details\n  \\p hipsparseXbsrsm2_analysis performs the analysis step for hipsparseXbsrsm2_solve().\n\n  \\note\n  If the matrix sparsity pattern changes, the gathered information will become invalid.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSbsrsm2_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transX: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrsm2_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transX: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrsm2_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transX: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const hipComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrsm2_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transX: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const hipDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level3_module\n  \\brief Sparse triangular system solve using BSR storage format\n\n  \\details\n  \\p hipsparseXbsrsm2_solve solves a sparse triangular linear system of a sparse\n  \\f$m \\times m\\f$ matrix, defined in BSR storage format, a dense solution matrix\n  \\f$X\\f$ and the right-hand side matrix \\f$B\\f$ that is multiplied by \\f$\\alpha\\f$, such that\n  \\f[\n    op(A) \\cdot op(X) = \\alpha \\cdot op(B),\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans_A == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        A^T, & \\text{if trans_A == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        A^H, & \\text{if trans_A == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n  ,\n  \\f[\n    op(X) = \\left\\{\n    \\begin{array}{ll}\n        X,   & \\text{if trans_X == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        X^T, & \\text{if trans_X == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        X^H, & \\text{if trans_X == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n\n  \\p hipsparseXbsrsm2_solve requires a user allocated temporary buffer. Its size is\n  returned by hipsparseXbsrsm2_bufferSize(). Furthermore, analysis meta data is\n  required. It can be obtained by hipsparseXbsrsm2_analysis(). \\p hipsparseXbsrsm2_solve\n  reports the first zero pivot (either numerical or structural zero). The zero pivot\n  status can be checked calling hipsparseXbsrsm2_zeroPivot(). If\n  \\ref hipsparseDiagType_t == \\ref HIPSPARSE_DIAG_TYPE_UNIT, no zero pivot will be\n  reported, even if \\f$A_{j,j} = 0\\f$ for some \\f$j\\f$.\n\n  \\note\n  The sparse BSR matrix has to be sorted.\n\n  \\note\n  Operation type of B and X must match, if \\f$op(B)=B, op(X)=X\\f$.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\note\n  Currently, only \\p trans_A != \\ref HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE and\n  \\p trans_X != \\ref HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE is supported.\n\n  \\par Example\n  \\code{.c}\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      // A = ( 1.0  0.0  0.0  0.0 )\n      //     ( 2.0  3.0  0.0  0.0 )\n      //     ( 4.0  5.0  6.0  0.0 )\n      //     ( 7.0  0.0  8.0  9.0 )\n      //\n      // with bsr_dim = 2\n      //\n      //      -------------------\n      //   = | 1.0 0.0 | 0.0 0.0 |\n      //     | 2.0 3.0 | 0.0 0.0 |\n      //      -------------------\n      //     | 4.0 5.0 | 6.0 0.0 |\n      //     | 7.0 0.0 | 8.0 9.0 |\n      //      -------------------\n\n      // Number of rows and columns\n      int m = 4;\n\n      // Number of block rows and block columns\n      int mb = 2;\n      int nb = 2;\n\n      // BSR block dimension\n      int bsr_dim = 2;\n\n      // Number of right-hand-sides\n      int nrhs = 4;\n\n      // Number of non-zero blocks\n      int nnzb = 3;\n\n      // BSR row pointers\n      int hbsr_row_ptr[3] = {0, 1, 3};\n\n      // BSR column indices\n      int hbsr_col_ind[3] = {0, 0, 1};\n\n      // BSR values\n      double hbsr_val[12] = {1.0, 2.0, 0.0, 3.0, 4.0, 7.0, 5.0, 0.0, 6.0, 8.0, 0.0, 9.0};\n\n      // Storage scheme of the BSR blocks\n      hipsparseDirection_t dir = HIPSPARSE_DIRECTION_COLUMN;\n\n      // Transposition of the matrix and rhs matrix\n      hipsparseOperation_t transA = HIPSPARSE_OPERATION_NON_TRANSPOSE;\n      hipsparseOperation_t transX = HIPSPARSE_OPERATION_NON_TRANSPOSE;\n\n      // Solve policy\n      hipsparseSolvePolicy_t solve_policy = HIPSPARSE_SOLVE_POLICY_NO_LEVEL;\n\n      // Scalar alpha and beta\n      double alpha = 1.0;\n\n      // rhs and solution matrix\n      int ldb = nb * bsr_dim;\n      int ldx = mb * bsr_dim;\n\n      double hB[16] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16};\n      double hX[16];\n\n      // Offload data to device\n      int* dbsr_row_ptr;\n      int* dbsr_col_ind;\n      double*        dbsr_val;\n      double*        dB;\n      double*        dX;\n\n      hipMalloc((void**)&dbsr_row_ptr, sizeof(int) * (mb + 1));\n      hipMalloc((void**)&dbsr_col_ind, sizeof(int) * nnzb);\n      hipMalloc((void**)&dbsr_val, sizeof(double) * nnzb * bsr_dim * bsr_dim);\n      hipMalloc((void**)&dB, sizeof(double) * nb * bsr_dim * nrhs);\n      hipMalloc((void**)&dX, sizeof(double) * mb * bsr_dim * nrhs);\n\n      hipMemcpy(dbsr_row_ptr, hbsr_row_ptr, sizeof(int) * (mb + 1), hipMemcpyHostToDevice);\n      hipMemcpy(dbsr_col_ind, hbsr_col_ind, sizeof(int) * nnzb, hipMemcpyHostToDevice);\n      hipMemcpy(dbsr_val, hbsr_val, sizeof(double) * nnzb * bsr_dim * bsr_dim, hipMemcpyHostToDevice);\n      hipMemcpy(dB, hB, sizeof(double) * nb * bsr_dim * nrhs, hipMemcpyHostToDevice);\n\n      // Matrix descriptor\n      hipsparseMatDescr_t descr;\n      hipsparseCreateMatDescr(&descr);\n\n      // Matrix fill mode\n      hipsparseSetMatFillMode(descr, HIPSPARSE_FILL_MODE_LOWER);\n\n      // Matrix diagonal type\n      hipsparseSetMatDiagType(descr, HIPSPARSE_DIAG_TYPE_NON_UNIT);\n\n      // Matrix info structure\n      bsrsm2Info_t info;\n      hipsparseCreateBsrsm2Info(&info);\n\n      // Obtain required buffer size\n      int buffer_size;\n      hipsparseDbsrsm2_bufferSize(handle,\n                                  dir,\n                                  transA,\n                                  transX,\n                                  mb,\n                                  nrhs,\n                                  nnzb,\n                                  descr,\n                                  dbsr_val,\n                                  dbsr_row_ptr,\n                                  dbsr_col_ind,\n                                  bsr_dim,\n                                  info,\n                                  &buffer_size);\n\n      // Allocate temporary buffer\n      void* dbuffer;\n      hipMalloc(&dbuffer, buffer_size);\n\n      // Perform analysis step\n      hipsparseDbsrsm2_analysis(handle,\n                                dir,\n                                transA,\n                                transX,\n                                mb,\n                                nrhs,\n                                nnzb,\n                                descr,\n                                dbsr_val,\n                                dbsr_row_ptr,\n                                dbsr_col_ind,\n                                bsr_dim,\n                                info,\n                                solve_policy,\n                                dbuffer);\n\n      // Call dbsrsm to perform lower triangular solve LX = B\n      hipsparseDbsrsm2_solve(handle,\n                             dir,\n                             transA,\n                             transX,\n                             mb,\n                             nrhs,\n                             nnzb,\n                             &alpha,\n                             descr,\n                             dbsr_val,\n                             dbsr_row_ptr,\n                             dbsr_col_ind,\n                             bsr_dim,\n                             info,\n                             dB,\n                             ldb,\n                             dX,\n                             ldx,\n                             solve_policy,\n                             dbuffer);\n\n      // Check for zero pivots\n      int    pivot;\n      hipsparseStatus_t status = hipsparseXbsrsm2_zeroPivot(handle, info, &pivot);\n\n      if(status == HIPSPARSE_STATUS_ZERO_PIVOT)\n      {\n          std::cout << \"Found zero pivot in matrix row \" << pivot << std::endl;\n      }\n\n      // Copy result back to host\n      hipMemcpy(hX, dX, sizeof(double) * mb * bsr_dim * nrhs, hipMemcpyDeviceToHost);\n\n      // Clear hipSPARSE\n      hipsparseDestroyBsrsm2Info(info);\n      hipsparseDestroyMatDescr(descr);\n      hipsparseDestroy(handle);\n\n      // Clear device memory\n      hipFree(dbsr_row_ptr);\n      hipFree(dbsr_col_ind);\n      hipFree(dbsr_val);\n      hipFree(dB);\n      hipFree(dX);\n      hipFree(dbuffer);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseSbsrsm2_solve(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transX: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsm2Info_t,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        X: *mut f32,
        ldx: ::core::ffi::c_int,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrsm2_solve(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transX: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsm2Info_t,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        X: *mut f64,
        ldx: ::core::ffi::c_int,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrsm2_solve(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transX: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const hipComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsm2Info_t,
        B: *const hipComplex,
        ldb: ::core::ffi::c_int,
        X: *mut hipComplex,
        ldx: ::core::ffi::c_int,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrsm2_solve(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        transA: hipsparseOperation_t,
        transX: hipsparseOperation_t,
        mb: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *const hipDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrsm2Info_t,
        B: *const hipDoubleComplex,
        ldb: ::core::ffi::c_int,
        X: *mut hipDoubleComplex,
        ldx: ::core::ffi::c_int,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup level3_module
  \brief Sparse triangular system solve using CSR storage format

  \details
  \p hipsparseXcsrsm2_zeroPivot returns \ref HIPSPARSE_STATUS_ZERO_PIVOT, if either a
  structural or numerical zero has been found during hipsparseXcsrsm2_analysis() or
  hipsparseXcsrsm2_solve() computation. The first zero pivot \f$j\f$ at \f$A_{j,j}\f$
  is stored in \p position, using same index base as the CSR matrix.

  \p position can be in host or device memory. If no zero pivot has been found,
  \p position is set to -1 and \ref HIPSPARSE_STATUS_SUCCESS is returned instead.

  \note \p hipsparseXcsrsm2_zeroPivot is a blocking function. It might influence
  performance negatively.*/
    pub fn hipsparseXcsrsm2_zeroPivot(
        handle: hipsparseHandle_t,
        info: csrsm2Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level3_module\n  \\brief Sparse triangular system solve using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrsm2_bufferSizeExt returns the size of the temporary storage buffer\n  in bytes that is required by hipsparseXcsrsm2_analysis() and hipsparseXcsrsm2_solve().\n  The temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseScsrsm2_bufferSizeExt(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        info: csrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrsm2_bufferSizeExt(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        info: csrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrsm2_bufferSizeExt(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const hipComplex,
        ldb: ::core::ffi::c_int,
        info: csrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrsm2_bufferSizeExt(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const hipDoubleComplex,
        ldb: ::core::ffi::c_int,
        info: csrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level3_module\n  \\brief Sparse triangular system solve using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrsm2_analysis performs the analysis step for hipsparseXcsrsm2_solve().\n\n  \\note\n  If the matrix sparsity pattern changes, the gathered information will become invalid.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseScsrsm2_analysis(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        info: csrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrsm2_analysis(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        info: csrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrsm2_analysis(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const hipComplex,
        ldb: ::core::ffi::c_int,
        info: csrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrsm2_analysis(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *const hipDoubleComplex,
        ldb: ::core::ffi::c_int,
        info: csrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level3_module\n  \\brief Sparse triangular system solve using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrsm2_solve solves a sparse triangular linear system of a sparse\n  \\f$m \\times m\\f$ matrix, defined in CSR storage format, a dense solution matrix\n  \\f$X\\f$ and the right-hand side matrix \\f$B\\f$ that is multiplied by \\f$\\alpha\\f$, such that\n  \\f[\n    op(A) \\cdot op(X) = \\alpha \\cdot op(B),\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans_A == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        A^T, & \\text{if trans_A == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        A^H, & \\text{if trans_A == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n  ,\n  \\f[\n    op(B) = \\left\\{\n    \\begin{array}{ll}\n        B,   & \\text{if trans_B == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        B^T, & \\text{if trans_B == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        B^H, & \\text{if trans_B == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n  and\n  \\f[\n    op(X) = \\left\\{\n    \\begin{array}{ll}\n        X,   & \\text{if trans_B == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        X^T, & \\text{if trans_B == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        X^H, & \\text{if trans_B == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n\n  \\p hipsparseXcsrsm2_solve requires a user allocated temporary buffer. Its size is\n  returned by hipsparseXcsrsm2_bufferSizeExt(). Furthermore, analysis meta data is\n  required. It can be obtained by hipsparseXcsrsm2_analysis().\n  \\p hipsparseXcsrsm2_solve reports the first zero pivot (either numerical or structural\n  zero). The zero pivot status can be checked calling hipsparseXcsrsm2_zeroPivot(). If\n  \\ref hipsparseDiagType_t == \\ref HIPSPARSE_DIAG_TYPE_UNIT, no zero pivot will be\n  reported, even if \\f$A_{j,j} = 0\\f$ for some \\f$j\\f$.\n\n  \\note\n  The sparse CSR matrix has to be sorted. This can be achieved by calling\n  hipsparseXcsrsort().\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n\n  \\note\n  Currently, only \\p trans_A != \\ref HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE and\n  \\p trans_B != \\ref HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE is supported.\n\n  \\par Example\n  \\code{.c}\n      // hipSPARSE handle\n      hipsparseHandle_t handle;\n      hipsparseCreate(&handle);\n\n      // A = ( 1.0  0.0  0.0  0.0 )\n      //     ( 2.0  3.0  0.0  0.0 )\n      //     ( 4.0  5.0  6.0  0.0 )\n      //     ( 7.0  0.0  8.0  9.0 )\n\n      // Number of rows and columns\n      int m = 4;\n      int n = 4;\n\n      // Number of right-hand-sides\n      int nrhs = 4;\n\n      // Number of non-zeros\n      int nnz = 9;\n\n      // CSR row pointers\n      int hcsr_row_ptr[5] = {0, 1, 3, 6, 9};\n\n      // CSR column indices\n      int hcsr_col_ind[9] = {0, 0, 1, 0, 1, 2, 0, 2, 3};\n\n      // CSR values\n      double hcsr_val[9] = {1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0};\n\n      // Transposition of the matrix and rhs matrix\n      hipsparseOperation_t transA = HIPSPARSE_OPERATION_NON_TRANSPOSE;\n      hipsparseOperation_t transB = HIPSPARSE_OPERATION_NON_TRANSPOSE;\n\n      // Solve policy\n      hipsparseSolvePolicy_t solve_policy = HIPSPARSE_SOLVE_POLICY_NO_LEVEL;\n\n      // Scalar alpha and beta\n      double alpha = 1.0;\n\n      // rhs and solution matrix\n      int ldb = n;\n\n      double hB[16] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16};\n\n      // Offload data to device\n      int* dcsr_row_ptr;\n      int* dcsr_col_ind;\n      double*        dcsr_val;\n      double*        dB;\n\n      hipMalloc((void**)&dcsr_row_ptr, sizeof(int) * (m + 1));\n      hipMalloc((void**)&dcsr_col_ind, sizeof(int) * nnz);\n      hipMalloc((void**)&dcsr_val, sizeof(double) * nnz);\n      hipMalloc((void**)&dB, sizeof(double) * n * nrhs);\n\n      hipMemcpy(dcsr_row_ptr, hcsr_row_ptr, sizeof(int) * (m + 1), hipMemcpyHostToDevice);\n      hipMemcpy(dcsr_col_ind, hcsr_col_ind, sizeof(int) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dcsr_val, hcsr_val, sizeof(double) * nnz, hipMemcpyHostToDevice);\n      hipMemcpy(dB, hB, sizeof(double) * n * nrhs, hipMemcpyHostToDevice);\n\n      // Matrix descriptor\n      hipsparseMatDescr_t descr;\n      hipsparseCreateMatDescr(&descr);\n\n      // Matrix fill mode\n      hipsparseSetMatFillMode(descr, HIPSPARSE_FILL_MODE_LOWER);\n\n      // Matrix diagonal type\n      hipsparseSetMatDiagType(descr, HIPSPARSE_DIAG_TYPE_NON_UNIT);\n\n      // Matrix info structure\n      csrsm2Info_t info;\n      hipsparseCreateCsrsm2Info(&info);\n\n      // Obtain required buffer size\n      size_t buffer_size;\n      hipsparseDcsrsm2_bufferSizeExt(handle,\n                                     0,\n                                     transA,\n                                     transB,\n                                     m,\n                                     nrhs,\n                                     nnz,\n                                     &alpha,\n                                     descr,\n                                     dcsr_val,\n                                     dcsr_row_ptr,\n                                     dcsr_col_ind,\n                                     dB,\n                                     ldb,\n                                     info,\n                                     solve_policy,\n                                     &buffer_size);\n\n      // Allocate temporary buffer\n      void* dbuffer;\n      hipMalloc(&dbuffer, buffer_size);\n\n      // Perform analysis step\n      hipsparseDcsrsm2_analysis(handle,\n                                0,\n                                transA,\n                                transB,\n                                m,\n                                nrhs,\n                                nnz,\n                                &alpha,\n                                descr,\n                                dcsr_val,\n                                dcsr_row_ptr,\n                                dcsr_col_ind,\n                                dB,\n                                ldb,\n                                info,\n                                solve_policy,\n                                dbuffer);\n\n      // Call dcsrsm to perform lower triangular solve LB = B\n      hipsparseDcsrsm2_solve(handle,\n                             0,\n                             transA,\n                             transB,\n                             m,\n                             nrhs,\n                             nnz,\n                             &alpha,\n                             descr,\n                             dcsr_val,\n                             dcsr_row_ptr,\n                             dcsr_col_ind,\n                             dB,\n                             ldb,\n                             info,\n                             solve_policy,\n                             dbuffer);\n\n      // Check for zero pivots\n      int    pivot;\n      hipsparseStatus_t status = hipsparseXcsrsm2_zeroPivot(handle, info, &pivot);\n\n      if(status == HIPSPARSE_STATUS_ZERO_PIVOT)\n      {\n          std::cout << \"Found zero pivot in matrix row \" << pivot << std::endl;\n      }\n\n      // Copy result back to host\n      hipMemcpy(hB, dB, sizeof(double) * m * nrhs, hipMemcpyDeviceToHost);\n\n      // Clear hipSPARSE\n      hipsparseDestroyCsrsm2Info(info);\n      hipsparseDestroyMatDescr(descr);\n      hipsparseDestroy(handle);\n\n      // Clear device memory\n      hipFree(dcsr_row_ptr);\n      hipFree(dcsr_col_ind);\n      hipFree(dcsr_val);\n      hipFree(dB);\n      hipFree(dbuffer);\n  \\endcode\n/\n/**@{"]
    pub fn hipsparseScsrsm2_solve(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *mut f32,
        ldb: ::core::ffi::c_int,
        info: csrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrsm2_solve(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *mut f64,
        ldb: ::core::ffi::c_int,
        info: csrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrsm2_solve(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *mut hipComplex,
        ldb: ::core::ffi::c_int,
        info: csrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrsm2_solve(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        B: *mut hipDoubleComplex,
        ldb: ::core::ffi::c_int,
        info: csrsm2Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup level3_module\n  \\brief Dense matrix sparse matrix multiplication using CSR storage format\n\n  \\details\n  \\p hipsparseXgemmi multiplies the scalar \\f$\\alpha\\f$ with a dense \\f$m \\times k\\f$\n  matrix \\f$A\\f$ and the sparse \\f$k \\times n\\f$ matrix \\f$B\\f$, defined in CSR\n  storage format and adds the result to the dense \\f$m \\times n\\f$ matrix \\f$C\\f$ that\n  is multiplied by the scalar \\f$\\beta\\f$, such that\n  \\f[\n    C := \\alpha \\cdot op(A) \\cdot op(B) + \\beta \\cdot C\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans_A == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        A^T, & \\text{if trans_A == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        A^H, & \\text{if trans_A == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n  and\n  \\f[\n    op(B) = \\left\\{\n    \\begin{array}{ll}\n        B,   & \\text{if trans_B == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        B^T, & \\text{if trans_B == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        B^H, & \\text{if trans_B == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSgemmi(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        cscValB: *const f32,
        cscColPtrB: *const ::core::ffi::c_int,
        cscRowIndB: *const ::core::ffi::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgemmi(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        cscValB: *const f64,
        cscColPtrB: *const ::core::ffi::c_int,
        cscRowIndB: *const ::core::ffi::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgemmi(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipComplex,
        A: *const hipComplex,
        lda: ::core::ffi::c_int,
        cscValB: *const hipComplex,
        cscColPtrB: *const ::core::ffi::c_int,
        cscRowIndB: *const ::core::ffi::c_int,
        beta: *const hipComplex,
        C: *mut hipComplex,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgemmi(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        A: *const hipDoubleComplex,
        lda: ::core::ffi::c_int,
        cscValB: *const hipDoubleComplex,
        cscColPtrB: *const ::core::ffi::c_int,
        cscRowIndB: *const ::core::ffi::c_int,
        beta: *const hipDoubleComplex,
        C: *mut hipDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup extra_module
  \brief Sparse matrix sparse matrix addition using CSR storage format

  \details
  \p hipsparseXcsrgeamNnz computes the total CSR non-zero elements and the CSR row
  offsets, that point to the start of every row of the sparse CSR matrix, of the
  resulting matrix C. It is assumed that \p csr_row_ptr_C has been allocated with
  size \p m + 1.

  \note
  This function is non blocking and executed asynchronously with respect to the host.
  It may return before the actual computation has finished.
  \note
  Currently, only \ref HIPSPARSE_MATRIX_TYPE_GENERAL is supported.*/
    pub fn hipsparseXcsrgeamNnz(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup extra_module\n  \\brief Sparse matrix sparse matrix addition using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrgeam multiplies the scalar \\f$\\alpha\\f$ with the sparse\n  \\f$m \\times n\\f$ matrix \\f$A\\f$, defined in CSR storage format, multiplies the\n  scalar \\f$\\beta\\f$ with the sparse \\f$m \\times n\\f$ matrix \\f$B\\f$, defined in CSR\n  storage format, and adds both resulting matrices to obtain the sparse\n  \\f$m \\times n\\f$ matrix \\f$C\\f$, defined in CSR storage format, such that\n  \\f[\n    C := \\alpha \\cdot A + \\beta \\cdot B.\n  \\f]\n\n  It is assumed that \\p csr_row_ptr_C has already been filled and that \\p csr_val_C and\n  \\p csr_col_ind_C are allocated by the user. \\p csr_row_ptr_C and allocation size of\n  \\p csr_col_ind_C and \\p csr_val_C is defined by the number of non-zero elements of\n  the sparse CSR matrix C. Both can be obtained by hipsparseXcsrgeamNnz().\n\n  \\note Both scalars \\f$\\alpha\\f$ and \\f$beta\\f$ have to be valid.\n  \\note Currently, only \\ref HIPSPARSE_MATRIX_TYPE_GENERAL is supported.\n  \\note This function is non blocking and executed asynchronously with respect to the\n        host. It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseScsrgeam(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        beta: *const f32,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrValB: *const f32,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f32,
        csrRowPtrC: *mut ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrgeam(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        beta: *const f64,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrValB: *const f64,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f64,
        csrRowPtrC: *mut ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrgeam(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrValA: *const hipComplex,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        beta: *const hipComplex,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrValB: *const hipComplex,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut hipComplex,
        csrRowPtrC: *mut ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrgeam(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrValA: *const hipDoubleComplex,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        beta: *const hipDoubleComplex,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrValB: *const hipDoubleComplex,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut hipDoubleComplex,
        csrRowPtrC: *mut ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup extra_module\n  \\brief Sparse matrix sparse matrix multiplication using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrgeam2_bufferSizeExt returns the size of the temporary storage buffer\n  in bytes that is required by hipsparseXcsrgeam2Nnz() and hipsparseXcsrgeam2(). The\n  temporary storage buffer must be allocated by the user.\n\n  \\note\n  Currently, only \\ref HIPSPARSE_MATRIX_TYPE_GENERAL is supported.\n/\n/**@{"]
    pub fn hipsparseScsrgeam2_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const f32,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const f32,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrSortedValC: *const f32,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrgeam2_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const f64,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const f64,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrSortedValC: *const f64,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrgeam2_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const hipComplex,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const hipComplex,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrSortedValC: *const hipComplex,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrgeam2_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const hipDoubleComplex,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const hipDoubleComplex,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrSortedValC: *const hipDoubleComplex,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup extra_module
  \brief Sparse matrix sparse matrix addition using CSR storage format

  \details
  \p hipsparseXcsrgeam2Nnz computes the total CSR non-zero elements and the CSR row
  offsets, that point to the start of every row of the sparse CSR matrix, of the
  resulting matrix C. It is assumed that \p csr_row_ptr_C has been allocated with
  size \p m + 1.

  \note
  This function is non blocking and executed asynchronously with respect to the host.
  It may return before the actual computation has finished.
  \note
  Currently, only \ref HIPSPARSE_MATRIX_TYPE_GENERAL is supported.*/
    pub fn hipsparseXcsrgeam2Nnz(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        workspace: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup extra_module\n  \\brief Sparse matrix sparse matrix addition using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrgeam2 multiplies the scalar \\f$\\alpha\\f$ with the sparse\n  \\f$m \\times n\\f$ matrix \\f$A\\f$, defined in CSR storage format, multiplies the\n  scalar \\f$\\beta\\f$ with the sparse \\f$m \\times n\\f$ matrix \\f$B\\f$, defined in CSR\n  storage format, and adds both resulting matrices to obtain the sparse\n  \\f$m \\times n\\f$ matrix \\f$C\\f$, defined in CSR storage format, such that\n  \\f[\n    C := \\alpha \\cdot A + \\beta \\cdot B.\n  \\f]\n\n  It is assumed that \\p csr_row_ptr_C has already been filled and that \\p csr_val_C and\n  \\p csr_col_ind_C are allocated by the user. \\p csr_row_ptr_C and allocation size of\n  \\p csr_col_ind_C and \\p csr_val_C is defined by the number of non-zero elements of\n  the sparse CSR matrix C. Both can be obtained by hipsparseXcsrgeam2Nnz().\n\n  \\note Both scalars \\f$\\alpha\\f$ and \\f$beta\\f$ have to be valid.\n  \\note Currently, only \\ref HIPSPARSE_MATRIX_TYPE_GENERAL is supported.\n  \\note This function is non blocking and executed asynchronously with respect to the\n        host. It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseScsrgeam2(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const f32,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const f32,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrSortedValC: *mut f32,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrgeam2(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const f64,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const f64,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrSortedValC: *mut f64,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrgeam2(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const hipComplex,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const hipComplex,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrSortedValC: *mut hipComplex,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrgeam2(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const hipDoubleComplex,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const hipDoubleComplex,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrSortedValC: *mut hipDoubleComplex,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup extra_module
  \brief Sparse matrix sparse matrix multiplication using CSR storage format

  \details
  \p hipsparseXcsrgemmNnz computes the total CSR non-zero elements and the CSR row
  offsets, that point to the start of every row of the sparse CSR matrix, of the
  resulting multiplied matrix C. It is assumed that \p csr_row_ptr_C has been allocated
  with size \p m + 1.

  \note
  This function is non blocking and executed asynchronously with respect to the host.
  It may return before the actual computation has finished.

  \note
  Please note, that for matrix products with more than 8192 intermediate products per
  row, additional temporary storage buffer is allocated by the algorithm.

  \note
  Currently, only \p trans_A == \p trans_B == \ref HIPSPARSE_OPERATION_NON_TRANSPOSE is
  supported.

  \note
  Currently, only \ref HIPSPARSE_MATRIX_TYPE_GENERAL is supported.*/
    pub fn hipsparseXcsrgemmNnz(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup extra_module\n  \\brief Sparse matrix sparse matrix multiplication using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrgemm multiplies the sparse \\f$m \\times k\\f$ matrix \\f$A\\f$, defined in\n  CSR storage format with the sparse \\f$k \\times n\\f$ matrix \\f$B\\f$, defined in CSR\n  storage format, and stores the result in the sparse \\f$m \\times n\\f$ matrix \\f$C\\f$,\n  defined in CSR storage format, such that\n  \\f[\n    C := op(A) \\cdot op(B),\n  \\f]\n  with\n  \\f[\n    op(A) = \\left\\{\n    \\begin{array}{ll}\n        A,   & \\text{if trans_A == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        A^T, & \\text{if trans_A == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        A^H, & \\text{if trans_A == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n  and\n  \\f[\n    op(B) = \\left\\{\n    \\begin{array}{ll}\n        B,   & \\text{if trans_B == HIPSPARSE_OPERATION_NON_TRANSPOSE} \\\\\n        B^T, & \\text{if trans_B == HIPSPARSE_OPERATION_TRANSPOSE} \\\\\n        B^H, & \\text{if trans_B == HIPSPARSE_OPERATION_CONJUGATE_TRANSPOSE}\n    \\end{array}\n    \\right.\n  \\f]\n\n  It is assumed that \\p csr_row_ptr_C has already been filled and that \\p csr_val_C and\n  \\p csr_col_ind_C are allocated by the user. \\p csr_row_ptr_C and allocation size of\n  \\p csr_col_ind_C and \\p csr_val_C is defined by the number of non-zero elements of\n  the sparse CSR matrix C. Both can be obtained by hipsparseXcsrgemmNnz().\n\n  \\note Currently, only \\p trans_A == \\ref HIPSPARSE_OPERATION_NON_TRANSPOSE is supported.\n  \\note Currently, only \\p trans_B == \\ref HIPSPARSE_OPERATION_NON_TRANSPOSE is supported.\n  \\note Currently, only \\ref HIPSPARSE_MATRIX_TYPE_GENERAL is supported.\n  \\note This function is non blocking and executed asynchronously with respect to the\n        host. It may return before the actual computation has finished.\n  \\note Please note, that for matrix products with more than 4096 non-zero entries per\n  row, additional temporary storage buffer is allocated by the algorithm.\n/\n/**@{"]
    pub fn hipsparseScsrgemm(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrValB: *const f32,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f32,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrgemm(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrValB: *const f64,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f64,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrgemm(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrValA: *const hipComplex,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrValB: *const hipComplex,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut hipComplex,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrgemm(
        handle: hipsparseHandle_t,
        transA: hipsparseOperation_t,
        transB: hipsparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrValA: *const hipDoubleComplex,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrValB: *const hipDoubleComplex,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut hipDoubleComplex,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup extra_module\n  \\brief Sparse matrix sparse matrix multiplication using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrgemm2_bufferSizeExt returns the size of the temporary storage buffer\n  in bytes that is required by hipsparseXcsrgemm2Nnz() and hipsparseXcsrgemm2(). The\n  temporary storage buffer must be allocated by the user.\n\n  \\note\n  Please note, that for matrix products with more than 4096 non-zero entries per row,\n  additional temporary storage buffer is allocated by the algorithm.\n\n  \\note\n  Please note, that for matrix products with more than 8192 intermediate products per\n  row, additional temporary storage buffer is allocated by the algorithm.\n\n  \\note\n  Currently, only \\ref HIPSPARSE_MATRIX_TYPE_GENERAL is supported.\n/\n/**@{"]
    pub fn hipsparseScsrgemm2_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        beta: *const f32,
        descrD: hipsparseMatDescr_t,
        nnzD: ::core::ffi::c_int,
        csrRowPtrD: *const ::core::ffi::c_int,
        csrColIndD: *const ::core::ffi::c_int,
        info: csrgemm2Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrgemm2_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        beta: *const f64,
        descrD: hipsparseMatDescr_t,
        nnzD: ::core::ffi::c_int,
        csrRowPtrD: *const ::core::ffi::c_int,
        csrColIndD: *const ::core::ffi::c_int,
        info: csrgemm2Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrgemm2_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        beta: *const hipComplex,
        descrD: hipsparseMatDescr_t,
        nnzD: ::core::ffi::c_int,
        csrRowPtrD: *const ::core::ffi::c_int,
        csrColIndD: *const ::core::ffi::c_int,
        info: csrgemm2Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrgemm2_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        beta: *const hipDoubleComplex,
        descrD: hipsparseMatDescr_t,
        nnzD: ::core::ffi::c_int,
        csrRowPtrD: *const ::core::ffi::c_int,
        csrColIndD: *const ::core::ffi::c_int,
        info: csrgemm2Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup extra_module
  \brief Sparse matrix sparse matrix multiplication using CSR storage format

  \details
  \p hipsparseXcsrgemm2Nnz computes the total CSR non-zero elements and the CSR row
  offsets, that point to the start of every row of the sparse CSR matrix, of the
  resulting multiplied matrix C. It is assumed that \p csr_row_ptr_C has been allocated
  with size \p m + 1.
  The required buffer size can be obtained by hipsparseXcsrgemm2_bufferSizeExt().

  \note
  This function is non blocking and executed asynchronously with respect to the host.
  It may return before the actual computation has finished.

  \note
  Please note, that for matrix products with more than 8192 intermediate products per
  row, additional temporary storage buffer is allocated by the algorithm.

  \note
  Currently, only \ref HIPSPARSE_MATRIX_TYPE_GENERAL is supported.*/
    pub fn hipsparseXcsrgemm2Nnz(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        descrD: hipsparseMatDescr_t,
        nnzD: ::core::ffi::c_int,
        csrRowPtrD: *const ::core::ffi::c_int,
        csrColIndD: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        info: csrgemm2Info_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup extra_module\n  \\brief Sparse matrix sparse matrix multiplication using CSR storage format\n\n  \\details\n  \\p hipsparseXcsrgemm2 multiplies the scalar \\f$\\alpha\\f$ with the sparse\n  \\f$m \\times k\\f$ matrix \\f$A\\f$, defined in CSR storage format, and the sparse\n  \\f$k \\times n\\f$ matrix \\f$B\\f$, defined in CSR storage format, and adds the result\n  to the sparse \\f$m \\times n\\f$ matrix \\f$D\\f$ that is multiplied by \\f$\\beta\\f$. The\n  final result is stored in the sparse \\f$m \\times n\\f$ matrix \\f$C\\f$, defined in CSR\n  storage format, such\n  that\n  \\f[\n    C := \\alpha \\cdot A \\cdot B + \\beta \\cdot D\n  \\f]\n\n  It is assumed that \\p csr_row_ptr_C has already been filled and that \\p csr_val_C and\n  \\p csr_col_ind_C are allocated by the user. \\p csr_row_ptr_C and allocation size of\n  \\p csr_col_ind_C and \\p csr_val_C is defined by the number of non-zero elements of\n  the sparse CSR matrix C. Both can be obtained by hipsparseXcsrgemm2Nnz(). The\n  required buffer size for the computation can be obtained by\n  hipsparseXcsrgemm2_bufferSizeExt().\n\n  \\note If \\f$\\alpha == 0\\f$, then \\f$C = \\beta \\cdot D\\f$ will be computed.\n  \\note If \\f$\\beta == 0\\f$, then \\f$C = \\alpha \\cdot A \\cdot B\\f$ will be computed.\n  \\note \\f$\\alpha == beta == 0\\f$ is invalid.\n  \\note Currently, only \\ref HIPSPARSE_MATRIX_TYPE_GENERAL is supported.\n  \\note This function is non blocking and executed asynchronously with respect to the\n        host. It may return before the actual computation has finished.\n  \\note Please note, that for matrix products with more than 4096 non-zero entries per\n  row, additional temporary storage buffer is allocated by the algorithm.\n/\n/**@{"]
    pub fn hipsparseScsrgemm2(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrValB: *const f32,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        beta: *const f32,
        descrD: hipsparseMatDescr_t,
        nnzD: ::core::ffi::c_int,
        csrValD: *const f32,
        csrRowPtrD: *const ::core::ffi::c_int,
        csrColIndD: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f32,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
        info: csrgemm2Info_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrgemm2(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrValB: *const f64,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        beta: *const f64,
        descrD: hipsparseMatDescr_t,
        nnzD: ::core::ffi::c_int,
        csrValD: *const f64,
        csrRowPtrD: *const ::core::ffi::c_int,
        csrColIndD: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f64,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
        info: csrgemm2Info_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrgemm2(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const hipComplex,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrValA: *const hipComplex,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrValB: *const hipComplex,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        beta: *const hipComplex,
        descrD: hipsparseMatDescr_t,
        nnzD: ::core::ffi::c_int,
        csrValD: *const hipComplex,
        csrRowPtrD: *const ::core::ffi::c_int,
        csrColIndD: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut hipComplex,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
        info: csrgemm2Info_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrgemm2(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const hipDoubleComplex,
        descrA: hipsparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrValA: *const hipDoubleComplex,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        descrB: hipsparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrValB: *const hipDoubleComplex,
        csrRowPtrB: *const ::core::ffi::c_int,
        csrColIndB: *const ::core::ffi::c_int,
        beta: *const hipDoubleComplex,
        descrD: hipsparseMatDescr_t,
        nnzD: ::core::ffi::c_int,
        csrValD: *const hipDoubleComplex,
        csrRowPtrD: *const ::core::ffi::c_int,
        csrColIndD: *const ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut hipDoubleComplex,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
        info: csrgemm2Info_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup precond_module
  \brief Incomplete LU factorization with 0 fill-ins and no pivoting using BSR storage
  format

  \details
  \p hipsparseXbsrilu02_zeroPivot returns \ref HIPSPARSE_STATUS_ZERO_PIVOT, if either a
  structural or numerical zero has been found during hipsparseXbsrilu02_analysis() or
  hipsparseXbsrilu02() computation. The first zero pivot \f$j\f$ at \f$A_{j,j}\f$ is
  stored in \p position, using same index base as the BSR matrix.

  \p position can be in host or device memory. If no zero pivot has been found,
  \p position is set to -1 and \ref HIPSPARSE_STATUS_SUCCESS is returned instead.

  \note
  If a zero pivot is found, \p position \f$=j\f$ means that either the diagonal block
  \f$A_{j,j}\f$ is missing (structural zero) or the diagonal block \f$A_{j,j}\f$ is not
  invertible (numerical zero).

  \note \p hipsparseXbsrilu02_zeroPivot is a blocking function. It might influence
  performance negatively.*/
    pub fn hipsparseXbsrilu02_zeroPivot(
        handle: hipsparseHandle_t,
        info: bsrilu02Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete LU factorization with 0 fill-ins and no pivoting using BSR storage\n  format\n\n  \\details\n  \\p hipsparseXbsrilu02_numericBoost enables the user to replace a numerical value in\n  an incomplete LU factorization. \\p tol is used to determine whether a numerical value\n  is replaced by \\p boost_val, such that \\f$A_{j,j} = \\text{boost_val}\\f$ if\n  \\f$\\text{tol} \\ge \\left|A_{j,j}\\right|\\f$.\n\n  \\note The boost value is enabled by setting \\p enable_boost to 1 or disabled by\n  setting \\p enable_boost to 0.\n\n  \\note \\p tol and \\p boost_val can be in host or device memory.\n/\n/**@{"]
    pub fn hipsparseSbsrilu02_numericBoost(
        handle: hipsparseHandle_t,
        info: bsrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut f32,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrilu02_numericBoost(
        handle: hipsparseHandle_t,
        info: bsrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut f64,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrilu02_numericBoost(
        handle: hipsparseHandle_t,
        info: bsrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut hipComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrilu02_numericBoost(
        handle: hipsparseHandle_t,
        info: bsrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut hipDoubleComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete LU factorization with 0 fill-ins and no pivoting using BSR storage\n  format\n\n  \\details\n  \\p hipsparseXbsrilu02_bufferSize returns the size of the temporary storage buffer\n  in bytes that is required by hipsparseXbsrilu02_analysis() and hipsparseXbsrilu02().\n  The temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSbsrilu02_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrilu02_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrilu02_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut hipComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrilu02_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut hipDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete LU factorization with 0 fill-ins and no pivoting using BSR storage\n  format\n\n  \\details\n  \\p hipsparseXbsrilu02_analysis performs the analysis step for hipsparseXbsrilu02().\n\n  \\note\n  If the matrix sparsity pattern changes, the gathered information will become invalid.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSbsrilu02_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrilu02_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrilu02_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut hipComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrilu02_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA: *mut hipDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete LU factorization with 0 fill-ins and no pivoting using BSR storage\n  format\n\n  \\details\n  \\p hipsparseXbsrilu02 computes the incomplete LU factorization with 0 fill-ins and no\n  pivoting of a sparse \\f$mb \\times mb\\f$ BSR matrix \\f$A\\f$, such that\n  \\f[\n    A \\approx LU\n  \\f]\n\n  \\p hipsparseXbsrilu02 requires a user allocated temporary buffer. Its size is\n  returned by hipsparseXbsrilu02_bufferSize(). Furthermore, analysis meta data is\n  required. It can be obtained by hipsparseXbsrilu02_analysis(). \\p hipsparseXbsrilu02\n  reports the first zero pivot (either numerical or structural zero). The zero pivot\n  status can be obtained by calling hipsparseXbsrilu02_zeroPivot().\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSbsrilu02(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA_valM: *mut f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsrilu02(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA_valM: *mut f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsrilu02(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA_valM: *mut hipComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsrilu02(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrSortedValA_valM: *mut hipDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup precond_module
  \brief Incomplete LU factorization with 0 fill-ins and no pivoting using CSR
  storage format

  \details
  \p hipsparseXcsrilu02_zeroPivot returns \ref HIPSPARSE_STATUS_ZERO_PIVOT, if either a
  structural or numerical zero has been found during hipsparseXcsrilu02() computation.
  The first zero pivot \f$j\f$ at \f$A_{j,j}\f$ is stored in \p position, using same
  index base as the CSR matrix.

  \p position can be in host or device memory. If no zero pivot has been found,
  \p position is set to -1 and \ref HIPSPARSE_STATUS_SUCCESS is returned instead.

  \note \p hipsparseXcsrilu02_zeroPivot is a blocking function. It might influence
  performance negatively.*/
    pub fn hipsparseXcsrilu02_zeroPivot(
        handle: hipsparseHandle_t,
        info: csrilu02Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete LU factorization with 0 fill-ins and no pivoting using CSR storage\n  format\n\n  \\details\n  \\p hipsparseXcsrilu02_numericBoost enables the user to replace a numerical value in\n  an incomplete LU factorization. \\p tol is used to determine whether a numerical value\n  is replaced by \\p boost_val, such that \\f$A_{j,j} = \\text{boost_val}\\f$ if\n  \\f$\\text{tol} \\ge \\left|A_{j,j}\\right|\\f$.\n\n  \\note The boost value is enabled by setting \\p enable_boost to 1 or disabled by\n  setting \\p enable_boost to 0.\n\n  \\note \\p tol and \\p boost_val can be in host or device memory.\n/\n/**@{"]
    pub fn hipsparseScsrilu02_numericBoost(
        handle: hipsparseHandle_t,
        info: csrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut f32,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrilu02_numericBoost(
        handle: hipsparseHandle_t,
        info: csrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut f64,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrilu02_numericBoost(
        handle: hipsparseHandle_t,
        info: csrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut hipComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrilu02_numericBoost(
        handle: hipsparseHandle_t,
        info: csrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut hipDoubleComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete LU factorization with 0 fill-ins and no pivoting using CSR\n  storage format\n\n  \\details\n  \\p hipsparseXcsrilu02_bufferSize returns the size of the temporary storage buffer\n  in bytes that is required by hipsparseXcsrilu02_analysis() and hipsparseXcsrilu02_solve().\n  The temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseScsrilu02_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrilu02_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrilu02_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrilu02_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete LU factorization with 0 fill-ins and no pivoting using CSR\n  storage format\n\n  \\details\n  \\p hipsparseXcsrilu02_bufferSizeExt returns the size of the temporary storage buffer\n  in bytes that is required by hipsparseXcsrilu02_analysis() and hipsparseXcsrilu02_solve().\n  The temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseScsrilu02_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrilu02_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrilu02_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrilu02_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete LU factorization with 0 fill-ins and no pivoting using CSR\n  storage format\n\n  \\details\n  \\p hipsparseXcsrilu02_analysis performs the analysis step for hipsparseXcsrilu02().\n\n  \\note\n  If the matrix sparsity pattern changes, the gathered information will become invalid.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseScsrilu02_analysis(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrilu02_analysis(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrilu02_analysis(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrilu02_analysis(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete LU factorization with 0 fill-ins and no pivoting using CSR\n  storage format\n\n  \\details\n  \\p hipsparseXcsrilu02 computes the incomplete LU factorization with 0 fill-ins and no\n  pivoting of a sparse \\f$m \\times m\\f$ CSR matrix \\f$A\\f$, such that\n  \\f[\n    A \\approx LU\n  \\f]\n\n  \\p hipsparseXcsrilu02 requires a user allocated temporary buffer. Its size is returned\n  by hipsparseXcsrilu02_bufferSize() or hipsparseXcsrilu02_bufferSizeExt(). Furthermore,\n  analysis meta data is required. It can be obtained by hipsparseXcsrilu02_analysis().\n  \\p hipsparseXcsrilu02 reports the first zero pivot (either numerical or structural\n  zero). The zero pivot status can be obtained by calling hipsparseXcsrilu02_zeroPivot().\n\n  \\note\n  The sparse CSR matrix has to be sorted. This can be achieved by calling\n  hipsparseXcsrsort().\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseScsrilu02(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA_valM: *mut f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrilu02(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA_valM: *mut f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrilu02(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA_valM: *mut hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrilu02(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA_valM: *mut hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csrilu02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup precond_module
  \brief Incomplete Cholesky factorization with 0 fill-ins and no pivoting using BSR
  storage format

  \details
  \p hipsparseXbsric02_zeroPivot returns \ref HIPSPARSE_STATUS_ZERO_PIVOT, if either a
  structural or numerical zero has been found during hipsparseXbsric02_analysis() or
  hipsparseXbsric02() computation. The first zero pivot \f$j\f$ at \f$A_{j,j}\f$ is
  stored in \p position, using same index base as the BSR matrix.

  \p position can be in host or device memory. If no zero pivot has been found,
  \p position is set to -1 and \ref HIPSPARSE_STATUS_SUCCESS is returned instead.

  \note
  If a zero pivot is found, \p position=j means that either the diagonal block \p A(j,j)
  is missing (structural zero) or the diagonal block \p A(j,j) is not positive definite
  (numerical zero).

  \note \p hipsparseXbsric02_zeroPivot is a blocking function. It might influence
  performance negatively.*/
    pub fn hipsparseXbsric02_zeroPivot(
        handle: hipsparseHandle_t,
        info: bsric02Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete Cholesky factorization with 0 fill-ins and no pivoting using BSR\n  storage format\n\n  \\details\n  \\p hipsparseXbsric02_bufferSize returns the size of the temporary storage buffer\n  in bytes that is required by hipsparseXbsric02_analysis() and hipsparseXbsric02().\n  The temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSbsric02_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *mut f32,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsric02_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *mut f64,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsric02_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *mut hipComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsric02_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *mut hipDoubleComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete Cholesky factorization with 0 fill-ins and no pivoting using BSR\n  storage format\n\n  \\details\n  \\p hipsparseXbsric02_analysis performs the analysis step for hipsparseXbsric02().\n\n  \\note\n  If the matrix sparsity pattern changes, the gathered information will become invalid.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSbsric02_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const f32,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsric02_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const f64,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsric02_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const hipComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsric02_analysis(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const hipDoubleComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete Cholesky factorization with 0 fill-ins and no pivoting using BSR\n  storage format\n\n  \\details\n  \\p hipsparseXbsric02 computes the incomplete Cholesky factorization with 0 fill-ins\n  and no pivoting of a sparse \\f$mb \\times mb\\f$ BSR matrix \\f$A\\f$, such that\n  \\f[\n    A \\approx LL^T\n  \\f]\n\n  \\p hipsparseXbsric02 requires a user allocated temporary buffer. Its size is returned\n  by hipsparseXbsric02_bufferSize(). Furthermore, analysis meta data is required. It\n  can be obtained by hipsparseXbsric02_analysis(). \\p hipsparseXbsric02 reports the\n  first zero pivot (either numerical or structural zero). The zero pivot status can be\n  obtained by calling hipsparseXbsric02_zeroPivot().\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSbsric02(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *mut f32,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsric02(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *mut f64,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsric02(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *mut hipComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsric02(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *mut hipDoubleComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: bsric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup precond_module
  \brief Incomplete Cholesky factorization with 0 fill-ins and no pivoting using CSR
  storage format

  \details
  \p hipsparseXcsric02_zeroPivot returns \ref HIPSPARSE_STATUS_ZERO_PIVOT, if either a
  structural or numerical zero has been found during hipsparseXcsric02_analysis() or
  hipsparseXcsric02() computation. The first zero pivot \f$j\f$ at \f$A_{j,j}\f$
  is stored in \p position, using same index base as the CSR matrix.

  \p position can be in host or device memory. If no zero pivot has been found,
  \p position is set to -1 and \ref HIPSPARSE_STATUS_SUCCESS is returned instead.

  \note \p hipsparseXcsric02_zeroPivot is a blocking function. It might influence
  performance negatively.*/
    pub fn hipsparseXcsric02_zeroPivot(
        handle: hipsparseHandle_t,
        info: csric02Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete Cholesky factorization with 0 fill-ins and no pivoting using CSR\n  storage format\n\n  \\details\n  \\p hipsparseXcsric02_bufferSize returns the size of the temporary storage buffer in bytes\n  that is required by hipsparseXcsric02_analysis() and hipsparseXcsric02().\n/\n/**@{"]
    pub fn hipsparseScsric02_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsric02_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsric02_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsric02_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete Cholesky factorization with 0 fill-ins and no pivoting using CSR\n  storage format\n\n  \\details\n  \\p hipsparseXcsric02_bufferSizeExt returns the size of the temporary storage buffer\n  in bytes that is required by hipsparseXcsric02_analysis() and hipsparseXcsric02().\n/\n/**@{"]
    pub fn hipsparseScsric02_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsric02_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsric02_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsric02_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *mut hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete Cholesky factorization with 0 fill-ins and no pivoting using CSR\n  storage format\n\n  \\details\n  \\p hipsparseXcsric02_analysis performs the analysis step for hipsparseXcsric02().\n\n  \\note\n  If the matrix sparsity pattern changes, the gathered information will become invalid.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseScsric02_analysis(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsric02_analysis(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsric02_analysis(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsric02_analysis(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Incomplete Cholesky factorization with 0 fill-ins and no pivoting using CSR\n  storage format\n\n  \\details\n  \\p hipsparseXcsric02 computes the incomplete Cholesky factorization with 0 fill-ins\n  and no pivoting of a sparse \\f$m \\times m\\f$ CSR matrix \\f$A\\f$, such that\n  \\f[\n    A \\approx LL^T\n  \\f]\n\n  \\p hipsparseXcsric02 requires a user allocated temporary buffer. Its size is returned\n  by hipsparseXcsric02_bufferSize() or hipsparseXcsric02_bufferSizeExt(). Furthermore,\n  analysis meta data is required. It can be obtained by hipsparseXcsric02_analysis().\n  \\p hipsparseXcsric02 reports the first zero pivot (either numerical or structural\n  zero). The zero pivot status can be obtained by calling hipsparseXcsric02_zeroPivot().\n\n  \\note\n  The sparse CSR matrix has to be sorted. This can be achieved by calling\n  hipsparseXcsrsort().\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseScsric02(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA_valM: *mut f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsric02(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA_valM: *mut f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsric02(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA_valM: *mut hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsric02(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA_valM: *mut hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: csric02Info_t,
        policy: hipsparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Tridiagonal solver with pivoting\n\n  \\details\n  \\p hipsparseXgtsv2_bufferSize returns the size of the temporary storage buffer\n  in bytes that is required by hipsparseXgtsv2(). The temporary storage buffer must\n  be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSgtsv2_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgtsv2_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        B: *const f64,
        db: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgtsv2_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const hipComplex,
        d: *const hipComplex,
        du: *const hipComplex,
        B: *const hipComplex,
        ldb: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgtsv2_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const hipDoubleComplex,
        d: *const hipDoubleComplex,
        du: *const hipDoubleComplex,
        B: *const hipDoubleComplex,
        ldb: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Tridiagonal solver with pivoting\n\n  \\details\n  \\p hipsparseXgtsv2 solves a tridiagonal system for multiple right hand sides using pivoting.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSgtsv2(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        B: *mut f32,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgtsv2(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        B: *mut f64,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgtsv2(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const hipComplex,
        d: *const hipComplex,
        du: *const hipComplex,
        B: *mut hipComplex,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgtsv2(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const hipDoubleComplex,
        d: *const hipDoubleComplex,
        du: *const hipDoubleComplex,
        B: *mut hipDoubleComplex,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Tridiagonal solver (no pivoting)\n\n  \\details\n  \\p hipsparseXgtsv2_nopivot_bufferSizeExt returns the size of the temporary storage\n  buffer in bytes that is required by hipsparseXgtsv2_nopivot(). The temporary storage\n  buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSgtsv2_nopivot_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgtsv2_nopivot_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        B: *const f64,
        db: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgtsv2_nopivot_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const hipComplex,
        d: *const hipComplex,
        du: *const hipComplex,
        B: *const hipComplex,
        ldb: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgtsv2_nopivot_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const hipDoubleComplex,
        d: *const hipDoubleComplex,
        du: *const hipDoubleComplex,
        B: *const hipDoubleComplex,
        ldb: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Tridiagonal solver (no pivoting)\n\n  \\details\n  \\p hipsparseXgtsv2_nopivot solves a tridiagonal linear system for multiple right-hand sides\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSgtsv2_nopivot(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        B: *mut f32,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgtsv2_nopivot(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        B: *mut f64,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgtsv2_nopivot(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const hipComplex,
        d: *const hipComplex,
        du: *const hipComplex,
        B: *mut hipComplex,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgtsv2_nopivot(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const hipDoubleComplex,
        d: *const hipDoubleComplex,
        du: *const hipDoubleComplex,
        B: *mut hipDoubleComplex,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Strided Batch tridiagonal solver (no pivoting)\n\n  \\details\n  \\p hipsparseXgtsv2StridedBatch_bufferSizeExt returns the size of the temporary storage\n  buffer in bytes that is required by hipsparseXgtsv2StridedBatch(). The temporary\n  storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSgtsv2StridedBatch_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        x: *const f32,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgtsv2StridedBatch_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        x: *const f64,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgtsv2StridedBatch_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const hipComplex,
        d: *const hipComplex,
        du: *const hipComplex,
        x: *const hipComplex,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgtsv2StridedBatch_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const hipDoubleComplex,
        d: *const hipDoubleComplex,
        du: *const hipDoubleComplex,
        x: *const hipDoubleComplex,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Strided Batch tridiagonal solver (no pivoting)\n\n  \\details\n  \\p hipsparseXgtsv2StridedBatch solves a batched tridiagonal linear system\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSgtsv2StridedBatch(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        x: *mut f32,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgtsv2StridedBatch(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        x: *mut f64,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgtsv2StridedBatch(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const hipComplex,
        d: *const hipComplex,
        du: *const hipComplex,
        x: *mut hipComplex,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgtsv2StridedBatch(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const hipDoubleComplex,
        d: *const hipDoubleComplex,
        du: *const hipDoubleComplex,
        x: *mut hipDoubleComplex,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Interleaved Batch tridiagonal solver\n\n  \\details\n  \\p hipsparseXgtsvInterleavedBatch_bufferSizeExt returns the size of the temporary storage\n  buffer in bytes that is required by hipsparseXgtsvInterleavedBatch(). The temporary storage\n  buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSgtsvInterleavedBatch_bufferSizeExt(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        x: *const f32,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgtsvInterleavedBatch_bufferSizeExt(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        x: *const f64,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgtsvInterleavedBatch_bufferSizeExt(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *const hipComplex,
        d: *const hipComplex,
        du: *const hipComplex,
        x: *const hipComplex,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgtsvInterleavedBatch_bufferSizeExt(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *const hipDoubleComplex,
        d: *const hipDoubleComplex,
        du: *const hipDoubleComplex,
        x: *const hipDoubleComplex,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Interleaved Batch tridiagonal solver\n\n  \\details\n  \\p hipsparseXgtsvInterleavedBatch solves a batched tridiagonal linear system\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSgtsvInterleavedBatch(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *mut f32,
        d: *mut f32,
        du: *mut f32,
        x: *mut f32,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgtsvInterleavedBatch(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *mut f64,
        d: *mut f64,
        du: *mut f64,
        x: *mut f64,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgtsvInterleavedBatch(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *mut hipComplex,
        d: *mut hipComplex,
        du: *mut hipComplex,
        x: *mut hipComplex,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgtsvInterleavedBatch(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *mut hipDoubleComplex,
        d: *mut hipDoubleComplex,
        du: *mut hipDoubleComplex,
        x: *mut hipDoubleComplex,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Interleaved Batch pentadiagonal solver\n\n  \\details\n  \\p hipsparseXgpsvInterleavedBatch_bufferSizeExt returns the size of the temporary storage\n  buffer in bytes that is required by hipsparseXgpsvInterleavedBatch(). The temporary\n  storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSgpsvInterleavedBatch_bufferSizeExt(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *const f32,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        dw: *const f32,
        x: *const f32,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgpsvInterleavedBatch_bufferSizeExt(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *const f64,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        dw: *const f64,
        x: *const f64,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgpsvInterleavedBatch_bufferSizeExt(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *const hipComplex,
        dl: *const hipComplex,
        d: *const hipComplex,
        du: *const hipComplex,
        dw: *const hipComplex,
        x: *const hipComplex,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgpsvInterleavedBatch_bufferSizeExt(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *const hipDoubleComplex,
        dl: *const hipDoubleComplex,
        d: *const hipDoubleComplex,
        du: *const hipDoubleComplex,
        dw: *const hipDoubleComplex,
        x: *const hipDoubleComplex,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup precond_module\n  \\brief Interleaved Batch pentadiagonal solver\n\n  \\details\n  \\p hipsparseXgpsvInterleavedBatch solves a batched pentadiagonal linear system\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSgpsvInterleavedBatch(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *mut f32,
        dl: *mut f32,
        d: *mut f32,
        du: *mut f32,
        dw: *mut f32,
        x: *mut f32,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgpsvInterleavedBatch(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *mut f64,
        dl: *mut f64,
        d: *mut f64,
        du: *mut f64,
        dw: *mut f64,
        x: *mut f64,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgpsvInterleavedBatch(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *mut hipComplex,
        dl: *mut hipComplex,
        d: *mut hipComplex,
        du: *mut hipComplex,
        dw: *mut hipComplex,
        x: *mut hipComplex,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgpsvInterleavedBatch(
        handle: hipsparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *mut hipDoubleComplex,
        dl: *mut hipDoubleComplex,
        d: *mut hipDoubleComplex,
        du: *mut hipDoubleComplex,
        dw: *mut hipDoubleComplex,
        x: *mut hipDoubleComplex,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function computes the number of nonzero elements per row or column and the total\n  number of nonzero elements in a dense matrix.\n\n  \\details\n  The routine does support asynchronous execution if the pointer mode is set to device.\n/\n/**@{"]
    pub fn hipsparseSnnz(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        A: *const f32,
        lda: ::core::ffi::c_int,
        nnzPerRowColumn: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDnnz(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        A: *const f64,
        lda: ::core::ffi::c_int,
        nnzPerRowColumn: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCnnz(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        A: *const hipComplex,
        lda: ::core::ffi::c_int,
        nnzPerRowColumn: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZnnz(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        A: *const hipDoubleComplex,
        lda: ::core::ffi::c_int,
        nnzPerRowColumn: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function converts the matrix A in dense format into a sparse matrix in CSR format.\n  All the parameters are assumed to have been pre-allocated by the user and the arrays\n  are filled in based on nnz_per_row, which can be pre-computed with hipsparseXnnz().\n  It is executed asynchronously with respect to the host and may return control to the\n  application on the host before the entire result is ready.\n/\n/**@{"]
    pub fn hipsparseSdense2csr(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        A: *const f32,
        ld: ::core::ffi::c_int,
        nnz_per_rows: *const ::core::ffi::c_int,
        csr_val: *mut f32,
        csr_row_ptr: *mut ::core::ffi::c_int,
        csr_col_ind: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDdense2csr(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        A: *const f64,
        ld: ::core::ffi::c_int,
        nnz_per_rows: *const ::core::ffi::c_int,
        csr_val: *mut f64,
        csr_row_ptr: *mut ::core::ffi::c_int,
        csr_col_ind: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCdense2csr(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        A: *const hipComplex,
        ld: ::core::ffi::c_int,
        nnz_per_rows: *const ::core::ffi::c_int,
        csr_val: *mut hipComplex,
        csr_row_ptr: *mut ::core::ffi::c_int,
        csr_col_ind: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZdense2csr(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        A: *const hipDoubleComplex,
        ld: ::core::ffi::c_int,
        nnz_per_rows: *const ::core::ffi::c_int,
        csr_val: *mut hipDoubleComplex,
        csr_row_ptr: *mut ::core::ffi::c_int,
        csr_col_ind: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function computes the the size of the user allocated temporary storage buffer used when converting and pruning\n  a dense matrix to a CSR matrix.\n\n  \\details\n  \\p hipsparseXpruneDense2csr_bufferSizeExt returns the size of the temporary storage buffer\n  that is required by hipsparseXpruneDense2csrNnz() and hipsparseXpruneDense2csr(). The\n  temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSpruneDense2csr_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        threshold: *const f32,
        descr: hipsparseMatDescr_t,
        csrVal: *const f32,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *const ::core::ffi::c_int,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneDense2csr_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        threshold: *const f64,
        descr: hipsparseMatDescr_t,
        csrVal: *const f64,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *const ::core::ffi::c_int,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpruneDense2csr_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        threshold: *const f32,
        descr: hipsparseMatDescr_t,
        csrVal: *const f32,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *const ::core::ffi::c_int,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneDense2csr_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        threshold: *const f64,
        descr: hipsparseMatDescr_t,
        csrVal: *const f64,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *const ::core::ffi::c_int,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function computes the number of nonzero elements per row and the total number of\n  nonzero elements in a dense matrix once elements less than the threshold are pruned\n  from the matrix.\n\n  \\details\n  The routine does support asynchronous execution if the pointer mode is set to device.\n/\n/**@{"]
    pub fn hipsparseSpruneDense2csrNnz(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        threshold: *const f32,
        descr: hipsparseMatDescr_t,
        csrRowPtr: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneDense2csrNnz(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        threshold: *const f64,
        descr: hipsparseMatDescr_t,
        csrRowPtr: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function converts the matrix A in dense format into a sparse matrix in CSR format\n  while pruning values that are less than the threshold. All the parameters are assumed\n  to have been pre-allocated by the user.\n\n  \\details\n  The user first allocates \\p csrRowPtr to have \\p m+1 elements and then calls\n  hipsparseXpruneDense2csrNnz() which fills in the \\p csrRowPtr array and stores the\n  number of elements that are larger than the pruning threshold in \\p nnzTotalDevHostPtr.\n  The user then allocates \\p csrColInd and \\p csrVal to have size \\p nnzTotalDevHostPtr\n  and completes the conversion by calling hipsparseXpruneDense2csr(). A temporary storage\n  buffer is used by both hipsparseXpruneDense2csrNnz() and hipsparseXpruneDense2csr() and\n  must be allocated by the user and whose size is determined by\n  hipsparseXpruneDense2csr_bufferSizeExt(). The routine hipsparseXpruneDense2csr() is\n  executed asynchronously with respect to the host and may return control to the\n  application on the host before the entire result is ready.\n/\n/**@{"]
    pub fn hipsparseSpruneDense2csr(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        threshold: *const f32,
        descr: hipsparseMatDescr_t,
        csrVal: *mut f32,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneDense2csr(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        threshold: *const f64,
        descr: hipsparseMatDescr_t,
        csrVal: *mut f64,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function computes the size of the user allocated temporary storage buffer used\n  when converting and pruning by percentage a dense matrix to a CSR matrix.\n\n  \\details\n  When converting and pruning a dense matrix A to a CSR matrix by percentage the\n  following steps are performed. First the user calls\n  \\p hipsparseXpruneDense2csrByPercentage_bufferSize which determines the size of the\n  temporary storage buffer. Once determined, this buffer must be allocated by the user.\n  Next the user allocates the csr_row_ptr array to have \\p m+1 elements and calls\n  \\p hipsparseXpruneDense2csrNnzByPercentage. Finally the user finishes the conversion\n  by allocating the csr_col_ind and csr_val arrays (whos size is determined by the value\n  at nnz_total_dev_host_ptr) and calling \\p hipsparseXpruneDense2csrByPercentage.\n\n  The pruning by percentage works by first sorting the absolute values of the dense\n  matrix \\p A. We then determine a position in this sorted array by\n  \\f[\n    pos = ceil(m*n*(percentage/100)) - 1\n    pos = min(pos, m*n-1)\n    pos = max(pos, 0)\n    threshold = sorted_A[pos]\n  \\f]\n  Once we have this threshold we prune values in the dense matrix \\p A as in\n  \\p hipsparseXpruneDense2csr. It is executed asynchronously with respect to the host\n  and may return control to the application on the host before the entire result is\n  ready.\n/\n/**@{"]
    pub fn hipsparseSpruneDense2csrByPercentage_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        percentage: f32,
        descr: hipsparseMatDescr_t,
        csrVal: *const f32,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *const ::core::ffi::c_int,
        info: pruneInfo_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneDense2csrByPercentage_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        percentage: f64,
        descr: hipsparseMatDescr_t,
        csrVal: *const f64,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *const ::core::ffi::c_int,
        info: pruneInfo_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function computes the size of the user allocated temporary storage buffer used\n  when converting and pruning by percentage a dense matrix to a CSR matrix.\n\n  \\details\n  When converting and pruning a dense matrix A to a CSR matrix by percentage the\n  following steps are performed. First the user calls\n  \\p hipsparseXpruneDense2csrByPercentage_bufferSizeExt which determines the size of the\n  temporary storage buffer. Once determined, this buffer must be allocated by the user.\n  Next the user allocates the csr_row_ptr array to have \\p m+1 elements and calls\n  \\p hipsparseXpruneDense2csrNnzByPercentage. Finally the user finishes the conversion\n  by allocating the csr_col_ind and csr_val arrays (whos size is determined by the value\n  at nnz_total_dev_host_ptr) and calling \\p hipsparseXpruneDense2csrByPercentage.\n\n  The pruning by percentage works by first sorting the absolute values of the dense\n  matrix \\p A. We then determine a position in this sorted array by\n  \\f[\n    pos = ceil(m*n*(percentage/100)) - 1\n    pos = min(pos, m*n-1)\n    pos = max(pos, 0)\n    threshold = sorted_A[pos]\n  \\f]\n  Once we have this threshold we prune values in the dense matrix \\p A as in\n  \\p hipsparseXpruneDense2csr. It is executed asynchronously with respect to the host\n  and may return control to the application on the host before the entire result is\n  ready.\n/\n/**@{"]
    pub fn hipsparseSpruneDense2csrByPercentage_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        percentage: f32,
        descr: hipsparseMatDescr_t,
        csrVal: *const f32,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *const ::core::ffi::c_int,
        info: pruneInfo_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneDense2csrByPercentage_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        percentage: f64,
        descr: hipsparseMatDescr_t,
        csrVal: *const f64,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *const ::core::ffi::c_int,
        info: pruneInfo_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function computes the number of nonzero elements per row and the total number of\n  nonzero elements in a dense matrix when converting and pruning by percentage a dense\n  matrix to a CSR matrix.\n\n  \\details\n  When converting and pruning a dense matrix A to a CSR matrix by percentage the\n  following steps are performed. First the user calls\n  \\p hipsparseXpruneDense2csrByPercentage_bufferSize which determines the size of the\n  temporary storage buffer. Once determined, this buffer must be allocated by the user.\n  Next the user allocates the csr_row_ptr array to have \\p m+1 elements and calls\n  \\p hipsparseXpruneDense2csrNnzByPercentage. Finally the user finishes the conversion\n  by allocating the csr_col_ind and csr_val arrays (whos size is determined by the value\n  at nnz_total_dev_host_ptr) and calling \\p hipsparseXpruneDense2csrByPercentage.\n\n  The pruning by percentage works by first sorting the absolute values of the dense\n  matrix \\p A. We then determine a position in this sorted array by\n  \\f[\n    pos = ceil(m*n*(percentage/100)) - 1\n    pos = min(pos, m*n-1)\n    pos = max(pos, 0)\n    threshold = sorted_A[pos]\n  \\f]\n  Once we have this threshold we prune values in the dense matrix \\p A as in\n  \\p hipsparseXpruneDense2csr. The routine does support asynchronous execution if the\n  pointer mode is set to device.\n/\n/**@{"]
    pub fn hipsparseSpruneDense2csrNnzByPercentage(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        percentage: f32,
        descr: hipsparseMatDescr_t,
        csrRowPtr: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        info: pruneInfo_t,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneDense2csrNnzByPercentage(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        percentage: f64,
        descr: hipsparseMatDescr_t,
        csrRowPtr: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        info: pruneInfo_t,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function computes the number of nonzero elements per row and the total number of\n  nonzero elements in a dense matrix when converting and pruning by percentage a dense\n  matrix to a CSR matrix.\n\n  \\details\n  When converting and pruning a dense matrix A to a CSR matrix by percentage the\n  following steps are performed. First the user calls\n  \\p hipsparseXpruneDense2csrByPercentage_bufferSize which determines the size of the\n  temporary storage buffer. Once determined, this buffer must be allocated by the user.\n  Next the user allocates the csr_row_ptr array to have \\p m+1 elements and calls\n  \\p hipsparseXpruneDense2csrNnzByPercentage. Finally the user finishes the conversion\n  by allocating the csr_col_ind and csr_val arrays (whos size is determined by the value\n  at nnz_total_dev_host_ptr) and calling \\p hipsparseXpruneDense2csrByPercentage.\n\n  The pruning by percentage works by first sorting the absolute values of the dense\n  matrix \\p A. We then determine a position in this sorted array by\n  \\f[\n    pos = ceil(m*n*(percentage/100)) - 1\n    pos = min(pos, m*n-1)\n    pos = max(pos, 0)\n    threshold = sorted_A[pos]\n  \\f]\n  Once we have this threshold we prune values in the dense matrix \\p A as in\n  \\p hipsparseXpruneDense2csr. The routine does support asynchronous execution if the\n  pointer mode is set to device.\n/\n/**@{"]
    pub fn hipsparseSpruneDense2csrByPercentage(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        percentage: f32,
        descr: hipsparseMatDescr_t,
        csrVal: *mut f32,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: pruneInfo_t,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneDense2csrByPercentage(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        percentage: f64,
        descr: hipsparseMatDescr_t,
        csrVal: *mut f64,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: pruneInfo_t,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n\n  This function converts the matrix A in dense format into a sparse matrix in CSC format.\n  All the parameters are assumed to have been pre-allocated by the user and the arrays are filled in based on nnz_per_columns, which can be pre-computed with hipsparseXnnz().\n  It is executed asynchronously with respect to the host and may return control to the application on the host before the entire result is ready.\n/\n/**@{"]
    pub fn hipsparseSdense2csc(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        A: *const f32,
        ld: ::core::ffi::c_int,
        nnz_per_columns: *const ::core::ffi::c_int,
        csc_val: *mut f32,
        csc_row_ind: *mut ::core::ffi::c_int,
        csc_col_ptr: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDdense2csc(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        A: *const f64,
        ld: ::core::ffi::c_int,
        nnz_per_columns: *const ::core::ffi::c_int,
        csc_val: *mut f64,
        csc_row_ind: *mut ::core::ffi::c_int,
        csc_col_ptr: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCdense2csc(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        A: *const hipComplex,
        ld: ::core::ffi::c_int,
        nnz_per_columns: *const ::core::ffi::c_int,
        csc_val: *mut hipComplex,
        csc_row_ind: *mut ::core::ffi::c_int,
        csc_col_ptr: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZdense2csc(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        A: *const hipDoubleComplex,
        ld: ::core::ffi::c_int,
        nnz_per_columns: *const ::core::ffi::c_int,
        csc_val: *mut hipDoubleComplex,
        csc_row_ind: *mut ::core::ffi::c_int,
        csc_col_ptr: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function converts the sparse matrix in CSR format into a dense matrix.\n  It is executed asynchronously with respect to the host and may return control to the application on the host before the entire result is ready.\n/\n/**@{"]
    pub fn hipsparseScsr2dense(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        csr_val: *const f32,
        csr_row_ptr: *const ::core::ffi::c_int,
        csr_col_ind: *const ::core::ffi::c_int,
        A: *mut f32,
        ld: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsr2dense(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        csr_val: *const f64,
        csr_row_ptr: *const ::core::ffi::c_int,
        csr_col_ind: *const ::core::ffi::c_int,
        A: *mut f64,
        ld: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsr2dense(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        csr_val: *const hipComplex,
        csr_row_ptr: *const ::core::ffi::c_int,
        csr_col_ind: *const ::core::ffi::c_int,
        A: *mut hipComplex,
        ld: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsr2dense(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        csr_val: *const hipDoubleComplex,
        csr_row_ptr: *const ::core::ffi::c_int,
        csr_col_ind: *const ::core::ffi::c_int,
        A: *mut hipDoubleComplex,
        ld: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function converts the sparse matrix in CSC format into a dense matrix.\n  It is executed asynchronously with respect to the host and may return control to the application on the host before the entire result is ready.\n/\n/**@{"]
    pub fn hipsparseScsc2dense(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        csc_val: *const f32,
        csc_row_ind: *const ::core::ffi::c_int,
        csc_col_ptr: *const ::core::ffi::c_int,
        A: *mut f32,
        ld: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsc2dense(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        csc_val: *const f64,
        csc_row_ind: *const ::core::ffi::c_int,
        csc_col_ptr: *const ::core::ffi::c_int,
        A: *mut f64,
        ld: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsc2dense(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        csc_val: *const hipComplex,
        csc_row_ind: *const ::core::ffi::c_int,
        csc_col_ptr: *const ::core::ffi::c_int,
        A: *mut hipComplex,
        ld: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsc2dense(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descr: hipsparseMatDescr_t,
        csc_val: *const hipDoubleComplex,
        csc_row_ind: *const ::core::ffi::c_int,
        csc_col_ptr: *const ::core::ffi::c_int,
        A: *mut hipDoubleComplex,
        ld: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief
  This function computes the number of nonzero block columns per row and the total number of nonzero blocks in a sparse
  BSR matrix given a sparse CSR matrix as input.

  \details
  The routine does support asynchronous execution if the pointer mode is set to device.*/
    pub fn hipsparseXcsr2bsrNnz(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        bsrRowPtrC: *mut ::core::ffi::c_int,
        bsrNnzb: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  Given a sparse CSR matrix and a non-negative tolerance, this function computes how many entries would be left\n  in each row of the matrix if elements less than the tolerance were removed. It also computes the total number\n  of remaining elements in the matrix.\n/\n/**@{"]
    pub fn hipsparseSnnz_compress(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        nnzPerRow: *mut ::core::ffi::c_int,
        nnzC: *mut ::core::ffi::c_int,
        tol: f32,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDnnz_compress(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        nnzPerRow: *mut ::core::ffi::c_int,
        nnzC: *mut ::core::ffi::c_int,
        tol: f64,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCnnz_compress(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const hipComplex,
        csrRowPtrA: *const ::core::ffi::c_int,
        nnzPerRow: *mut ::core::ffi::c_int,
        nnzC: *mut ::core::ffi::c_int,
        tol: hipComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZnnz_compress(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const hipDoubleComplex,
        csrRowPtrA: *const ::core::ffi::c_int,
        nnzPerRow: *mut ::core::ffi::c_int,
        nnzC: *mut ::core::ffi::c_int,
        tol: hipDoubleComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief Convert a sparse CSR matrix into a sparse COO matrix

  \details
  \p hipsparseXcsr2coo converts the CSR array containing the row offsets, that point
  to the start of every row, into a COO array of row indices.

  \note
  It can also be used to convert a CSC array containing the column offsets into a COO
  array of column indices.

  \note
  This function is non blocking and executed asynchronously with respect to the host.
  It may return before the actual computation has finished.*/
    pub fn hipsparseXcsr2coo(
        handle: hipsparseHandle_t,
        csrRowPtr: *const ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        cooRowInd: *mut ::core::ffi::c_int,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert a sparse CSR matrix into a sparse CSC matrix\n\n  \\details\n  \\p hipsparseXcsr2csc converts a CSR matrix into a CSC matrix. \\p hipsparseXcsr2csc\n  can also be used to convert a CSC matrix into a CSR matrix. \\p copy_values decides\n  whether \\p csc_val is being filled during conversion (\\ref HIPSPARSE_ACTION_NUMERIC)\n  or not (\\ref HIPSPARSE_ACTION_SYMBOLIC).\n\n  \\note\n  The resulting matrix can also be seen as the transpose of the input matrix.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseScsr2csc(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrSortedVal: *const f32,
        csrSortedRowPtr: *const ::core::ffi::c_int,
        csrSortedColInd: *const ::core::ffi::c_int,
        cscSortedVal: *mut f32,
        cscSortedRowInd: *mut ::core::ffi::c_int,
        cscSortedColPtr: *mut ::core::ffi::c_int,
        copyValues: hipsparseAction_t,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsr2csc(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrSortedVal: *const f64,
        csrSortedRowPtr: *const ::core::ffi::c_int,
        csrSortedColInd: *const ::core::ffi::c_int,
        cscSortedVal: *mut f64,
        cscSortedRowInd: *mut ::core::ffi::c_int,
        cscSortedColPtr: *mut ::core::ffi::c_int,
        copyValues: hipsparseAction_t,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsr2csc(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrSortedVal: *const hipComplex,
        csrSortedRowPtr: *const ::core::ffi::c_int,
        csrSortedColInd: *const ::core::ffi::c_int,
        cscSortedVal: *mut hipComplex,
        cscSortedRowInd: *mut ::core::ffi::c_int,
        cscSortedColPtr: *mut ::core::ffi::c_int,
        copyValues: hipsparseAction_t,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsr2csc(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrSortedVal: *const hipDoubleComplex,
        csrSortedRowPtr: *const ::core::ffi::c_int,
        csrSortedColInd: *const ::core::ffi::c_int,
        cscSortedVal: *mut hipDoubleComplex,
        cscSortedRowInd: *mut ::core::ffi::c_int,
        cscSortedColPtr: *mut ::core::ffi::c_int,
        copyValues: hipsparseAction_t,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
impl hipsparseCsr2CscAlg_t {
    pub const HIPSPARSE_CSR2CSC_ALG_DEFAULT: hipsparseCsr2CscAlg_t = hipsparseCsr2CscAlg_t(
        0,
    );
}
impl hipsparseCsr2CscAlg_t {
    pub const HIPSPARSE_CSR2CSC_ALG1: hipsparseCsr2CscAlg_t = hipsparseCsr2CscAlg_t(1);
}
impl hipsparseCsr2CscAlg_t {
    pub const HIPSPARSE_CSR2CSC_ALG2: hipsparseCsr2CscAlg_t = hipsparseCsr2CscAlg_t(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseCsr2CscAlg_t(pub ::core::ffi::c_uint);
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief This function computes the size of the user allocated temporary storage buffer used
  when converting a sparse CSR matrix into a sparse CSC matrix.

  \details
  \p hipsparseCsr2cscEx2_bufferSize calculates the required user allocated temporary buffer needed
  by \p hipsparseCsr2cscEx2 to convert a CSR matrix into a CSC matrix. \p hipsparseCsr2cscEx2
  can also be used to convert a CSC matrix into a CSR matrix. \p copy_values decides
  whether \p csc_val is being filled during conversion (\ref HIPSPARSE_ACTION_NUMERIC)
  or not (\ref HIPSPARSE_ACTION_SYMBOLIC).

  \note
  The resulting matrix can also be seen as the transpose of the input matrix.

  \note
  This function is non blocking and executed asynchronously with respect to the host.
  It may return before the actual computation has finished.*/
    pub fn hipsparseCsr2cscEx2_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrVal: *const ::core::ffi::c_void,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *const ::core::ffi::c_int,
        cscVal: *mut ::core::ffi::c_void,
        cscColPtr: *mut ::core::ffi::c_int,
        cscRowInd: *mut ::core::ffi::c_int,
        valType: hipDataType,
        copyValues: hipsparseAction_t,
        idxBase: hipsparseIndexBase_t,
        alg: hipsparseCsr2CscAlg_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief Convert a sparse CSR matrix into a sparse CSC matrix

  \details
  \p hipsparseCsr2cscEx2 converts a CSR matrix into a CSC matrix. \p hipsparseCsr2cscEx2
  can also be used to convert a CSC matrix into a CSR matrix. \p copy_values decides
  whether \p csc_val is being filled during conversion (\ref HIPSPARSE_ACTION_NUMERIC)
  or not (\ref HIPSPARSE_ACTION_SYMBOLIC).

  \note
  The resulting matrix can also be seen as the transpose of the input matrix.

  \note
  This function is non blocking and executed asynchronously with respect to the host.
  It may return before the actual computation has finished.*/
    pub fn hipsparseCsr2cscEx2(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrVal: *const ::core::ffi::c_void,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *const ::core::ffi::c_int,
        cscVal: *mut ::core::ffi::c_void,
        cscColPtr: *mut ::core::ffi::c_int,
        cscRowInd: *mut ::core::ffi::c_int,
        valType: hipDataType,
        copyValues: hipsparseAction_t,
        idxBase: hipsparseIndexBase_t,
        alg: hipsparseCsr2CscAlg_t,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert a sparse CSR matrix into a sparse HYB matrix\n\n  \\details\n  \\p hipsparseXcsr2hyb converts a CSR matrix into a HYB matrix. It is assumed\n  that \\p hyb has been initialized with hipsparseCreateHybMat().\n\n  \\note\n  This function requires a significant amount of storage for the HYB matrix,\n  depending on the matrix structure.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseScsr2hyb(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        hybA: hipsparseHybMat_t,
        userEllWidth: ::core::ffi::c_int,
        partitionType: hipsparseHybPartition_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsr2hyb(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        hybA: hipsparseHybMat_t,
        userEllWidth: ::core::ffi::c_int,
        partitionType: hipsparseHybPartition_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsr2hyb(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        hybA: hipsparseHybMat_t,
        userEllWidth: ::core::ffi::c_int,
        partitionType: hipsparseHybPartition_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsr2hyb(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrSortedValA: *const hipDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        hybA: hipsparseHybMat_t,
        userEllWidth: ::core::ffi::c_int,
        partitionType: hipsparseHybPartition_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert a sparse GEneral BSR matrix into a sparse GEneral BSC matrix\n\n  \\details\n  \\p hipsparseXgebsr2gebsc_bufferSize returns the size of the temporary storage buffer\n  required by hipsparseXgebsr2gebsc().\n  The temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSgebsr2gebsc_bufferSize(
        handle: hipsparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsr_val: *const f32,
        bsr_row_ptr: *const ::core::ffi::c_int,
        bsr_col_ind: *const ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        p_buffer_size: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgebsr2gebsc_bufferSize(
        handle: hipsparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsr_val: *const f64,
        bsr_row_ptr: *const ::core::ffi::c_int,
        bsr_col_ind: *const ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        p_buffer_size: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgebsr2gebsc_bufferSize(
        handle: hipsparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsr_val: *const hipComplex,
        bsr_row_ptr: *const ::core::ffi::c_int,
        bsr_col_ind: *const ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        p_buffer_size: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgebsr2gebsc_bufferSize(
        handle: hipsparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsr_val: *const hipDoubleComplex,
        bsr_row_ptr: *const ::core::ffi::c_int,
        bsr_col_ind: *const ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        p_buffer_size: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert a sparse GEneral BSR matrix into a sparse GEneral BSC matrix\n\n  \\details\n  \\p hipsparseXgebsr2gebsc converts a GEneral BSR matrix into a GEneral BSC matrix. \\p hipsparseXgebsr2gebsc\n  can also be used to convert a GEneral BSC matrix into a GEneral BSR matrix. \\p copy_values decides\n  whether \\p bsc_val is being filled during conversion (\\ref HIPSPARSE_ACTION_NUMERIC)\n  or not (\\ref HIPSPARSE_ACTION_SYMBOLIC).\n\n  \\p hipsparseXgebsr2gebsc requires extra temporary storage buffer that has to be allocated\n  by the user. Storage buffer size can be determined by hipsparseXgebsr2gebsc_bufferSize().\n\n  \\note\n  The resulting matrix can also be seen as the transpose of the input matrix.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSgebsr2gebsc(
        handle: hipsparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsr_val: *const f32,
        bsr_row_ptr: *const ::core::ffi::c_int,
        bsr_col_ind: *const ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        bsc_val: *mut f32,
        bsc_row_ind: *mut ::core::ffi::c_int,
        bsc_col_ptr: *mut ::core::ffi::c_int,
        copy_values: hipsparseAction_t,
        idx_base: hipsparseIndexBase_t,
        temp_buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgebsr2gebsc(
        handle: hipsparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsr_val: *const f64,
        bsr_row_ptr: *const ::core::ffi::c_int,
        bsr_col_ind: *const ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        bsc_val: *mut f64,
        bsc_row_ind: *mut ::core::ffi::c_int,
        bsc_col_ptr: *mut ::core::ffi::c_int,
        copy_values: hipsparseAction_t,
        idx_base: hipsparseIndexBase_t,
        temp_buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgebsr2gebsc(
        handle: hipsparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsr_val: *const hipComplex,
        bsr_row_ptr: *const ::core::ffi::c_int,
        bsr_col_ind: *const ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        bsc_val: *mut hipComplex,
        bsc_row_ind: *mut ::core::ffi::c_int,
        bsc_col_ptr: *mut ::core::ffi::c_int,
        copy_values: hipsparseAction_t,
        idx_base: hipsparseIndexBase_t,
        temp_buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgebsr2gebsc(
        handle: hipsparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsr_val: *const hipDoubleComplex,
        bsr_row_ptr: *const ::core::ffi::c_int,
        bsr_col_ind: *const ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        bsc_val: *mut hipDoubleComplex,
        bsc_row_ind: *mut ::core::ffi::c_int,
        bsc_col_ptr: *mut ::core::ffi::c_int,
        copy_values: hipsparseAction_t,
        idx_base: hipsparseIndexBase_t,
        temp_buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  \\details\n  \\p hipsparseXcsr2gebsr_bufferSize returns the size of the temporary buffer that\n  is required by \\p hipsparseXcsr2gebcsrNnz and \\p hipsparseXcsr2gebcsr.\n  The temporary storage buffer must be allocated by the user.\n\n  This function computes the number of nonzero block columns per row and the total number of nonzero blocks in a sparse\n  GEneral BSR matrix given a sparse CSR matrix as input.\n\n  \\details\n  The routine does support asynchronous execution if the pointer mode is set to device.\n/\n/**@{"]
    pub fn hipsparseScsr2gebsr_bufferSize(
        handle: hipsparseHandle_t,
        dir: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        csr_descr: hipsparseMatDescr_t,
        csr_val: *const f32,
        csr_row_ptr: *const ::core::ffi::c_int,
        csr_col_ind: *const ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        p_buffer_size: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsr2gebsr_bufferSize(
        handle: hipsparseHandle_t,
        dir: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        csr_descr: hipsparseMatDescr_t,
        csr_val: *const f64,
        csr_row_ptr: *const ::core::ffi::c_int,
        csr_col_ind: *const ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        p_buffer_size: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsr2gebsr_bufferSize(
        handle: hipsparseHandle_t,
        dir: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        csr_descr: hipsparseMatDescr_t,
        csr_val: *const hipComplex,
        csr_row_ptr: *const ::core::ffi::c_int,
        csr_col_ind: *const ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        p_buffer_size: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsr2gebsr_bufferSize(
        handle: hipsparseHandle_t,
        dir: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        csr_descr: hipsparseMatDescr_t,
        csr_val: *const hipDoubleComplex,
        csr_row_ptr: *const ::core::ffi::c_int,
        csr_col_ind: *const ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        p_buffer_size: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief
  This function computes the number of nonzero block columns per row and the total number of nonzero blocks in a sparse
  GEneral BSR matrix given a sparse CSR matrix as input.*/
    pub fn hipsparseXcsr2gebsrNnz(
        handle: hipsparseHandle_t,
        dir: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        csr_descr: hipsparseMatDescr_t,
        csr_row_ptr: *const ::core::ffi::c_int,
        csr_col_ind: *const ::core::ffi::c_int,
        bsr_descr: hipsparseMatDescr_t,
        bsr_row_ptr: *mut ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        bsr_nnz_devhost: *mut ::core::ffi::c_int,
        p_buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert a sparse CSR matrix into a sparse GEneral BSR matrix\n\n  \\details\n  \\p hipsparseXcsr2gebsr converts a CSR matrix into a GEneral BSR matrix. It is assumed,\n  that \\p bsr_val, \\p bsr_col_ind and \\p bsr_row_ptr are allocated. Allocation size\n  for \\p bsr_row_ptr is computed as \\p mb+1 where \\p mb is the number of block rows in\n  the GEneral BSR matrix. Allocation size for \\p bsr_val and \\p bsr_col_ind is computed using\n  \\p csr2gebsr_nnz() which also fills in \\p bsr_row_ptr.\n/\n/**@{"]
    pub fn hipsparseScsr2gebsr(
        handle: hipsparseHandle_t,
        dir: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        csr_descr: hipsparseMatDescr_t,
        csr_val: *const f32,
        csr_row_ptr: *const ::core::ffi::c_int,
        csr_col_ind: *const ::core::ffi::c_int,
        bsr_descr: hipsparseMatDescr_t,
        bsr_val: *mut f32,
        bsr_row_ptr: *mut ::core::ffi::c_int,
        bsr_col_ind: *mut ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        p_buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsr2gebsr(
        handle: hipsparseHandle_t,
        dir: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        csr_descr: hipsparseMatDescr_t,
        csr_val: *const f64,
        csr_row_ptr: *const ::core::ffi::c_int,
        csr_col_ind: *const ::core::ffi::c_int,
        bsr_descr: hipsparseMatDescr_t,
        bsr_val: *mut f64,
        bsr_row_ptr: *mut ::core::ffi::c_int,
        bsr_col_ind: *mut ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        p_buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsr2gebsr(
        handle: hipsparseHandle_t,
        dir: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        csr_descr: hipsparseMatDescr_t,
        csr_val: *const hipComplex,
        csr_row_ptr: *const ::core::ffi::c_int,
        csr_col_ind: *const ::core::ffi::c_int,
        bsr_descr: hipsparseMatDescr_t,
        bsr_val: *mut hipComplex,
        bsr_row_ptr: *mut ::core::ffi::c_int,
        bsr_col_ind: *mut ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        p_buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsr2gebsr(
        handle: hipsparseHandle_t,
        dir: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        csr_descr: hipsparseMatDescr_t,
        csr_val: *const hipDoubleComplex,
        csr_row_ptr: *const ::core::ffi::c_int,
        csr_col_ind: *const ::core::ffi::c_int,
        bsr_descr: hipsparseMatDescr_t,
        bsr_val: *mut hipDoubleComplex,
        bsr_row_ptr: *mut ::core::ffi::c_int,
        bsr_col_ind: *mut ::core::ffi::c_int,
        row_block_dim: ::core::ffi::c_int,
        col_block_dim: ::core::ffi::c_int,
        p_buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert a sparse CSR matrix into a sparse BSR matrix\n\n  \\details\n  \\p hipsparseXcsr2bsr converts a CSR matrix into a BSR matrix. It is assumed,\n  that \\p bsr_val, \\p bsr_col_ind and \\p bsr_row_ptr are allocated. Allocation size\n  for \\p bsr_row_ptr is computed as \\p mb+1 where \\p mb is the number of block rows in\n  the BSR matrix. Allocation size for \\p bsr_val and \\p bsr_col_ind is computed using\n  \\p csr2bsr_nnz() which also fills in \\p bsr_row_ptr.\n\n  \\p hipsparseXcsr2bsr requires extra temporary storage that is allocated internally if\n  \\p block_dim>16\n/\n/**@{"]
    pub fn hipsparseScsr2bsr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        bsrValC: *mut f32,
        bsrRowPtrC: *mut ::core::ffi::c_int,
        bsrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsr2bsr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        bsrValC: *mut f64,
        bsrRowPtrC: *mut ::core::ffi::c_int,
        bsrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsr2bsr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const hipComplex,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        bsrValC: *mut hipComplex,
        bsrRowPtrC: *mut ::core::ffi::c_int,
        bsrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsr2bsr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const hipDoubleComplex,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        bsrValC: *mut hipDoubleComplex,
        bsrRowPtrC: *mut ::core::ffi::c_int,
        bsrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert a sparse BSR matrix into a sparse CSR matrix\n\n  \\details\n  \\p hipsparseXbsr2csr converts a BSR matrix into a CSR matrix. It is assumed,\n  that \\p csr_val, \\p csr_col_ind and \\p csr_row_ptr are allocated. Allocation size\n  for \\p csr_row_ptr is computed by the number of block rows multiplied by the block\n  dimension plus one. Allocation for \\p csr_val and \\p csr_col_ind is computed by the\n  the number of blocks in the BSR matrix multiplied by the block dimension squared.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSbsr2csr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const f32,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f32,
        csrRowPtrC: *mut ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDbsr2csr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const f64,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f64,
        csrRowPtrC: *mut ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCbsr2csr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const hipComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut hipComplex,
        csrRowPtrC: *mut ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZbsr2csr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const hipDoubleComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut hipDoubleComplex,
        csrRowPtrC: *mut ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert a sparse general BSR matrix into a sparse CSR matrix\n\n  \\details\n  \\p hipsparseXgebsr2csr converts a BSR matrix into a CSR matrix. It is assumed,\n  that \\p csr_val, \\p csr_col_ind and \\p csr_row_ptr are allocated. Allocation size\n  for \\p csr_row_ptr is computed by the number of block rows multiplied by the block\n  dimension plus one. Allocation for \\p csr_val and \\p csr_col_ind is computed by the\n  the number of blocks in the BSR matrix multiplied by the product of the block dimensions.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseSgebsr2csr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const f32,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f32,
        csrRowPtrC: *mut ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgebsr2csr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const f64,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f64,
        csrRowPtrC: *mut ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgebsr2csr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const hipComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut hipComplex,
        csrRowPtrC: *mut ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgebsr2csr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const hipDoubleComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut hipDoubleComplex,
        csrRowPtrC: *mut ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert a sparse CSR matrix into a compressed sparse CSR matrix\n\n  \\details\n  \\p hipsparseXcsr2csr_compress converts a CSR matrix into a compressed CSR matrix by\n  removing entries in the input CSR matrix that are below a non-negative threshold \\p tol\n\n  \\note\n  In the case of complex matrices only the magnitude of the real part of \\p tol is used.\n/\n/**@{"]
    pub fn hipsparseScsr2csr_compress(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f32,
        csrColIndA: *const ::core::ffi::c_int,
        csrRowPtrA: *const ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        nnzPerRow: *const ::core::ffi::c_int,
        csrValC: *mut f32,
        csrColIndC: *mut ::core::ffi::c_int,
        csrRowPtrC: *mut ::core::ffi::c_int,
        tol: f32,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsr2csr_compress(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f64,
        csrColIndA: *const ::core::ffi::c_int,
        csrRowPtrA: *const ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        nnzPerRow: *const ::core::ffi::c_int,
        csrValC: *mut f64,
        csrColIndC: *mut ::core::ffi::c_int,
        csrRowPtrC: *mut ::core::ffi::c_int,
        tol: f64,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsr2csr_compress(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const hipComplex,
        csrColIndA: *const ::core::ffi::c_int,
        csrRowPtrA: *const ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        nnzPerRow: *const ::core::ffi::c_int,
        csrValC: *mut hipComplex,
        csrColIndC: *mut ::core::ffi::c_int,
        csrRowPtrC: *mut ::core::ffi::c_int,
        tol: hipComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsr2csr_compress(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const hipDoubleComplex,
        csrColIndA: *const ::core::ffi::c_int,
        csrRowPtrA: *const ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        nnzPerRow: *const ::core::ffi::c_int,
        csrValC: *mut hipDoubleComplex,
        csrColIndC: *mut ::core::ffi::c_int,
        csrRowPtrC: *mut ::core::ffi::c_int,
        tol: hipDoubleComplex,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert and prune sparse CSR matrix into a sparse CSR matrix\n\n  \\details\n  \\p hipsparseXpruneCsr2csr_bufferSize returns the size of the temporary buffer that\n  is required by \\p hipsparseXpruneCsr2csrNnz and hipsparseXpruneCsr2csr. The\n  temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSpruneCsr2csr_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        threshold: *const f32,
        descrC: hipsparseMatDescr_t,
        csrValC: *const f32,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *const ::core::ffi::c_int,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneCsr2csr_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        threshold: *const f64,
        descrC: hipsparseMatDescr_t,
        csrValC: *const f64,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *const ::core::ffi::c_int,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert and prune sparse CSR matrix into a sparse CSR matrix\n\n  \\details\n  \\p hipsparseXpruneCsr2csr_bufferSizeExt returns the size of the temporary buffer that\n  is required by \\p hipsparseXpruneCsr2csrNnz and hipsparseXpruneCsr2csr. The\n  temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSpruneCsr2csr_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        threshold: *const f32,
        descrC: hipsparseMatDescr_t,
        csrValC: *const f32,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *const ::core::ffi::c_int,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneCsr2csr_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        threshold: *const f64,
        descrC: hipsparseMatDescr_t,
        csrValC: *const f64,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *const ::core::ffi::c_int,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert and prune sparse CSR matrix into a sparse CSR matrix\n\n  \\details\n  \\p hipsparseXpruneCsr2csrNnz computes the number of nonzero elements per row and the total\n  number of nonzero elements in a sparse CSR matrix once elements less than the threshold are\n  pruned from the matrix.\n\n  \\note The routine does support asynchronous execution if the pointer mode is set to device.\n/\n/**@{"]
    pub fn hipsparseSpruneCsr2csrNnz(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        threshold: *const f32,
        descrC: hipsparseMatDescr_t,
        csrRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneCsr2csrNnz(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        threshold: *const f64,
        descrC: hipsparseMatDescr_t,
        csrRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert and prune sparse CSR matrix into a sparse CSR matrix\n\n  \\details\n  This function converts the sparse CSR matrix A into a sparse CSR matrix C by pruning values in A\n  that are less than the threshold. All the parameters are assumed to have been pre-allocated by the user.\n  The user first calls hipsparseXpruneCsr2csr_bufferSize() to determine the size of the buffer used\n  by hipsparseXpruneCsr2csrNnz() and hipsparseXpruneCsr2csr() which the user then allocates. The user then\n  allocates \\p csr_row_ptr_C to have \\p m+1 elements and then calls hipsparseXpruneCsr2csrNnz() which fills\n  in the \\p csr_row_ptr_C array stores then number of elements that are larger than the pruning threshold\n  in \\p nnz_total_dev_host_ptr. The user then calls hipsparseXpruneCsr2csr() to complete the conversion. It\n  is executed asynchronously with respect to the host and may return control to the application on the host\n  before the entire result is ready.\n/\n/**@{"]
    pub fn hipsparseSpruneCsr2csr(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        threshold: *const f32,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f32,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneCsr2csr(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        threshold: *const f64,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f64,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert and prune by percentage a sparse CSR matrix into a sparse CSR matrix\n\n  \\details\n  \\p hipsparseXpruneCsr2csrByPercentage_bufferSize returns the size of the temporary buffer that\n  is required by \\p hipsparseXpruneCsr2csrNnzByPercentage.\n  The temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSpruneCsr2csrByPercentage_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        percentage: f32,
        descrC: hipsparseMatDescr_t,
        csrValC: *const f32,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *const ::core::ffi::c_int,
        info: pruneInfo_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneCsr2csrByPercentage_bufferSize(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        percentage: f64,
        descrC: hipsparseMatDescr_t,
        csrValC: *const f64,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *const ::core::ffi::c_int,
        info: pruneInfo_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert and prune by percentage a sparse CSR matrix into a sparse CSR matrix\n\n  \\details\n  \\p hipsparseXpruneCsr2csrByPercentage_bufferSizeExt returns the size of the temporary buffer that\n  is required by \\p hipsparseXpruneCsr2csrNnzByPercentage.\n  The temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSpruneCsr2csrByPercentage_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        percentage: f32,
        descrC: hipsparseMatDescr_t,
        csrValC: *const f32,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *const ::core::ffi::c_int,
        info: pruneInfo_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneCsr2csrByPercentage_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        percentage: f64,
        descrC: hipsparseMatDescr_t,
        csrValC: *const f64,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *const ::core::ffi::c_int,
        info: pruneInfo_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert and prune by percentage a sparse CSR matrix into a sparse CSR matrix\n\n  \\details\n  \\p hipsparseXpruneCsr2csrNnzByPercentage computes the number of nonzero elements per row and the total\n  number of nonzero elements in a sparse CSR matrix once elements less than the threshold are\n  pruned from the matrix.\n\n  \\note The routine does support asynchronous execution if the pointer mode is set to device.\n/\n/**@{"]
    pub fn hipsparseSpruneCsr2csrNnzByPercentage(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        percentage: f32,
        descrC: hipsparseMatDescr_t,
        csrRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        info: pruneInfo_t,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneCsr2csrNnzByPercentage(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        percentage: f64,
        descrC: hipsparseMatDescr_t,
        csrRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        info: pruneInfo_t,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert and prune by percentage a sparse CSR matrix into a sparse CSR matrix\n\n  \\details\n  This function converts the sparse CSR matrix A into a sparse CSR matrix C by pruning values in A\n  that are less than the threshold. All the parameters are assumed to have been pre-allocated by the user.\n  The user first calls hipsparseXpruneCsr2csr_bufferSize() to determine the size of the buffer used\n  by hipsparseXpruneCsr2csrNnz() and hipsparseXpruneCsr2csr() which the user then allocates. The user then\n  allocates \\p csr_row_ptr_C to have \\p m+1 elements and then calls hipsparseXpruneCsr2csrNnz() which fills\n  in the \\p csr_row_ptr_C array stores then number of elements that are larger than the pruning threshold\n  in \\p nnz_total_dev_host_ptr. The user then calls hipsparseXpruneCsr2csr() to complete the conversion. It\n  is executed asynchronously with respect to the host and may return control to the application on the host\n  before the entire result is ready.\n/\n/**@{"]
    pub fn hipsparseSpruneCsr2csrByPercentage(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        percentage: f32,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f32,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
        info: pruneInfo_t,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDpruneCsr2csrByPercentage(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        percentage: f64,
        descrC: hipsparseMatDescr_t,
        csrValC: *mut f64,
        csrRowPtrC: *const ::core::ffi::c_int,
        csrColIndC: *mut ::core::ffi::c_int,
        info: pruneInfo_t,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief Convert a sparse HYB matrix into a sparse CSR matrix\n\n  \\details\n  \\p hipsparseXhyb2csr converts a HYB matrix into a CSR matrix.\n\n  \\note\n  This function is non blocking and executed asynchronously with respect to the host.\n  It may return before the actual computation has finished.\n/\n/**@{"]
    pub fn hipsparseShyb2csr(
        handle: hipsparseHandle_t,
        descrA: hipsparseMatDescr_t,
        hybA: hipsparseHybMat_t,
        csrSortedValA: *mut f32,
        csrSortedRowPtrA: *mut ::core::ffi::c_int,
        csrSortedColIndA: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDhyb2csr(
        handle: hipsparseHandle_t,
        descrA: hipsparseMatDescr_t,
        hybA: hipsparseHybMat_t,
        csrSortedValA: *mut f64,
        csrSortedRowPtrA: *mut ::core::ffi::c_int,
        csrSortedColIndA: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseChyb2csr(
        handle: hipsparseHandle_t,
        descrA: hipsparseMatDescr_t,
        hybA: hipsparseHybMat_t,
        csrSortedValA: *mut hipComplex,
        csrSortedRowPtrA: *mut ::core::ffi::c_int,
        csrSortedColIndA: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZhyb2csr(
        handle: hipsparseHandle_t,
        descrA: hipsparseMatDescr_t,
        hybA: hipsparseHybMat_t,
        csrSortedValA: *mut hipDoubleComplex,
        csrSortedRowPtrA: *mut ::core::ffi::c_int,
        csrSortedColIndA: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief Convert a sparse COO matrix into a sparse CSR matrix

  \details
  \p hipsparseXcoo2csr converts the COO array containing the row indices into a
  CSR array of row offsets, that point to the start of every row.
  It is assumed that the COO row index array is sorted.

  \note It can also be used, to convert a COO array containing the column indices into
  a CSC array of column offsets, that point to the start of every column. Then, it is
  assumed that the COO column index array is sorted, instead.

  \note
  This function is non blocking and executed asynchronously with respect to the host.
  It may return before the actual computation has finished.*/
    pub fn hipsparseXcoo2csr(
        handle: hipsparseHandle_t,
        cooRowInd: *const ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        csrRowPtr: *mut ::core::ffi::c_int,
        idxBase: hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief Create the identity map

  \details
  \p hipsparseCreateIdentityPermutation stores the identity map in \p p, such that
  \f$p = 0:1:(n-1)\f$.

  \code{.c}
      for(i = 0; i < n; ++i)
      {
          p[i] = i;
      }
  \endcode

  \note
  This function is non blocking and executed asynchronously with respect to the host.
  It may return before the actual computation has finished.*/
    pub fn hipsparseCreateIdentityPermutation(
        handle: hipsparseHandle_t,
        n: ::core::ffi::c_int,
        p: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief Sort a sparse CSR matrix

  \details
  \p hipsparseXcsrsort_bufferSizeExt returns the size of the temporary storage buffer
  in bytes required by hipsparseXcsrsort(). The temporary storage buffer must be allocated by
  the user.*/
    pub fn hipsparseXcsrsort_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief Sort a sparse CSR matrix

  \details
  \p hipsparseXcsrsort sorts a matrix in CSR format. The sorted permutation vector
  \p perm can be used to obtain sorted \p csr_val array. In this case, \p perm must be
  initialized as the identity permutation, see hipsparseCreateIdentityPermutation().

  \p hipsparseXcsrsort requires extra temporary storage buffer that has to be allocated by
  the user. Storage buffer size can be determined by hipsparseXcsrsort_bufferSizeExt().

  \note
  \p perm can be \p NULL if a sorted permutation vector is not required.

  \note
  This function is non blocking and executed asynchronously with respect to the host.
  It may return before the actual computation has finished.*/
    pub fn hipsparseXcsrsort(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief Sort a sparse CSC matrix

  \details
  \p hipsparseXcscsort_bufferSizeExt returns the size of the temporary storage buffer
  in bytes required by hipsparseXcscsort(). The temporary storage buffer must be
  allocated by the user.*/
    pub fn hipsparseXcscsort_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        cscColPtr: *const ::core::ffi::c_int,
        cscRowInd: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief Sort a sparse CSC matrix

  \details
  \p hipsparseXcscsort sorts a matrix in CSC format. The sorted permutation vector
  \p perm can be used to obtain sorted \p csc_val array. In this case, \p perm must be
  initialized as the identity permutation, see hipsparseCreateIdentityPermutation().

  \p hipsparseXcscsort requires extra temporary storage buffer that has to be allocated by
  the user. Storage buffer size can be determined by hipsparseXcscsort_bufferSizeExt().

  \note
  \p perm can be \p NULL if a sorted permutation vector is not required.

  \note
  This function is non blocking and executed asynchronously with respect to the host.
  It may return before the actual computation has finished.*/
    pub fn hipsparseXcscsort(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        cscColPtr: *const ::core::ffi::c_int,
        cscRowInd: *mut ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief Sort a sparse COO matrix

  \details
  \p hipsparseXcoosort_bufferSizeExt returns the size of the temporary storage buffer
  in bytes required by hipsparseXcoosort(). The temporary storage buffer must be
  allocated by the user.*/
    pub fn hipsparseXcoosort_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        cooRows: *const ::core::ffi::c_int,
        cooCols: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief Sort a sparse COO matrix by row

  \details
  \p hipsparseXcoosortByRow sorts a matrix in COO format by row. The sorted
  permutation vector \p perm can be used to obtain sorted \p coo_val array. In this
  case, \p perm must be initialized as the identity permutation, see
  hipsparseCreateIdentityPermutation().

  \p hipsparseXcoosortByRow requires extra temporary storage buffer that has to be
  allocated by the user. Storage buffer size can be determined by
  hipsparseXcoosort_bufferSizeExt().

  \note
  \p perm can be \p NULL if a sorted permutation vector is not required.

  \note
  This function is non blocking and executed asynchronously with respect to the host.
  It may return before the actual computation has finished.*/
    pub fn hipsparseXcoosortByRow(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        cooRows: *mut ::core::ffi::c_int,
        cooCols: *mut ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief Sort a sparse COO matrix by column

  \details
  \p hipsparseXcoosortByColumn sorts a matrix in COO format by column. The sorted
  permutation vector \p perm can be used to obtain sorted \p coo_val array. In this
  case, \p perm must be initialized as the identity permutation, see
  hipsparseCreateIdentityPermutation().

  \p hipsparseXcoosortByColumn requires extra temporary storage buffer that has to be
  allocated by the user. Storage buffer size can be determined by
  hipsparseXcoosort_bufferSizeExt().

  \note
  \p perm can be \p NULL if a sorted permutation vector is not required.

  \note
  This function is non blocking and executed asynchronously with respect to the host.
  It may return before the actual computation has finished.*/
    pub fn hipsparseXcoosortByColumn(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        cooRows: *mut ::core::ffi::c_int,
        cooCols: *mut ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function computes the the size of the user allocated temporary storage buffer used when converting a sparse\n  general BSR matrix to another sparse general BSR matrix.\n\n  \\details\n  \\p hipsparseXgebsr2gebsr_bufferSize returns the size of the temporary storage buffer\n  that is required by hipsparseXgebsr2gebsrNnz() and hipsparseXgebsr2gebsr().\n  The temporary storage buffer must be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseSgebsr2gebsr_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const f32,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        bufferSize: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgebsr2gebsr_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const f64,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        bufferSize: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgebsr2gebsr_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const hipComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        bufferSize: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgebsr2gebsr_bufferSize(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const hipDoubleComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        bufferSize: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** \ingroup conv_module
  \brief This function is used when converting a general BSR sparse matrix \p A to another general BSR sparse matrix \p C.
  Specifically, this function determines the number of non-zero blocks that will exist in \p C (stored using either a host
  or device pointer), and computes the row pointer array for \p C.

  \details
  The routine does support asynchronous execution.*/
    pub fn hipsparseXgebsr2gebsrNnz(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        bsrRowPtrC: *mut ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function converts the general BSR sparse matrix \\p A to another general BSR sparse matrix \\p C.\n\n  \\details\n  The conversion uses three steps. First, the user calls hipsparseXgebsr2gebsr_bufferSize() to determine the size of\n  the required temporary storage buffer. The user then allocates this buffer. Secondly, the user then allocates \\p mb_C+1\n  integers for the row pointer array for \\p C where \\p mb_C=(m+row_block_dim_C-1)/row_block_dim_C. The user then calls\n  hipsparseXgebsr2gebsrNnz() to fill in the row pointer array for \\p C ( \\p bsr_row_ptr_C ) and determine the number of\n  non-zero blocks that will exist in \\p C. Finally, the user allocates space for the colimn indices array of \\p C to have\n  \\p nnzb_C elements and space for the values array of \\p C to have \\p nnzb_C*roc_block_dim_C*col_block_dim_C and then calls\n  hipsparseXgebsr2gebsr() to complete the conversion.\n/\n/**@{"]
    pub fn hipsparseSgebsr2gebsr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const f32,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        bsrValC: *mut f32,
        bsrRowPtrC: *mut ::core::ffi::c_int,
        bsrColIndC: *mut ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDgebsr2gebsr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const f64,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        bsrValC: *mut f64,
        bsrRowPtrC: *mut ::core::ffi::c_int,
        bsrColIndC: *mut ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCgebsr2gebsr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const hipComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        bsrValC: *mut hipComplex,
        bsrRowPtrC: *mut ::core::ffi::c_int,
        bsrColIndC: *mut ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZgebsr2gebsr(
        handle: hipsparseHandle_t,
        dirA: hipsparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        bsrValA: *const hipDoubleComplex,
        bsrRowPtrA: *const ::core::ffi::c_int,
        bsrColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        descrC: hipsparseMatDescr_t,
        bsrValC: *mut hipDoubleComplex,
        bsrRowPtrC: *mut ::core::ffi::c_int,
        bsrColIndC: *mut ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function calculates the amount of temporary storage in bytes required for\n  hipsparseXcsru2csr() and hipsparseXcsr2csru().\n/\n/**@{"]
    pub fn hipsparseScsru2csr_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrVal: *mut f32,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: csru2csrInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsru2csr_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrVal: *mut f64,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: csru2csrInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsru2csr_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrVal: *mut hipComplex,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: csru2csrInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsru2csr_bufferSizeExt(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrVal: *mut hipDoubleComplex,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: csru2csrInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function converts unsorted CSR format to sorted CSR format. The required\n  temporary storage has to be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseScsru2csr(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrVal: *mut f32,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsru2csr(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrVal: *mut f64,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsru2csr(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrVal: *mut hipComplex,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsru2csr(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrVal: *mut hipDoubleComplex,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup conv_module\n  \\brief\n  This function converts sorted CSR format to unsorted CSR format. The required\n  temporary storage has to be allocated by the user.\n/\n/**@{"]
    pub fn hipsparseScsr2csru(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrVal: *mut f32,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsr2csru(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrVal: *mut f64,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsr2csru(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrVal: *mut hipComplex,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsr2csru(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrVal: *mut hipDoubleComplex,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\ingroup reordering_module\n  \\brief Coloring of the adjacency graph of the matrix \\f$A\\f$ stored in the CSR format.\n\n  \\details\n  \\p hipsparseXcsrcolor performs the coloring of the undirected graph represented by the (symmetric) sparsity pattern of the matrix \\f$A\\f$ stored in CSR format. Graph coloring is a way of coloring the nodes of a graph such that no two adjacent nodes are of the same color. The \\p fraction_to_color is a parameter to only color a given percentage of the graph nodes, the remaining uncolored nodes receive distinct new colors. The optional \\p reordering array is a permutation array such that unknowns of the same color are grouped. The matrix \\f$A\\f$ must be stored as a general matrix with a symmetric sparsity pattern, and if the matrix \\f$A\\f$ is non-symmetric then the user is responsible to provide the symmetric part \\f$\\frac{A+A^T}{2}\\f$.\n/\n/**@{"]
    pub fn hipsparseScsrcolor(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f32,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        fractionToColor: *const f32,
        ncolors: *mut ::core::ffi::c_int,
        coloring: *mut ::core::ffi::c_int,
        reordering: *mut ::core::ffi::c_int,
        info: hipsparseColorInfo_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDcsrcolor(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const f64,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        fractionToColor: *const f64,
        ncolors: *mut ::core::ffi::c_int,
        coloring: *mut ::core::ffi::c_int,
        reordering: *mut ::core::ffi::c_int,
        info: hipsparseColorInfo_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCcsrcolor(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const hipComplex,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        fractionToColor: *const f32,
        ncolors: *mut ::core::ffi::c_int,
        coloring: *mut ::core::ffi::c_int,
        reordering: *mut ::core::ffi::c_int,
        info: hipsparseColorInfo_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseZcsrcolor(
        handle: hipsparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: hipsparseMatDescr_t,
        csrValA: *const hipDoubleComplex,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        fractionToColor: *const f64,
        ncolors: *mut ::core::ffi::c_int,
        coloring: *mut ::core::ffi::c_int,
        reordering: *mut ::core::ffi::c_int,
        info: hipsparseColorInfo_t,
    ) -> hipsparseStatus_t;
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSpVecDescr_t(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseDnVecDescr_t(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSpMatDescr_t(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseDnMatDescr_t(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseConstSpVecDescr_t(pub *const ::core::ffi::c_void);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseConstDnVecDescr_t(pub *const ::core::ffi::c_void);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseConstSpMatDescr_t(pub *const ::core::ffi::c_void);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseConstDnMatDescr_t(pub *const ::core::ffi::c_void);
/// \cond DO_NOT_DOCUMENT
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipsparseSpGEMMDescr {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipsparseSpSVDescr {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipsparseSpSMDescr {
    _unused: [u8; 0],
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSpGEMMDescr_t(pub *mut hipsparseSpGEMMDescr);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSpSVDescr_t(pub *mut hipsparseSpSVDescr);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSpSMDescr_t(pub *mut hipsparseSpSMDescr);
impl hipsparseFormat_t {
    pub const HIPSPARSE_FORMAT_CSR: hipsparseFormat_t = hipsparseFormat_t(1);
}
impl hipsparseFormat_t {
    pub const HIPSPARSE_FORMAT_CSC: hipsparseFormat_t = hipsparseFormat_t(2);
}
impl hipsparseFormat_t {
    pub const HIPSPARSE_FORMAT_COO: hipsparseFormat_t = hipsparseFormat_t(3);
}
impl hipsparseFormat_t {
    pub const HIPSPARSE_FORMAT_COO_AOS: hipsparseFormat_t = hipsparseFormat_t(4);
}
impl hipsparseFormat_t {
    pub const HIPSPARSE_FORMAT_BLOCKED_ELL: hipsparseFormat_t = hipsparseFormat_t(5);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseFormat_t(pub ::core::ffi::c_uint);
impl hipsparseOrder_t {
    pub const HIPSPARSE_ORDER_COLUMN: hipsparseOrder_t = hipsparseOrder_t(1);
}
impl hipsparseOrder_t {
    ///< Column major
    pub const HIPSPARSE_ORDER_COL: hipsparseOrder_t = hipsparseOrder_t(1);
}
impl hipsparseOrder_t {
    ///< Row major
    pub const HIPSPARSE_ORDER_ROW: hipsparseOrder_t = hipsparseOrder_t(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseOrder_t(pub ::core::ffi::c_uint);
impl hipsparseIndexType_t {
    ///< 16 bit unsigned integer indices
    pub const HIPSPARSE_INDEX_16U: hipsparseIndexType_t = hipsparseIndexType_t(1);
}
impl hipsparseIndexType_t {
    ///< 32 bit signed integer indices
    pub const HIPSPARSE_INDEX_32I: hipsparseIndexType_t = hipsparseIndexType_t(2);
}
impl hipsparseIndexType_t {
    ///< 64 bit signed integer indices
    pub const HIPSPARSE_INDEX_64I: hipsparseIndexType_t = hipsparseIndexType_t(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseIndexType_t(pub ::core::ffi::c_uint);
impl hipsparseSpMVAlg_t {
    pub const HIPSPARSE_MV_ALG_DEFAULT: hipsparseSpMVAlg_t = hipsparseSpMVAlg_t(0);
}
impl hipsparseSpMVAlg_t {
    pub const HIPSPARSE_COOMV_ALG: hipsparseSpMVAlg_t = hipsparseSpMVAlg_t(1);
}
impl hipsparseSpMVAlg_t {
    pub const HIPSPARSE_CSRMV_ALG1: hipsparseSpMVAlg_t = hipsparseSpMVAlg_t(2);
}
impl hipsparseSpMVAlg_t {
    pub const HIPSPARSE_CSRMV_ALG2: hipsparseSpMVAlg_t = hipsparseSpMVAlg_t(3);
}
impl hipsparseSpMVAlg_t {
    pub const HIPSPARSE_SPMV_ALG_DEFAULT: hipsparseSpMVAlg_t = hipsparseSpMVAlg_t(0);
}
impl hipsparseSpMVAlg_t {
    pub const HIPSPARSE_SPMV_COO_ALG1: hipsparseSpMVAlg_t = hipsparseSpMVAlg_t(1);
}
impl hipsparseSpMVAlg_t {
    pub const HIPSPARSE_SPMV_CSR_ALG1: hipsparseSpMVAlg_t = hipsparseSpMVAlg_t(2);
}
impl hipsparseSpMVAlg_t {
    pub const HIPSPARSE_SPMV_CSR_ALG2: hipsparseSpMVAlg_t = hipsparseSpMVAlg_t(3);
}
impl hipsparseSpMVAlg_t {
    pub const HIPSPARSE_SPMV_COO_ALG2: hipsparseSpMVAlg_t = hipsparseSpMVAlg_t(4);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSpMVAlg_t(pub ::core::ffi::c_uint);
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_MM_ALG_DEFAULT: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(0);
}
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_COOMM_ALG1: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(1);
}
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_COOMM_ALG2: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(2);
}
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_COOMM_ALG3: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(3);
}
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_CSRMM_ALG1: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(4);
}
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_SPMM_ALG_DEFAULT: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(0);
}
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_SPMM_COO_ALG1: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(1);
}
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_SPMM_COO_ALG2: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(2);
}
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_SPMM_COO_ALG3: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(3);
}
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_SPMM_COO_ALG4: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(5);
}
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_SPMM_CSR_ALG1: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(4);
}
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_SPMM_CSR_ALG2: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(6);
}
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_SPMM_CSR_ALG3: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(12);
}
impl hipsparseSpMMAlg_t {
    pub const HIPSPARSE_SPMM_BLOCKED_ELL_ALG1: hipsparseSpMMAlg_t = hipsparseSpMMAlg_t(
        13,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSpMMAlg_t(pub ::core::ffi::c_uint);
impl hipsparseSparseToDenseAlg_t {
    pub const HIPSPARSE_SPARSETODENSE_ALG_DEFAULT: hipsparseSparseToDenseAlg_t = hipsparseSparseToDenseAlg_t(
        0,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSparseToDenseAlg_t(pub ::core::ffi::c_uint);
impl hipsparseDenseToSparseAlg_t {
    pub const HIPSPARSE_DENSETOSPARSE_ALG_DEFAULT: hipsparseDenseToSparseAlg_t = hipsparseDenseToSparseAlg_t(
        0,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseDenseToSparseAlg_t(pub ::core::ffi::c_uint);
impl hipsparseSDDMMAlg_t {
    pub const HIPSPARSE_SDDMM_ALG_DEFAULT: hipsparseSDDMMAlg_t = hipsparseSDDMMAlg_t(0);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSDDMMAlg_t(pub ::core::ffi::c_uint);
impl hipsparseSpSVAlg_t {
    pub const HIPSPARSE_SPSV_ALG_DEFAULT: hipsparseSpSVAlg_t = hipsparseSpSVAlg_t(0);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSpSVAlg_t(pub ::core::ffi::c_uint);
impl hipsparseSpSMAlg_t {
    pub const HIPSPARSE_SPSM_ALG_DEFAULT: hipsparseSpSMAlg_t = hipsparseSpSMAlg_t(0);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSpSMAlg_t(pub ::core::ffi::c_uint);
impl hipsparseSpMatAttribute_t {
    ///< Fill mode attribute
    pub const HIPSPARSE_SPMAT_FILL_MODE: hipsparseSpMatAttribute_t = hipsparseSpMatAttribute_t(
        0,
    );
}
impl hipsparseSpMatAttribute_t {
    ///< Diag type attribute
    pub const HIPSPARSE_SPMAT_DIAG_TYPE: hipsparseSpMatAttribute_t = hipsparseSpMatAttribute_t(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSpMatAttribute_t(pub ::core::ffi::c_uint);
impl hipsparseSpGEMMAlg_t {
    pub const HIPSPARSE_SPGEMM_DEFAULT: hipsparseSpGEMMAlg_t = hipsparseSpGEMMAlg_t(0);
}
impl hipsparseSpGEMMAlg_t {
    pub const HIPSPARSE_SPGEMM_CSR_ALG_DETERMINISTIC: hipsparseSpGEMMAlg_t = hipsparseSpGEMMAlg_t(
        1,
    );
}
impl hipsparseSpGEMMAlg_t {
    pub const HIPSPARSE_SPGEMM_CSR_ALG_NONDETERMINISTIC: hipsparseSpGEMMAlg_t = hipsparseSpGEMMAlg_t(
        2,
    );
}
impl hipsparseSpGEMMAlg_t {
    pub const HIPSPARSE_SPGEMM_ALG1: hipsparseSpGEMMAlg_t = hipsparseSpGEMMAlg_t(3);
}
impl hipsparseSpGEMMAlg_t {
    pub const HIPSPARSE_SPGEMM_ALG2: hipsparseSpGEMMAlg_t = hipsparseSpGEMMAlg_t(4);
}
impl hipsparseSpGEMMAlg_t {
    pub const HIPSPARSE_SPGEMM_ALG3: hipsparseSpGEMMAlg_t = hipsparseSpGEMMAlg_t(5);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipsparseSpGEMMAlg_t(pub ::core::ffi::c_uint);
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateSpVec(
        spVecDescr: *mut hipsparseSpVecDescr_t,
        size: i64,
        nnz: i64,
        indices: *mut ::core::ffi::c_void,
        values: *mut ::core::ffi::c_void,
        idxType: hipsparseIndexType_t,
        idxBase: hipsparseIndexBase_t,
        valueType: hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateConstSpVec(
        spVecDescr: *mut hipsparseConstSpVecDescr_t,
        size: i64,
        nnz: i64,
        indices: *const ::core::ffi::c_void,
        values: *const ::core::ffi::c_void,
        idxType: hipsparseIndexType_t,
        idxBase: hipsparseIndexBase_t,
        valueType: hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDestroySpVec(
        spVecDescr: hipsparseConstSpVecDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpVecGet(
        spVecDescr: hipsparseSpVecDescr_t,
        size: *mut i64,
        nnz: *mut i64,
        indices: *mut *mut ::core::ffi::c_void,
        values: *mut *mut ::core::ffi::c_void,
        idxType: *mut hipsparseIndexType_t,
        idxBase: *mut hipsparseIndexBase_t,
        valueType: *mut hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseConstSpVecGet(
        spVecDescr: hipsparseConstSpVecDescr_t,
        size: *mut i64,
        nnz: *mut i64,
        indices: *mut *const ::core::ffi::c_void,
        values: *mut *const ::core::ffi::c_void,
        idxType: *mut hipsparseIndexType_t,
        idxBase: *mut hipsparseIndexBase_t,
        valueType: *mut hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpVecGetIndexBase(
        spVecDescr: hipsparseConstSpVecDescr_t,
        idxBase: *mut hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpVecGetValues(
        spVecDescr: hipsparseSpVecDescr_t,
        values: *mut *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseConstSpVecGetValues(
        spVecDescr: hipsparseConstSpVecDescr_t,
        values: *mut *const ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpVecSetValues(
        spVecDescr: hipsparseSpVecDescr_t,
        values: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateCoo(
        spMatDescr: *mut hipsparseSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        cooRowInd: *mut ::core::ffi::c_void,
        cooColInd: *mut ::core::ffi::c_void,
        cooValues: *mut ::core::ffi::c_void,
        cooIdxType: hipsparseIndexType_t,
        idxBase: hipsparseIndexBase_t,
        valueType: hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateConstCoo(
        spMatDescr: *mut hipsparseConstSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        cooRowInd: *const ::core::ffi::c_void,
        cooColInd: *const ::core::ffi::c_void,
        cooValues: *const ::core::ffi::c_void,
        cooIdxType: hipsparseIndexType_t,
        idxBase: hipsparseIndexBase_t,
        valueType: hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateCooAoS(
        spMatDescr: *mut hipsparseSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        cooInd: *mut ::core::ffi::c_void,
        cooValues: *mut ::core::ffi::c_void,
        cooIdxType: hipsparseIndexType_t,
        idxBase: hipsparseIndexBase_t,
        valueType: hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateCsr(
        spMatDescr: *mut hipsparseSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        csrRowOffsets: *mut ::core::ffi::c_void,
        csrColInd: *mut ::core::ffi::c_void,
        csrValues: *mut ::core::ffi::c_void,
        csrRowOffsetsType: hipsparseIndexType_t,
        csrColIndType: hipsparseIndexType_t,
        idxBase: hipsparseIndexBase_t,
        valueType: hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateConstCsr(
        spMatDescr: *mut hipsparseConstSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        csrRowOffsets: *const ::core::ffi::c_void,
        csrColInd: *const ::core::ffi::c_void,
        csrValues: *const ::core::ffi::c_void,
        csrRowOffsetsType: hipsparseIndexType_t,
        csrColIndType: hipsparseIndexType_t,
        idxBase: hipsparseIndexBase_t,
        valueType: hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateCsc(
        spMatDescr: *mut hipsparseSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        cscColOffsets: *mut ::core::ffi::c_void,
        cscRowInd: *mut ::core::ffi::c_void,
        cscValues: *mut ::core::ffi::c_void,
        cscColOffsetsType: hipsparseIndexType_t,
        cscRowIndType: hipsparseIndexType_t,
        idxBase: hipsparseIndexBase_t,
        valueType: hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateConstCsc(
        spMatDescr: *mut hipsparseConstSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        cscColOffsets: *const ::core::ffi::c_void,
        cscRowInd: *const ::core::ffi::c_void,
        cscValues: *const ::core::ffi::c_void,
        cscColOffsetsType: hipsparseIndexType_t,
        cscRowIndType: hipsparseIndexType_t,
        idxBase: hipsparseIndexBase_t,
        valueType: hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateBlockedEll(
        spMatDescr: *mut hipsparseSpMatDescr_t,
        rows: i64,
        cols: i64,
        ellBlockSize: i64,
        ellCols: i64,
        ellColInd: *mut ::core::ffi::c_void,
        ellValue: *mut ::core::ffi::c_void,
        ellIdxType: hipsparseIndexType_t,
        idxBase: hipsparseIndexBase_t,
        valueType: hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateConstBlockedEll(
        spMatDescr: *mut hipsparseConstSpMatDescr_t,
        rows: i64,
        cols: i64,
        ellBlockSize: i64,
        ellCols: i64,
        ellColInd: *const ::core::ffi::c_void,
        ellValue: *const ::core::ffi::c_void,
        ellIdxType: hipsparseIndexType_t,
        idxBase: hipsparseIndexBase_t,
        valueType: hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDestroySpMat(
        spMatDescr: hipsparseConstSpMatDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCooGet(
        spMatDescr: hipsparseSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
        cooRowInd: *mut *mut ::core::ffi::c_void,
        cooColInd: *mut *mut ::core::ffi::c_void,
        cooValues: *mut *mut ::core::ffi::c_void,
        idxType: *mut hipsparseIndexType_t,
        idxBase: *mut hipsparseIndexBase_t,
        valueType: *mut hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseConstCooGet(
        spMatDescr: hipsparseConstSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
        cooRowInd: *mut *const ::core::ffi::c_void,
        cooColInd: *mut *const ::core::ffi::c_void,
        cooValues: *mut *const ::core::ffi::c_void,
        idxType: *mut hipsparseIndexType_t,
        idxBase: *mut hipsparseIndexBase_t,
        valueType: *mut hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCooAoSGet(
        spMatDescr: hipsparseSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
        cooInd: *mut *mut ::core::ffi::c_void,
        cooValues: *mut *mut ::core::ffi::c_void,
        idxType: *mut hipsparseIndexType_t,
        idxBase: *mut hipsparseIndexBase_t,
        valueType: *mut hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCsrGet(
        spMatDescr: hipsparseSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
        csrRowOffsets: *mut *mut ::core::ffi::c_void,
        csrColInd: *mut *mut ::core::ffi::c_void,
        csrValues: *mut *mut ::core::ffi::c_void,
        csrRowOffsetsType: *mut hipsparseIndexType_t,
        csrColIndType: *mut hipsparseIndexType_t,
        idxBase: *mut hipsparseIndexBase_t,
        valueType: *mut hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseConstCsrGet(
        spMatDescr: hipsparseConstSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
        csrRowOffsets: *mut *const ::core::ffi::c_void,
        csrColInd: *mut *const ::core::ffi::c_void,
        csrValues: *mut *const ::core::ffi::c_void,
        csrRowOffsetsType: *mut hipsparseIndexType_t,
        csrColIndType: *mut hipsparseIndexType_t,
        idxBase: *mut hipsparseIndexBase_t,
        valueType: *mut hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCscGet(
        spMatDescr: hipsparseSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
        cscColOffsets: *mut *mut ::core::ffi::c_void,
        cscRowInd: *mut *mut ::core::ffi::c_void,
        cscValues: *mut *mut ::core::ffi::c_void,
        cscColOffsetsType: *mut hipsparseIndexType_t,
        cscRowIndType: *mut hipsparseIndexType_t,
        idxBase: *mut hipsparseIndexBase_t,
        valueType: *mut hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseConstCscGet(
        spMatDescr: hipsparseConstSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
        cscColOffsets: *mut *const ::core::ffi::c_void,
        cscRowInd: *mut *const ::core::ffi::c_void,
        cscValues: *mut *const ::core::ffi::c_void,
        cscColOffsetsType: *mut hipsparseIndexType_t,
        cscRowIndType: *mut hipsparseIndexType_t,
        idxBase: *mut hipsparseIndexBase_t,
        valueType: *mut hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseBlockedEllGet(
        spMatDescr: hipsparseSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        ellBlockSize: *mut i64,
        ellCols: *mut i64,
        ellColInd: *mut *mut ::core::ffi::c_void,
        ellValue: *mut *mut ::core::ffi::c_void,
        ellIdxType: *mut hipsparseIndexType_t,
        idxBase: *mut hipsparseIndexBase_t,
        valueType: *mut hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseConstBlockedEllGet(
        spMatDescr: hipsparseConstSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        ellBlockSize: *mut i64,
        ellCols: *mut i64,
        ellColInd: *mut *const ::core::ffi::c_void,
        ellValue: *mut *const ::core::ffi::c_void,
        ellIdxType: *mut hipsparseIndexType_t,
        idxBase: *mut hipsparseIndexBase_t,
        valueType: *mut hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCsrSetPointers(
        spMatDescr: hipsparseSpMatDescr_t,
        csrRowOffsets: *mut ::core::ffi::c_void,
        csrColInd: *mut ::core::ffi::c_void,
        csrValues: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCscSetPointers(
        spMatDescr: hipsparseSpMatDescr_t,
        cscColOffsets: *mut ::core::ffi::c_void,
        cscRowInd: *mut ::core::ffi::c_void,
        cscValues: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCooSetPointers(
        spMatDescr: hipsparseSpMatDescr_t,
        cooRowInd: *mut ::core::ffi::c_void,
        cooColInd: *mut ::core::ffi::c_void,
        cooValues: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMatGetSize(
        spMatDescr: hipsparseConstSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMatGetFormat(
        spMatDescr: hipsparseConstSpMatDescr_t,
        format: *mut hipsparseFormat_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMatGetIndexBase(
        spMatDescr: hipsparseConstSpMatDescr_t,
        idxBase: *mut hipsparseIndexBase_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMatGetValues(
        spMatDescr: hipsparseSpMatDescr_t,
        values: *mut *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseConstSpMatGetValues(
        spMatDescr: hipsparseConstSpMatDescr_t,
        values: *mut *const ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMatSetValues(
        spMatDescr: hipsparseSpMatDescr_t,
        values: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMatGetStridedBatch(
        spMatDescr: hipsparseConstSpMatDescr_t,
        batchCount: *mut ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMatSetStridedBatch(
        spMatDescr: hipsparseSpMatDescr_t,
        batchCount: ::core::ffi::c_int,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCooSetStridedBatch(
        spMatDescr: hipsparseSpMatDescr_t,
        batchCount: ::core::ffi::c_int,
        batchStride: i64,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCsrSetStridedBatch(
        spMatDescr: hipsparseSpMatDescr_t,
        batchCount: ::core::ffi::c_int,
        offsetsBatchStride: i64,
        columnsValuesBatchStride: i64,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMatGetAttribute(
        spMatDescr: hipsparseConstSpMatDescr_t,
        attribute: hipsparseSpMatAttribute_t,
        data: *mut ::core::ffi::c_void,
        dataSize: usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMatSetAttribute(
        spMatDescr: hipsparseSpMatDescr_t,
        attribute: hipsparseSpMatAttribute_t,
        data: *const ::core::ffi::c_void,
        dataSize: usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateDnVec(
        dnVecDescr: *mut hipsparseDnVecDescr_t,
        size: i64,
        values: *mut ::core::ffi::c_void,
        valueType: hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateConstDnVec(
        dnVecDescr: *mut hipsparseConstDnVecDescr_t,
        size: i64,
        values: *const ::core::ffi::c_void,
        valueType: hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDestroyDnVec(
        dnVecDescr: hipsparseConstDnVecDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDnVecGet(
        dnVecDescr: hipsparseDnVecDescr_t,
        size: *mut i64,
        values: *mut *mut ::core::ffi::c_void,
        valueType: *mut hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseConstDnVecGet(
        dnVecDescr: hipsparseConstDnVecDescr_t,
        size: *mut i64,
        values: *mut *const ::core::ffi::c_void,
        valueType: *mut hipDataType,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDnVecGetValues(
        dnVecDescr: hipsparseDnVecDescr_t,
        values: *mut *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseConstDnVecGetValues(
        dnVecDescr: hipsparseConstDnVecDescr_t,
        values: *mut *const ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDnVecSetValues(
        dnVecDescr: hipsparseDnVecDescr_t,
        values: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateDnMat(
        dnMatDescr: *mut hipsparseDnMatDescr_t,
        rows: i64,
        cols: i64,
        ld: i64,
        values: *mut ::core::ffi::c_void,
        valueType: hipDataType,
        order: hipsparseOrder_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseCreateConstDnMat(
        dnMatDescr: *mut hipsparseConstDnMatDescr_t,
        rows: i64,
        cols: i64,
        ld: i64,
        values: *const ::core::ffi::c_void,
        valueType: hipDataType,
        order: hipsparseOrder_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDestroyDnMat(
        dnMatDescr: hipsparseConstDnMatDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDnMatGet(
        dnMatDescr: hipsparseDnMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        ld: *mut i64,
        values: *mut *mut ::core::ffi::c_void,
        valueType: *mut hipDataType,
        order: *mut hipsparseOrder_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseConstDnMatGet(
        dnMatDescr: hipsparseConstDnMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        ld: *mut i64,
        values: *mut *const ::core::ffi::c_void,
        valueType: *mut hipDataType,
        order: *mut hipsparseOrder_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDnMatGetValues(
        dnMatDescr: hipsparseDnMatDescr_t,
        values: *mut *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseConstDnMatGetValues(
        dnMatDescr: hipsparseConstDnMatDescr_t,
        values: *mut *const ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDnMatSetValues(
        dnMatDescr: hipsparseDnMatDescr_t,
        values: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDnMatGetStridedBatch(
        dnMatDescr: hipsparseConstDnMatDescr_t,
        batchCount: *mut ::core::ffi::c_int,
        batchStride: *mut i64,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDnMatSetStridedBatch(
        dnMatDescr: hipsparseDnMatDescr_t,
        batchCount: ::core::ffi::c_int,
        batchStride: i64,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseAxpby(
        handle: hipsparseHandle_t,
        alpha: *const ::core::ffi::c_void,
        vecX: hipsparseConstSpVecDescr_t,
        beta: *const ::core::ffi::c_void,
        vecY: hipsparseDnVecDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseGather(
        handle: hipsparseHandle_t,
        vecY: hipsparseConstDnVecDescr_t,
        vecX: hipsparseSpVecDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseScatter(
        handle: hipsparseHandle_t,
        vecX: hipsparseConstSpVecDescr_t,
        vecY: hipsparseDnVecDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseRot(
        handle: hipsparseHandle_t,
        c_coeff: *const ::core::ffi::c_void,
        s_coeff: *const ::core::ffi::c_void,
        vecX: hipsparseSpVecDescr_t,
        vecY: hipsparseDnVecDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSparseToDense_bufferSize(
        handle: hipsparseHandle_t,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseDnMatDescr_t,
        alg: hipsparseSparseToDenseAlg_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSparseToDense(
        handle: hipsparseHandle_t,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseDnMatDescr_t,
        alg: hipsparseSparseToDenseAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDenseToSparse_bufferSize(
        handle: hipsparseHandle_t,
        matA: hipsparseConstDnMatDescr_t,
        matB: hipsparseSpMatDescr_t,
        alg: hipsparseDenseToSparseAlg_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDenseToSparse_analysis(
        handle: hipsparseHandle_t,
        matA: hipsparseConstDnMatDescr_t,
        matB: hipsparseSpMatDescr_t,
        alg: hipsparseDenseToSparseAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseDenseToSparse_convert(
        handle: hipsparseHandle_t,
        matA: hipsparseConstDnMatDescr_t,
        matB: hipsparseSpMatDescr_t,
        alg: hipsparseDenseToSparseAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpVV_bufferSize(
        handle: hipsparseHandle_t,
        opX: hipsparseOperation_t,
        vecX: hipsparseConstSpVecDescr_t,
        vecY: hipsparseConstDnVecDescr_t,
        result: *mut ::core::ffi::c_void,
        computeType: hipDataType,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpVV(
        handle: hipsparseHandle_t,
        opX: hipsparseOperation_t,
        vecX: hipsparseConstSpVecDescr_t,
        vecY: hipsparseConstDnVecDescr_t,
        result: *mut ::core::ffi::c_void,
        computeType: hipDataType,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMV_bufferSize(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        vecX: hipsparseConstDnVecDescr_t,
        beta: *const ::core::ffi::c_void,
        vecY: hipsparseDnVecDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpMVAlg_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMV_preprocess(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        vecX: hipsparseConstDnVecDescr_t,
        beta: *const ::core::ffi::c_void,
        vecY: hipsparseDnVecDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpMVAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMV(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        vecX: hipsparseConstDnVecDescr_t,
        beta: *const ::core::ffi::c_void,
        vecY: hipsparseDnVecDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpMVAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMM_bufferSize(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseConstDnMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: hipsparseDnMatDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpMMAlg_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMM_preprocess(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseConstDnMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: hipsparseDnMatDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpMMAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpMM(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseConstDnMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: hipsparseDnMatDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpMMAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpGEMM_createDescr(
        descr: *mut hipsparseSpGEMMDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpGEMM_destroyDescr(
        descr: hipsparseSpGEMMDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpGEMM_workEstimation(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseConstSpMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: hipsparseSpMatDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpGEMMAlg_t,
        spgemmDescr: hipsparseSpGEMMDescr_t,
        bufferSize1: *mut usize,
        externalBuffer1: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpGEMM_compute(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseConstSpMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: hipsparseSpMatDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpGEMMAlg_t,
        spgemmDescr: hipsparseSpGEMMDescr_t,
        bufferSize2: *mut usize,
        externalBuffer2: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpGEMM_copy(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseConstSpMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: hipsparseSpMatDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpGEMMAlg_t,
        spgemmDescr: hipsparseSpGEMMDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpGEMMreuse_workEstimation(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseConstSpMatDescr_t,
        matC: hipsparseSpMatDescr_t,
        alg: hipsparseSpGEMMAlg_t,
        spgemmDescr: hipsparseSpGEMMDescr_t,
        bufferSize1: *mut usize,
        externalBuffer1: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpGEMMreuse_nnz(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseConstSpMatDescr_t,
        matC: hipsparseSpMatDescr_t,
        alg: hipsparseSpGEMMAlg_t,
        spgemmDescr: hipsparseSpGEMMDescr_t,
        bufferSize2: *mut usize,
        externalBuffer2: *mut ::core::ffi::c_void,
        bufferSize3: *mut usize,
        externalBuffer3: *mut ::core::ffi::c_void,
        bufferSize4: *mut usize,
        externalBuffer4: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpGEMMreuse_copy(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseConstSpMatDescr_t,
        matC: hipsparseSpMatDescr_t,
        alg: hipsparseSpGEMMAlg_t,
        spgemmDescr: hipsparseSpGEMMDescr_t,
        bufferSize5: *mut usize,
        externalBuffer5: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpGEMMreuse_compute(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseConstSpMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: hipsparseSpMatDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpGEMMAlg_t,
        spgemmDescr: hipsparseSpGEMMDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSDDMM_bufferSize(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        A: hipsparseConstDnMatDescr_t,
        B: hipsparseConstDnMatDescr_t,
        beta: *const ::core::ffi::c_void,
        C: hipsparseSpMatDescr_t,
        computeType: hipDataType,
        alg: hipsparseSDDMMAlg_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSDDMM_preprocess(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        A: hipsparseConstDnMatDescr_t,
        B: hipsparseConstDnMatDescr_t,
        beta: *const ::core::ffi::c_void,
        C: hipsparseSpMatDescr_t,
        computeType: hipDataType,
        alg: hipsparseSDDMMAlg_t,
        tempBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSDDMM(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        A: hipsparseConstDnMatDescr_t,
        B: hipsparseConstDnMatDescr_t,
        beta: *const ::core::ffi::c_void,
        C: hipsparseSpMatDescr_t,
        computeType: hipDataType,
        alg: hipsparseSDDMMAlg_t,
        tempBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpSV_createDescr(
        descr: *mut hipsparseSpSVDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpSV_destroyDescr(descr: hipsparseSpSVDescr_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpSV_bufferSize(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        x: hipsparseConstDnVecDescr_t,
        y: hipsparseDnVecDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpSVAlg_t,
        spsvDescr: hipsparseSpSVDescr_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpSV_analysis(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        x: hipsparseConstDnVecDescr_t,
        y: hipsparseDnVecDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpSVAlg_t,
        spsvDescr: hipsparseSpSVDescr_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpSV_solve(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        x: hipsparseConstDnVecDescr_t,
        y: hipsparseDnVecDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpSVAlg_t,
        spsvDescr: hipsparseSpSVDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpSM_createDescr(
        descr: *mut hipsparseSpSMDescr_t,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpSM_destroyDescr(descr: hipsparseSpSMDescr_t) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpSM_bufferSize(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseConstDnMatDescr_t,
        matC: hipsparseDnMatDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpSMAlg_t,
        spsmDescr: hipsparseSpSMDescr_t,
        bufferSize: *mut usize,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpSM_analysis(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseConstDnMatDescr_t,
        matC: hipsparseDnMatDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpSMAlg_t,
        spsmDescr: hipsparseSpSMDescr_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
#[cfg_attr(windows, link(name = "hipsparse", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipsparseSpSM_solve(
        handle: hipsparseHandle_t,
        opA: hipsparseOperation_t,
        opB: hipsparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: hipsparseConstSpMatDescr_t,
        matB: hipsparseConstDnMatDescr_t,
        matC: hipsparseDnMatDescr_t,
        computeType: hipDataType,
        alg: hipsparseSpSMAlg_t,
        spsmDescr: hipsparseSpSMDescr_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> hipsparseStatus_t;
}
impl hipsparseError_t {
    pub const r#NOT_INITIALIZED: hipsparseError_t = hipsparseError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1)
    });
    pub const r#ALLOC_FAILED: hipsparseError_t = hipsparseError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2)
    });
    pub const r#INVALID_VALUE: hipsparseError_t = hipsparseError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3)
    });
    pub const r#ARCH_MISMATCH: hipsparseError_t = hipsparseError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4)
    });
    pub const r#MAPPING_ERROR: hipsparseError_t = hipsparseError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5)
    });
    pub const r#EXECUTION_FAILED: hipsparseError_t = hipsparseError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(6)
    });
    pub const r#INTERNAL_ERROR: hipsparseError_t = hipsparseError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(7)
    });
    pub const r#MATRIX_TYPE_NOT_SUPPORTED: hipsparseError_t = hipsparseError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(8)
    });
    pub const r#ZERO_PIVOT: hipsparseError_t = hipsparseError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(9)
    });
    pub const r#NOT_SUPPORTED: hipsparseError_t = hipsparseError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(10)
    });
    pub const r#INSUFFICIENT_RESOURCES: hipsparseError_t = hipsparseError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(11)
    });
}
#[repr(transparent)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct hipsparseError_t(pub ::core::num::NonZeroU32);
pub trait hipsparseStatus_tConsts {
    const SUCCESS: hipsparseStatus_t = hipsparseStatus_t::Ok(());
    const ERROR_NOT_INITIALIZED: hipsparseStatus_t = hipsparseStatus_t::Err(
        hipsparseError_t::r#NOT_INITIALIZED,
    );
    const ERROR_ALLOC_FAILED: hipsparseStatus_t = hipsparseStatus_t::Err(
        hipsparseError_t::r#ALLOC_FAILED,
    );
    const ERROR_INVALID_VALUE: hipsparseStatus_t = hipsparseStatus_t::Err(
        hipsparseError_t::r#INVALID_VALUE,
    );
    const ERROR_ARCH_MISMATCH: hipsparseStatus_t = hipsparseStatus_t::Err(
        hipsparseError_t::r#ARCH_MISMATCH,
    );
    const ERROR_MAPPING_ERROR: hipsparseStatus_t = hipsparseStatus_t::Err(
        hipsparseError_t::r#MAPPING_ERROR,
    );
    const ERROR_EXECUTION_FAILED: hipsparseStatus_t = hipsparseStatus_t::Err(
        hipsparseError_t::r#EXECUTION_FAILED,
    );
    const ERROR_INTERNAL_ERROR: hipsparseStatus_t = hipsparseStatus_t::Err(
        hipsparseError_t::r#INTERNAL_ERROR,
    );
    const ERROR_MATRIX_TYPE_NOT_SUPPORTED: hipsparseStatus_t = hipsparseStatus_t::Err(
        hipsparseError_t::r#MATRIX_TYPE_NOT_SUPPORTED,
    );
    const ERROR_ZERO_PIVOT: hipsparseStatus_t = hipsparseStatus_t::Err(
        hipsparseError_t::r#ZERO_PIVOT,
    );
    const ERROR_NOT_SUPPORTED: hipsparseStatus_t = hipsparseStatus_t::Err(
        hipsparseError_t::r#NOT_SUPPORTED,
    );
    const ERROR_INSUFFICIENT_RESOURCES: hipsparseStatus_t = hipsparseStatus_t::Err(
        hipsparseError_t::r#INSUFFICIENT_RESOURCES,
    );
}
impl hipsparseStatus_tConsts for hipsparseStatus_t {}
#[must_use]
pub type hipsparseStatus_t = ::core::result::Result<(), hipsparseError_t>;
const _: fn() = || {
    let _ = std::mem::transmute::<hipsparseStatus_t, u32>;
};
unsafe impl Send for hipsparseHandle_t {}
unsafe impl Sync for hipsparseHandle_t {}
unsafe impl Send for hipsparseMatDescr_t {}
unsafe impl Sync for hipsparseMatDescr_t {}
unsafe impl Send for hipsparseHybMat_t {}
unsafe impl Sync for hipsparseHybMat_t {}
unsafe impl Send for hipsparseColorInfo_t {}
unsafe impl Sync for hipsparseColorInfo_t {}
unsafe impl Send for hipsparseSpVecDescr_t {}
unsafe impl Sync for hipsparseSpVecDescr_t {}
unsafe impl Send for hipsparseDnVecDescr_t {}
unsafe impl Sync for hipsparseDnVecDescr_t {}
unsafe impl Send for hipsparseSpMatDescr_t {}
unsafe impl Sync for hipsparseSpMatDescr_t {}
unsafe impl Send for hipsparseDnMatDescr_t {}
unsafe impl Sync for hipsparseDnMatDescr_t {}
unsafe impl Send for hipsparseConstSpVecDescr_t {}
unsafe impl Sync for hipsparseConstSpVecDescr_t {}
unsafe impl Send for hipsparseConstDnVecDescr_t {}
unsafe impl Sync for hipsparseConstDnVecDescr_t {}
unsafe impl Send for hipsparseConstSpMatDescr_t {}
unsafe impl Sync for hipsparseConstSpMatDescr_t {}
unsafe impl Send for hipsparseConstDnMatDescr_t {}
unsafe impl Sync for hipsparseConstDnMatDescr_t {}
unsafe impl Send for hipsparseSpGEMMDescr_t {}
unsafe impl Sync for hipsparseSpGEMMDescr_t {}
unsafe impl Send for hipsparseSpSVDescr_t {}
unsafe impl Sync for hipsparseSpSVDescr_t {}
unsafe impl Send for hipsparseSpSMDescr_t {}
unsafe impl Sync for hipsparseSpSMDescr_t {}
