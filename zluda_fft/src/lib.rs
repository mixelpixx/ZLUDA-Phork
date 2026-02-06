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
                cuda_macros::cufft_normalize_fn!( crate::r#impl::$fn_name ) ( $( $arg_id ),* )
            }
        )*
    };
}

cuda_macros::cufft_function_declarations!(
    unimplemented,
    implemented
        <= [
            cufftCreate,
            cufftDestroy,
            cufftEstimate1d,
            cufftEstimate2d,
            cufftEstimate3d,
            cufftEstimateMany,
            cufftExecC2C,
            cufftExecC2R,
            cufftExecD2Z,
            cufftExecR2C,
            cufftExecZ2D,
            cufftExecZ2Z,
            cufftGetProperty,
            cufftGetSize,
            cufftGetSize1d,
            cufftGetSize2d,
            cufftGetSize3d,
            cufftGetSizeMany,
            cufftGetSizeMany64,
            cufftGetVersion,
            cufftMakePlan1d,
            cufftMakePlan2d,
            cufftMakePlan3d,
            cufftMakePlanMany,
            cufftMakePlanMany64,
            cufftPlan1d,
            cufftPlan2d,
            cufftPlan3d,
            cufftPlanMany,
            cufftSetAutoAllocation,
            cufftSetStream,
            cufftSetWorkArea
        ]
);
