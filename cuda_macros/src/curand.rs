// Generated manually for ZLUDA cuRAND support
#![allow(warnings)]
extern "system" {
    fn curandCreateGenerator(
        generator: *mut cuda_types::curand::curandGenerator_t,
        rng_type: cuda_types::curand::curandRngType_t,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandCreateGeneratorHost(
        generator: *mut cuda_types::curand::curandGenerator_t,
        rng_type: cuda_types::curand::curandRngType_t,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandDestroyGenerator(
        generator: cuda_types::curand::curandGenerator_t,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGenerate(
        generator: cuda_types::curand::curandGenerator_t,
        outputPtr: *mut ::core::ffi::c_uint,
        num: usize,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGenerateLongLong(
        generator: cuda_types::curand::curandGenerator_t,
        outputPtr: *mut ::core::ffi::c_ulonglong,
        num: usize,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGenerateUniform(
        generator: cuda_types::curand::curandGenerator_t,
        outputPtr: *mut f32,
        num: usize,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGenerateUniformDouble(
        generator: cuda_types::curand::curandGenerator_t,
        outputPtr: *mut f64,
        num: usize,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGenerateNormal(
        generator: cuda_types::curand::curandGenerator_t,
        outputPtr: *mut f32,
        n: usize,
        mean: f32,
        stddev: f32,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGenerateNormalDouble(
        generator: cuda_types::curand::curandGenerator_t,
        outputPtr: *mut f64,
        n: usize,
        mean: f64,
        stddev: f64,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGenerateLogNormal(
        generator: cuda_types::curand::curandGenerator_t,
        outputPtr: *mut f32,
        n: usize,
        mean: f32,
        stddev: f32,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGenerateLogNormalDouble(
        generator: cuda_types::curand::curandGenerator_t,
        outputPtr: *mut f64,
        n: usize,
        mean: f64,
        stddev: f64,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGeneratePoisson(
        generator: cuda_types::curand::curandGenerator_t,
        outputPtr: *mut ::core::ffi::c_uint,
        n: usize,
        lambda: f64,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGenerateSeeds(
        generator: cuda_types::curand::curandGenerator_t,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandSetStream(
        generator: cuda_types::curand::curandGenerator_t,
        stream: cuda_types::curand::cudaStream_t,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandSetPseudoRandomGeneratorSeed(
        generator: cuda_types::curand::curandGenerator_t,
        seed: ::core::ffi::c_ulonglong,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandSetGeneratorOffset(
        generator: cuda_types::curand::curandGenerator_t,
        offset: ::core::ffi::c_ulonglong,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandSetGeneratorOrdering(
        generator: cuda_types::curand::curandGenerator_t,
        order: cuda_types::curand::curandOrdering_t,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandSetQuasiRandomGeneratorDimensions(
        generator: cuda_types::curand::curandGenerator_t,
        num_dimensions: ::core::ffi::c_uint,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGetVersion(
        version: *mut ::core::ffi::c_int,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandCreatePoissonDistribution(
        lambda: f64,
        discrete_distribution: *mut cuda_types::curand::curandDiscreteDistribution_t,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandDestroyDistribution(
        discrete_distribution: cuda_types::curand::curandDiscreteDistribution_t,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGetDirectionVectors32(
        vectors: *mut *mut cuda_types::curand::curandDirectionVectors32_t,
        set: cuda_types::curand::curandDirectionVectorSet_t,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGetDirectionVectors64(
        vectors: *mut *mut cuda_types::curand::curandDirectionVectors64_t,
        set: cuda_types::curand::curandDirectionVectorSet_t,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGetScrambleConstants32(
        constants: *mut *const ::core::ffi::c_uint,
    ) -> cuda_types::curand::curandStatus_t;
    fn curandGetScrambleConstants64(
        constants: *mut *const ::core::ffi::c_ulonglong,
    ) -> cuda_types::curand::curandStatus_t;
}
