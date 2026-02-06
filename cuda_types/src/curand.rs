#![allow(warnings)]
pub type cudaStream_t = super::cuda::CUstream;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curandGenerator_st {
    _unused: [u8; 0],
}
pub type curandGenerator_t = *mut curandGenerator_st;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curandDiscreteDistribution_st {
    _unused: [u8; 0],
}
pub type curandDiscreteDistribution_t = *mut curandDiscreteDistribution_st;

pub type curandDirectionVectors32_t = [::core::ffi::c_uint; 32usize];
pub type curandDirectionVectors64_t = [::core::ffi::c_ulonglong; 64usize];

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct curandRngType_t(pub ::core::ffi::c_uint);

impl curandRngType_t {
    pub const CURAND_RNG_TEST: curandRngType_t = curandRngType_t(0);
    pub const CURAND_RNG_PSEUDO_DEFAULT: curandRngType_t = curandRngType_t(100);
    pub const CURAND_RNG_PSEUDO_XORWOW: curandRngType_t = curandRngType_t(101);
    pub const CURAND_RNG_PSEUDO_MRG32K3A: curandRngType_t = curandRngType_t(121);
    pub const CURAND_RNG_PSEUDO_MTGP32: curandRngType_t = curandRngType_t(141);
    pub const CURAND_RNG_PSEUDO_MT19937: curandRngType_t = curandRngType_t(142);
    pub const CURAND_RNG_PSEUDO_PHILOX4_32_10: curandRngType_t = curandRngType_t(161);
    pub const CURAND_RNG_QUASI_DEFAULT: curandRngType_t = curandRngType_t(200);
    pub const CURAND_RNG_QUASI_SOBOL32: curandRngType_t = curandRngType_t(201);
    pub const CURAND_RNG_QUASI_SCRAMBLED_SOBOL32: curandRngType_t = curandRngType_t(202);
    pub const CURAND_RNG_QUASI_SOBOL64: curandRngType_t = curandRngType_t(203);
    pub const CURAND_RNG_QUASI_SCRAMBLED_SOBOL64: curandRngType_t = curandRngType_t(204);
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct curandOrdering_t(pub ::core::ffi::c_uint);

impl curandOrdering_t {
    pub const CURAND_ORDERING_PSEUDO_BEST: curandOrdering_t = curandOrdering_t(100);
    pub const CURAND_ORDERING_PSEUDO_DEFAULT: curandOrdering_t = curandOrdering_t(101);
    pub const CURAND_ORDERING_PSEUDO_SEEDED: curandOrdering_t = curandOrdering_t(102);
    pub const CURAND_ORDERING_PSEUDO_LEGACY: curandOrdering_t = curandOrdering_t(103);
    pub const CURAND_ORDERING_PSEUDO_DYNAMIC: curandOrdering_t = curandOrdering_t(104);
    pub const CURAND_ORDERING_QUASI_DEFAULT: curandOrdering_t = curandOrdering_t(201);
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct curandDirectionVectorSet_t(pub ::core::ffi::c_uint);

impl curandDirectionVectorSet_t {
    pub const CURAND_DIRECTION_VECTORS_32_JOEKUO6: curandDirectionVectorSet_t = curandDirectionVectorSet_t(101);
    pub const CURAND_SCRAMBLED_DIRECTION_VECTORS_32_JOEKUO6: curandDirectionVectorSet_t = curandDirectionVectorSet_t(102);
    pub const CURAND_DIRECTION_VECTORS_64_JOEKUO6: curandDirectionVectorSet_t = curandDirectionVectorSet_t(103);
    pub const CURAND_SCRAMBLED_DIRECTION_VECTORS_64_JOEKUO6: curandDirectionVectorSet_t = curandDirectionVectorSet_t(104);
}

impl curandError_t {
    pub const r#VERSION_MISMATCH: curandError_t = curandError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(100)
    });
    pub const r#NOT_INITIALIZED: curandError_t = curandError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(101)
    });
    pub const r#ALLOCATION_FAILED: curandError_t = curandError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(102)
    });
    pub const r#TYPE_ERROR: curandError_t = curandError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(103)
    });
    pub const r#OUT_OF_RANGE: curandError_t = curandError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(104)
    });
    pub const r#LENGTH_NOT_MULTIPLE: curandError_t = curandError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(105)
    });
    pub const r#DOUBLE_PRECISION_REQUIRED: curandError_t = curandError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(106)
    });
    pub const r#LAUNCH_FAILURE: curandError_t = curandError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(201)
    });
    pub const r#PREEXISTING_FAILURE: curandError_t = curandError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(202)
    });
    pub const r#INITIALIZATION_FAILED: curandError_t = curandError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(203)
    });
    pub const r#ARCH_MISMATCH: curandError_t = curandError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(204)
    });
    pub const r#INTERNAL_ERROR: curandError_t = curandError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(999)
    });
    pub const r#NOT_IMPLEMENTED: curandError_t = curandError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1000)
    });
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct curandError_t(pub ::core::num::NonZeroU32);

pub trait curandStatus_tConsts {
    const SUCCESS: curandStatus_t = curandStatus_t::Ok(());
    const ERROR_VERSION_MISMATCH: curandStatus_t = curandStatus_t::Err(
        curandError_t::r#VERSION_MISMATCH,
    );
    const ERROR_NOT_INITIALIZED: curandStatus_t = curandStatus_t::Err(
        curandError_t::r#NOT_INITIALIZED,
    );
    const ERROR_ALLOCATION_FAILED: curandStatus_t = curandStatus_t::Err(
        curandError_t::r#ALLOCATION_FAILED,
    );
    const ERROR_TYPE_ERROR: curandStatus_t = curandStatus_t::Err(
        curandError_t::r#TYPE_ERROR,
    );
    const ERROR_OUT_OF_RANGE: curandStatus_t = curandStatus_t::Err(
        curandError_t::r#OUT_OF_RANGE,
    );
    const ERROR_LENGTH_NOT_MULTIPLE: curandStatus_t = curandStatus_t::Err(
        curandError_t::r#LENGTH_NOT_MULTIPLE,
    );
    const ERROR_DOUBLE_PRECISION_REQUIRED: curandStatus_t = curandStatus_t::Err(
        curandError_t::r#DOUBLE_PRECISION_REQUIRED,
    );
    const ERROR_LAUNCH_FAILURE: curandStatus_t = curandStatus_t::Err(
        curandError_t::r#LAUNCH_FAILURE,
    );
    const ERROR_PREEXISTING_FAILURE: curandStatus_t = curandStatus_t::Err(
        curandError_t::r#PREEXISTING_FAILURE,
    );
    const ERROR_INITIALIZATION_FAILED: curandStatus_t = curandStatus_t::Err(
        curandError_t::r#INITIALIZATION_FAILED,
    );
    const ERROR_ARCH_MISMATCH: curandStatus_t = curandStatus_t::Err(
        curandError_t::r#ARCH_MISMATCH,
    );
    const ERROR_INTERNAL_ERROR: curandStatus_t = curandStatus_t::Err(
        curandError_t::r#INTERNAL_ERROR,
    );
    const ERROR_NOT_IMPLEMENTED: curandStatus_t = curandStatus_t::Err(
        curandError_t::r#NOT_IMPLEMENTED,
    );
}
impl curandStatus_tConsts for curandStatus_t {}

pub type curandStatus_t = ::core::result::Result<(), curandError_t>;
const _: () = {
    let _ = std::mem::transmute::<curandStatus_t, u32>;
};
