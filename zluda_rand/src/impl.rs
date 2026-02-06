use cuda_types::curand::*;
use hiprand_sys::*;
use std::mem;

fn hiprand_status_to_curand(status: hiprandStatus_t) -> curandStatus_t {
    match status {
        hiprandStatus_t::HIPRAND_STATUS_SUCCESS => Ok(()),
        hiprandStatus_t::HIPRAND_STATUS_VERSION_MISMATCH => Err(curandError_t::VERSION_MISMATCH),
        hiprandStatus_t::HIPRAND_STATUS_NOT_INITIALIZED => Err(curandError_t::NOT_INITIALIZED),
        hiprandStatus_t::HIPRAND_STATUS_ALLOCATION_FAILED => Err(curandError_t::ALLOCATION_FAILED),
        hiprandStatus_t::HIPRAND_STATUS_TYPE_ERROR => Err(curandError_t::TYPE_ERROR),
        hiprandStatus_t::HIPRAND_STATUS_OUT_OF_RANGE => Err(curandError_t::OUT_OF_RANGE),
        hiprandStatus_t::HIPRAND_STATUS_LENGTH_NOT_MULTIPLE => Err(curandError_t::LENGTH_NOT_MULTIPLE),
        hiprandStatus_t::HIPRAND_STATUS_DOUBLE_PRECISION_REQUIRED => Err(curandError_t::DOUBLE_PRECISION_REQUIRED),
        hiprandStatus_t::HIPRAND_STATUS_LAUNCH_FAILURE => Err(curandError_t::LAUNCH_FAILURE),
        hiprandStatus_t::HIPRAND_STATUS_PREEXISTING_FAILURE => Err(curandError_t::PREEXISTING_FAILURE),
        hiprandStatus_t::HIPRAND_STATUS_INITIALIZATION_FAILED => Err(curandError_t::INITIALIZATION_FAILED),
        hiprandStatus_t::HIPRAND_STATUS_ARCH_MISMATCH => Err(curandError_t::ARCH_MISMATCH),
        hiprandStatus_t::HIPRAND_STATUS_INTERNAL_ERROR => Err(curandError_t::INTERNAL_ERROR),
        hiprandStatus_t::HIPRAND_STATUS_NOT_IMPLEMENTED => Err(curandError_t::NOT_IMPLEMENTED),
        _ => Err(curandError_t::INTERNAL_ERROR),
    }
}

fn curand_rng_type_to_hiprand(rng_type: curandRngType_t) -> hiprandRngType_t {
    match rng_type {
        curandRngType_t::CURAND_RNG_TEST => hiprandRngType_t::HIPRAND_RNG_TEST,
        curandRngType_t::CURAND_RNG_PSEUDO_DEFAULT => hiprandRngType_t::HIPRAND_RNG_PSEUDO_DEFAULT,
        curandRngType_t::CURAND_RNG_PSEUDO_XORWOW => hiprandRngType_t::HIPRAND_RNG_PSEUDO_XORWOW,
        curandRngType_t::CURAND_RNG_PSEUDO_MRG32K3A => hiprandRngType_t::HIPRAND_RNG_PSEUDO_MRG32K3A,
        curandRngType_t::CURAND_RNG_PSEUDO_MTGP32 => hiprandRngType_t::HIPRAND_RNG_PSEUDO_MTGP32,
        curandRngType_t::CURAND_RNG_PSEUDO_MT19937 => hiprandRngType_t::HIPRAND_RNG_PSEUDO_MT19937,
        curandRngType_t::CURAND_RNG_PSEUDO_PHILOX4_32_10 => hiprandRngType_t::HIPRAND_RNG_PSEUDO_PHILOX4_32_10,
        curandRngType_t::CURAND_RNG_QUASI_DEFAULT => hiprandRngType_t::HIPRAND_RNG_QUASI_DEFAULT,
        curandRngType_t::CURAND_RNG_QUASI_SOBOL32 => hiprandRngType_t::HIPRAND_RNG_QUASI_SOBOL32,
        curandRngType_t::CURAND_RNG_QUASI_SCRAMBLED_SOBOL32 => hiprandRngType_t::HIPRAND_RNG_QUASI_SCRAMBLED_SOBOL32,
        curandRngType_t::CURAND_RNG_QUASI_SOBOL64 => hiprandRngType_t::HIPRAND_RNG_QUASI_SOBOL64,
        curandRngType_t::CURAND_RNG_QUASI_SCRAMBLED_SOBOL64 => hiprandRngType_t::HIPRAND_RNG_QUASI_SCRAMBLED_SOBOL64,
        _ => hiprandRngType_t::HIPRAND_RNG_PSEUDO_DEFAULT,
    }
}

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> curandStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> curandStatus_t {
    Err(curandError_t::NOT_IMPLEMENTED)
}

