#![allow(non_camel_case_types)]

use hip_runtime_sys::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hiprandGenerator_st {
    _unused: [u8; 0],
}
pub type hiprandGenerator_t = *mut hiprandGenerator_st;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hiprandDiscreteDistribution_st {
    _unused: [u8; 0],
}
pub type hiprandDiscreteDistribution_t = *mut hiprandDiscreteDistribution_st;

pub type hiprandDirectionVectors32_t = [::core::ffi::c_uint; 32usize];
pub type hiprandDirectionVectors64_t = [::core::ffi::c_ulonglong; 64usize];

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hiprandStatus_t(pub ::core::ffi::c_uint);

impl hiprandStatus_t {
    pub const HIPRAND_STATUS_SUCCESS: hiprandStatus_t = hiprandStatus_t(0);
    pub const HIPRAND_STATUS_VERSION_MISMATCH: hiprandStatus_t = hiprandStatus_t(100);
    pub const HIPRAND_STATUS_NOT_INITIALIZED: hiprandStatus_t = hiprandStatus_t(101);
    pub const HIPRAND_STATUS_ALLOCATION_FAILED: hiprandStatus_t = hiprandStatus_t(102);
    pub const HIPRAND_STATUS_TYPE_ERROR: hiprandStatus_t = hiprandStatus_t(103);
    pub const HIPRAND_STATUS_OUT_OF_RANGE: hiprandStatus_t = hiprandStatus_t(104);
    pub const HIPRAND_STATUS_LENGTH_NOT_MULTIPLE: hiprandStatus_t = hiprandStatus_t(105);
    pub const HIPRAND_STATUS_DOUBLE_PRECISION_REQUIRED: hiprandStatus_t = hiprandStatus_t(106);
    pub const HIPRAND_STATUS_LAUNCH_FAILURE: hiprandStatus_t = hiprandStatus_t(201);
    pub const HIPRAND_STATUS_PREEXISTING_FAILURE: hiprandStatus_t = hiprandStatus_t(202);
    pub const HIPRAND_STATUS_INITIALIZATION_FAILED: hiprandStatus_t = hiprandStatus_t(203);
    pub const HIPRAND_STATUS_ARCH_MISMATCH: hiprandStatus_t = hiprandStatus_t(204);
    pub const HIPRAND_STATUS_INTERNAL_ERROR: hiprandStatus_t = hiprandStatus_t(999);
    pub const HIPRAND_STATUS_NOT_IMPLEMENTED: hiprandStatus_t = hiprandStatus_t(1000);
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hiprandRngType_t(pub ::core::ffi::c_uint);

impl hiprandRngType_t {
    pub const HIPRAND_RNG_TEST: hiprandRngType_t = hiprandRngType_t(0);
    pub const HIPRAND_RNG_PSEUDO_DEFAULT: hiprandRngType_t = hiprandRngType_t(400);
    pub const HIPRAND_RNG_PSEUDO_XORWOW: hiprandRngType_t = hiprandRngType_t(401);
    pub const HIPRAND_RNG_PSEUDO_MRG32K3A: hiprandRngType_t = hiprandRngType_t(402);
    pub const HIPRAND_RNG_PSEUDO_MTGP32: hiprandRngType_t = hiprandRngType_t(403);
    pub const HIPRAND_RNG_PSEUDO_MT19937: hiprandRngType_t = hiprandRngType_t(404);
    pub const HIPRAND_RNG_PSEUDO_PHILOX4_32_10: hiprandRngType_t = hiprandRngType_t(405);
    pub const HIPRAND_RNG_QUASI_DEFAULT: hiprandRngType_t = hiprandRngType_t(500);
    pub const HIPRAND_RNG_QUASI_SOBOL32: hiprandRngType_t = hiprandRngType_t(501);
    pub const HIPRAND_RNG_QUASI_SCRAMBLED_SOBOL32: hiprandRngType_t = hiprandRngType_t(502);
    pub const HIPRAND_RNG_QUASI_SOBOL64: hiprandRngType_t = hiprandRngType_t(503);
    pub const HIPRAND_RNG_QUASI_SCRAMBLED_SOBOL64: hiprandRngType_t = hiprandRngType_t(504);
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hiprandOrdering_t(pub ::core::ffi::c_uint);

impl hiprandOrdering_t {
    pub const HIPRAND_ORDERING_PSEUDO_BEST: hiprandOrdering_t = hiprandOrdering_t(100);
    pub const HIPRAND_ORDERING_PSEUDO_DEFAULT: hiprandOrdering_t = hiprandOrdering_t(101);
    pub const HIPRAND_ORDERING_PSEUDO_SEEDED: hiprandOrdering_t = hiprandOrdering_t(102);
    pub const HIPRAND_ORDERING_PSEUDO_LEGACY: hiprandOrdering_t = hiprandOrdering_t(103);
    pub const HIPRAND_ORDERING_PSEUDO_DYNAMIC: hiprandOrdering_t = hiprandOrdering_t(104);
    pub const HIPRAND_ORDERING_QUASI_DEFAULT: hiprandOrdering_t = hiprandOrdering_t(201);
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hiprandDirectionVectorSet_t(pub ::core::ffi::c_uint);

impl hiprandDirectionVectorSet_t {
    pub const HIPRAND_DIRECTION_VECTORS_32_JOEKUO6: hiprandDirectionVectorSet_t = hiprandDirectionVectorSet_t(101);
    pub const HIPRAND_SCRAMBLED_DIRECTION_VECTORS_32_JOEKUO6: hiprandDirectionVectorSet_t = hiprandDirectionVectorSet_t(102);
    pub const HIPRAND_DIRECTION_VECTORS_64_JOEKUO6: hiprandDirectionVectorSet_t = hiprandDirectionVectorSet_t(103);
    pub const HIPRAND_SCRAMBLED_DIRECTION_VECTORS_64_JOEKUO6: hiprandDirectionVectorSet_t = hiprandDirectionVectorSet_t(104);
}

#[cfg_attr(windows, link(name = "hiprand", kind = "raw-dylib"))]
extern "C" {
    pub fn hiprandCreateGenerator(
        generator: *mut hiprandGenerator_t,
        rng_type: hiprandRngType_t,
    ) -> hiprandStatus_t;

    pub fn hiprandCreateGeneratorHost(
        generator: *mut hiprandGenerator_t,
        rng_type: hiprandRngType_t,
    ) -> hiprandStatus_t;

    pub fn hiprandDestroyGenerator(
        generator: hiprandGenerator_t,
    ) -> hiprandStatus_t;

    pub fn hiprandGenerate(
        generator: hiprandGenerator_t,
        output_data: *mut ::core::ffi::c_uint,
        n: usize,
    ) -> hiprandStatus_t;

    pub fn hiprandGenerateLongLong(
        generator: hiprandGenerator_t,
        output_data: *mut ::core::ffi::c_ulonglong,
        n: usize,
    ) -> hiprandStatus_t;

    pub fn hiprandGenerateUniform(
        generator: hiprandGenerator_t,
        output_data: *mut f32,
        n: usize,
    ) -> hiprandStatus_t;

    pub fn hiprandGenerateUniformDouble(
        generator: hiprandGenerator_t,
        output_data: *mut f64,
        n: usize,
    ) -> hiprandStatus_t;

    pub fn hiprandGenerateNormal(
        generator: hiprandGenerator_t,
        output_data: *mut f32,
        n: usize,
        mean: f32,
        stddev: f32,
    ) -> hiprandStatus_t;

    pub fn hiprandGenerateNormalDouble(
        generator: hiprandGenerator_t,
        output_data: *mut f64,
        n: usize,
        mean: f64,
        stddev: f64,
    ) -> hiprandStatus_t;

    pub fn hiprandGenerateLogNormal(
        generator: hiprandGenerator_t,
        output_data: *mut f32,
        n: usize,
        mean: f32,
        stddev: f32,
    ) -> hiprandStatus_t;

    pub fn hiprandGenerateLogNormalDouble(
        generator: hiprandGenerator_t,
        output_data: *mut f64,
        n: usize,
        mean: f64,
        stddev: f64,
    ) -> hiprandStatus_t;

    pub fn hiprandGeneratePoisson(
        generator: hiprandGenerator_t,
        output_data: *mut ::core::ffi::c_uint,
        n: usize,
        lambda: f64,
    ) -> hiprandStatus_t;

    pub fn hiprandGenerateSeeds(
        generator: hiprandGenerator_t,
    ) -> hiprandStatus_t;

    pub fn hiprandSetStream(
        generator: hiprandGenerator_t,
        stream: hipStream_t,
    ) -> hiprandStatus_t;

    pub fn hiprandSetPseudoRandomGeneratorSeed(
        generator: hiprandGenerator_t,
        seed: ::core::ffi::c_ulonglong,
    ) -> hiprandStatus_t;

    pub fn hiprandSetGeneratorOffset(
        generator: hiprandGenerator_t,
        offset: ::core::ffi::c_ulonglong,
    ) -> hiprandStatus_t;

    pub fn hiprandSetGeneratorOrdering(
        generator: hiprandGenerator_t,
        order: hiprandOrdering_t,
    ) -> hiprandStatus_t;

    pub fn hiprandSetQuasiRandomGeneratorDimensions(
        generator: hiprandGenerator_t,
        num_dimensions: ::core::ffi::c_uint,
    ) -> hiprandStatus_t;

    pub fn hiprandGetVersion(
        version: *mut ::core::ffi::c_int,
    ) -> hiprandStatus_t;

    pub fn hiprandCreatePoissonDistribution(
        lambda: f64,
        discrete_distribution: *mut hiprandDiscreteDistribution_t,
    ) -> hiprandStatus_t;

    pub fn hiprandDestroyDistribution(
        discrete_distribution: hiprandDiscreteDistribution_t,
    ) -> hiprandStatus_t;

    pub fn hiprandGetDirectionVectors32(
        vectors: *mut *mut hiprandDirectionVectors32_t,
        set: hiprandDirectionVectorSet_t,
    ) -> hiprandStatus_t;

    pub fn hiprandGetDirectionVectors64(
        vectors: *mut *mut hiprandDirectionVectors64_t,
        set: hiprandDirectionVectorSet_t,
    ) -> hiprandStatus_t;

    pub fn hiprandGetScrambleConstants32(
        constants: *mut *const ::core::ffi::c_uint,
    ) -> hiprandStatus_t;

    pub fn hiprandGetScrambleConstants64(
        constants: *mut *const ::core::ffi::c_ulonglong,
    ) -> hiprandStatus_t;
}
