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
                cuda_macros::cusparse_normalize_fn!( crate::r#impl::$fn_name ) ( $( $arg_id ),* )
            }
        )*
    };
}

cuda_macros::cusparse_function_declarations!(
    unimplemented,
    implemented
        <= [
            cusparseCreate,
            cusparseCreateCsr,
            cusparseCreateDnMat,
            cusparseCreateDnVec,
            cusparseCreateMatDescr,
            cusparseCreateSpVec,
            cusparseDestroy,
            cusparseDestroyDnMat,
            cusparseDestroyDnVec,
            cusparseDestroyMatDescr,
            cusparseDestroySpMat,
            cusparseDestroySpVec,
            cusparseGetErrorName,
            cusparseGetErrorString,
            cusparseGetMatDiagType,
            cusparseGetMatFillMode,
            cusparseGetMatIndexBase,
            cusparseGetMatType,
            cusparseGetPointerMode,
            cusparseGetStream,
            cusparseSetMatDiagType,
            cusparseSetMatFillMode,
            cusparseSetMatIndexBase,
            cusparseSetMatType,
            cusparseSetPointerMode,
            cusparseSetStream,
            cusparseSpMM,
            cusparseSpMM_bufferSize,
            cusparseSpMV,
            cusparseSpMV_bufferSize
        ]
);