pub(crate) unsafe fn create_generator(
    generator: *mut curandGenerator_t,
    rng_type: curandRngType_t,
) -> curandStatus_t {
    if generator.is_null() {
        return Err(curandError_t::NOT_INITIALIZED);
    }
    let hip_rng_type = curand_rng_type_to_hiprand(rng_type);
    let mut hip_generator: hiprandGenerator_t = std::ptr::null_mut();
    hiprand_status_to_curand(hiprandCreateGenerator(
        &mut hip_generator,
        hip_rng_type,
    ))?;
    *generator = hip_generator as curandGenerator_t;
    Ok(())
}

pub(crate) unsafe fn create_generator_host(
    generator: *mut curandGenerator_t,
    rng_type: curandRngType_t,
) -> curandStatus_t {
    if generator.is_null() {
        return Err(curandError_t::NOT_INITIALIZED);
    }
    let hip_rng_type = curand_rng_type_to_hiprand(rng_type);
    let mut hip_generator: hiprandGenerator_t = std::ptr::null_mut();
    hiprand_status_to_curand(hiprandCreateGeneratorHost(
        &mut hip_generator,
        hip_rng_type,
    ))?;
    *generator = hip_generator as curandGenerator_t;
    Ok(())
}

pub(crate) unsafe fn destroy_generator(
    generator: curandGenerator_t,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandDestroyGenerator(
        generator as hiprandGenerator_t,
    ))
}

pub(crate) unsafe fn generate(
    generator: curandGenerator_t,
    output_ptr: *mut ::core::ffi::c_uint,
    num: usize,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGenerate(
        generator as hiprandGenerator_t,
        output_ptr,
        num,
    ))
}

pub(crate) unsafe fn generate_long_long(
    generator: curandGenerator_t,
    output_ptr: *mut ::core::ffi::c_ulonglong,
    num: usize,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGenerateLongLong(
        generator as hiprandGenerator_t,
        output_ptr,
        num,
    ))
}

pub(crate) unsafe fn generate_uniform(
    generator: curandGenerator_t,
    output_ptr: *mut f32,
    num: usize,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGenerateUniform(
        generator as hiprandGenerator_t,
        output_ptr,
        num,
    ))
}

pub(crate) unsafe fn generate_uniform_double(
    generator: curandGenerator_t,
    output_ptr: *mut f64,
    num: usize,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGenerateUniformDouble(
        generator as hiprandGenerator_t,
        output_ptr,
        num,
    ))
}

pub(crate) unsafe fn generate_normal(
    generator: curandGenerator_t,
    output_ptr: *mut f32,
    n: usize,
    mean: f32,
    stddev: f32,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGenerateNormal(
        generator as hiprandGenerator_t,
        output_ptr,
        n,
        mean,
        stddev,
    ))
}

pub(crate) unsafe fn generate_normal_double(
    generator: curandGenerator_t,
    output_ptr: *mut f64,
    n: usize,
    mean: f64,
    stddev: f64,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGenerateNormalDouble(
        generator as hiprandGenerator_t,
        output_ptr,
        n,
        mean,
        stddev,
    ))
}

