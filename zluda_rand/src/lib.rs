mod r#impl;

macro_rules! unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            #[allow(unused_variables)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                crate::r#impl::unimplemented()
            }
        )*
    };
}

macro_rules! implemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                cuda_macros::curand_normalize_fn!( crate::r#impl::$fn_name ) ( $( $arg_id ),* )
            }
        )*
    };
}

cuda_macros::curand_function_declarations!(
    unimplemented,
    implemented
        <= [
            curandCreateGenerator,
            curandCreateGeneratorHost,
            curandCreatePoissonDistribution,
            curandDestroyDistribution,
            curandDestroyGenerator,
            curandGenerate,
            curandGenerateLongLong,
            curandGenerateLogNormal,
            curandGenerateLogNormalDouble,
            curandGenerateNormal,
            curandGenerateNormalDouble,
            curandGeneratePoisson,
            curandGenerateSeeds,
            curandGenerateUniform,
            curandGenerateUniformDouble,
            curandGetDirectionVectors32,
            curandGetDirectionVectors64,
            curandGetScrambleConstants32,
            curandGetScrambleConstants64,
            curandGetVersion,
            curandSetGeneratorOffset,
            curandSetGeneratorOrdering,
            curandSetPseudoRandomGeneratorSeed,
            curandSetQuasiRandomGeneratorDimensions,
            curandSetStream
        ]
);