pub(crate) unsafe fn generate_log_normal(
    generator: curandGenerator_t,
    output_ptr: *mut f32,
    n: usize,
    mean: f32,
    stddev: f32,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGenerateLogNormal(
        generator as hiprandGenerator_t,
        output_ptr,
        n,
        mean,
        stddev,
    ))
}

pub(crate) unsafe fn generate_log_normal_double(
    generator: curandGenerator_t,
    output_ptr: *mut f64,
    n: usize,
    mean: f64,
    stddev: f64,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGenerateLogNormalDouble(
        generator as hiprandGenerator_t,
        output_ptr,
        n,
        mean,
        stddev,
    ))
}

pub(crate) unsafe fn generate_poisson(
    generator: curandGenerator_t,
    output_ptr: *mut ::core::ffi::c_uint,
    n: usize,
    lambda: f64,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGeneratePoisson(
        generator as hiprandGenerator_t,
        output_ptr,
        n,
        lambda,
    ))
}

pub(crate) unsafe fn generate_seeds(
    generator: curandGenerator_t,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGenerateSeeds(
        generator as hiprandGenerator_t,
    ))
}

pub(crate) unsafe fn set_stream(
    generator: curandGenerator_t,
    stream: cudaStream_t,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandSetStream(
        generator as hiprandGenerator_t,
        mem::transmute(stream),
    ))
}

pub(crate) unsafe fn set_pseudo_random_generator_seed(
    generator: curandGenerator_t,
    seed: ::core::ffi::c_ulonglong,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandSetPseudoRandomGeneratorSeed(
        generator as hiprandGenerator_t,
        seed,
    ))
}

pub(crate) unsafe fn set_generator_offset(
    generator: curandGenerator_t,
    offset: ::core::ffi::c_ulonglong,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandSetGeneratorOffset(
        generator as hiprandGenerator_t,
        offset,
    ))
}

pub(crate) unsafe fn set_generator_ordering(
    generator: curandGenerator_t,
    order: curandOrdering_t,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandSetGeneratorOrdering(
        generator as hiprandGenerator_t,
        hiprandOrdering_t(order.0),
    ))
}

pub(crate) unsafe fn set_quasi_random_generator_dimensions(
    generator: curandGenerator_t,
    num_dimensions: ::core::ffi::c_uint,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandSetQuasiRandomGeneratorDimensions(
        generator as hiprandGenerator_t,
        num_dimensions,
    ))
}

pub(crate) unsafe fn get_version(
    version: *mut ::core::ffi::c_int,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGetVersion(version))
}

pub(crate) unsafe fn create_poisson_distribution(
    lambda: f64,
    discrete_distribution: *mut curandDiscreteDistribution_t,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandCreatePoissonDistribution(
        lambda,
        discrete_distribution as *mut hiprandDiscreteDistribution_t,
    ))
}

pub(crate) unsafe fn destroy_distribution(
    discrete_distribution: curandDiscreteDistribution_t,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandDestroyDistribution(
        discrete_distribution as hiprandDiscreteDistribution_t,
    ))
}

pub(crate) unsafe fn get_direction_vectors32(
    vectors: *mut *mut curandDirectionVectors32_t,
    set: curandDirectionVectorSet_t,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGetDirectionVectors32(
        vectors as *mut *mut hiprandDirectionVectors32_t,
        hiprandDirectionVectorSet_t(set.0),
    ))
}

pub(crate) unsafe fn get_direction_vectors64(
    vectors: *mut *mut curandDirectionVectors64_t,
    set: curandDirectionVectorSet_t,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGetDirectionVectors64(
        vectors as *mut *mut hiprandDirectionVectors64_t,
        hiprandDirectionVectorSet_t(set.0),
    ))
}

pub(crate) unsafe fn get_scramble_constants32(
    constants: *mut *const ::core::ffi::c_uint,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGetScrambleConstants32(constants))
}

pub(crate) unsafe fn get_scramble_constants64(
    constants: *mut *const ::core::ffi::c_ulonglong,
) -> curandStatus_t {
    hiprand_status_to_curand(hiprandGetScrambleConstants64(constants))
}
