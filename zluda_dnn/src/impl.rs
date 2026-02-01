use cuda_types::cudnn9::*;
use hip_runtime_sys::*;
use miopen_sys::*;
use rustc_hash::FxHashMap;
use std::{collections::VecDeque, mem, ptr, sync::Mutex};
use zluda_common::{from_cuda_object, ZludaObject};

pub(crate) struct Context {
    base: miopenHandle_t,
    search_workspace: Mutex<ContextCache>,
}

from_cuda_object!(Context);

impl Context {
    fn new(base: miopenHandle_t) -> Self {
        Self {
            base,
            search_workspace: Mutex::new(ContextCache {
                beta_buffer_cache: TemporaryBufferAllocator::new(),
                algo_cache: FxHashMap::default(),
            }),
        }
    }
}

impl ZludaObject for Context {
    const COOKIE: usize = 0x575f48767c76029a;

    type Error = cudnnError_t;
    type CudaHandle = cudnnHandle_t;

    fn drop_checked(&mut self) -> Result<(), cudnnError_t> {
        let result1 = unsafe { miopenDestroy(self.base) }.map_err(Into::into);
        let result2 = if let Ok(search_workspace) = self.search_workspace.get_mut() {
            search_workspace
                .drop_checked()
                .map_err(|_| cudnnError_t::INTERNAL_ERROR)
        } else {
            Ok(())
        };
        result1.and(result2)
    }
}

struct ContextCache {
    beta_buffer_cache: TemporaryBufferAllocator,
    algo_cache: FxHashMap<ConvolutionOpCacheKey, miopenConvAlgoPerf_t>,
}

impl ContextCache {
    fn drop_checked(&mut self) -> Result<(), hipErrorCode_t> {
        let mut err = None;
        while let Some(mut buffer) = self.beta_buffer_cache.buffers.pop_front() {
            if let Err(e) = buffer.drop_checked() {
                err = Some(e);
            }
        }
        err.map_or(Ok(()), Err)
    }
}

unsafe impl Send for ContextCache {}
unsafe impl Sync for ContextCache {}

// At various point of the execution we need temporary buffers to hold device
// data to support various MIOpen workarounds.
// Normally, their use does not overlap, but it's still possible.
struct TemporaryBufferAllocator {
    buffers: VecDeque<BetaBuffer>,
}

impl TemporaryBufferAllocator {
    fn new() -> Self {
        Self {
            buffers: VecDeque::new(),
        }
    }

    fn get_or_allocate(&mut self, size: usize) -> Result<&BetaBuffer, hipErrorCode_t> {
        let mut buffer = self.scavange_or_allocate_buffer(size)?;
        if let Some(event) = buffer.free.take() {
            unsafe { hipEventDestroy(event) }.ok();
        }
        self.buffers.push_front(buffer);
        Ok(&self.buffers.front().unwrap())
    }

    fn scavange_or_allocate_buffer(&mut self, size: usize) -> Result<BetaBuffer, hipErrorCode_t> {
        let mut result = None;
        let mut err = None;
        let mut update_buffer_if_biggest = |buffer: BetaBuffer| {
            if result
                .as_ref()
                .map(|b: &BetaBuffer| buffer.size > b.size)
                .unwrap_or(true)
            {
                result = Some(buffer);
            }
        };
        while let Some(buffer) = self.buffers.pop_front() {
            match buffer.free {
                None => {
                    update_buffer_if_biggest(buffer);
                }
                Some(event) => match unsafe { hipEventQuery(event) } {
                    hipError_t::ErrorNotReady => {
                        self.buffers.push_front(buffer);
                        break;
                    }
                    hipError_t::Success => {
                        update_buffer_if_biggest(buffer);
                    }
                    Err(err_code) => {
                        err = Some(err_code);
                        break;
                    }
                },
            }
        }
        match err {
            Some(err_code) => Err(err_code),
            None => match result {
                Some(buffer) if buffer.size >= size => Ok(buffer),
                _ => Ok(BetaBuffer::new(size)?),
            },
        }
    }

    fn with_async_buffer(
        &mut self,
        size: usize,
        stream: hipStream_t,
        fn_: impl FnOnce(*mut ::std::os::raw::c_void) -> Result<(), miopenError_t>,
    ) -> Result<(), miopenError_t> {
        let mut buffer = self
            .scavange_or_allocate_buffer(size)
            .map_err(|_| miopenError_t::InternalError)?;
        if buffer.free.is_none() {
            let mut event = std::ptr::null_mut();
            unsafe {
                hipEventCreateWithFlags(
                    &mut event,
                    hipEventDisableTiming | hipEventDisableSystemFence,
                )
                .map_err(|_| miopenError_t::InternalError)?
            };
            buffer.free = Some(event);
        }
        fn_(buffer.data)?;
        unsafe { hipEventRecord(buffer.free.unwrap(), stream) }
            .map_err(|_| miopenError_t::InternalError)?;
        self.buffers.push_back(buffer);
        Ok(())
    }
}

struct BetaBuffer {
    size: usize,
    data: *mut ::std::os::raw::c_void,
    free: Option<hipEvent_t>,
}

impl BetaBuffer {
    fn new(size: usize) -> Result<Self, hipErrorCode_t> {
        let mut data = std::ptr::null_mut();
        unsafe { hipMalloc(&mut data, size)? };
        Ok(Self {
            size,
            data,
            free: None,
        })
    }

    fn drop_checked(&mut self) -> Result<(), hipErrorCode_t> {
        let result1 = unsafe { hipFree(self.data) };
        let result2 = self
            .free
            .map(|e| unsafe { hipEventDestroy(e) })
            .unwrap_or(Ok(()));
        result1.and(result2)
    }
}

impl Drop for BetaBuffer {
    fn drop(&mut self) {
        self.drop_checked().ok();
    }
}

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cudnnStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cudnnStatus_t {
    cudnnStatus_t::ERROR_NOT_SUPPORTED
}

pub(crate) fn get_version() -> usize {
    // Return cuDNN version 9.13.1
    cuda_types::cudnn9::CUDNN_VERSION as usize
}
pub(crate) fn get_max_device_version() -> usize {
    // Return maximum supported cuDNN version (same as library version)
    cuda_types::cudnn9::CUDNN_VERSION as usize
}
pub(crate) fn get_cudart_version() -> usize {
    // Return CUDA runtime version matching the driver (CUDA 13.0)
    cuda_types::cuda::CUDA_VERSION as usize
}
pub(crate) fn get_last_error_string(message: *mut ::core::ffi::c_char, max_size: usize) -> () {
    // Write empty string - we don't track last error currently
    if !message.is_null() && max_size > 0 {
        unsafe {
            *message = 0;
        }
    }
}

pub(crate) unsafe fn create(handle: &mut cudnnHandle_t) -> miopenStatus_t {
    let mut miopen_handle = mem::zeroed();
    miopenCreate(&mut miopen_handle)?;
    *handle = Context::new(miopen_handle).wrap();
    Ok(())
}

pub(crate) fn destroy(handle: cudnnHandle_t) -> cudnnStatus_t {
    zluda_common::drop_checked::<Context>(handle)?;
    Ok(())
}

pub(crate) fn create_tensor_descriptor(
    tensor_desc: &mut miopenTensorDescriptor_t,
) -> miopenStatus_t {
    unsafe { miopenCreateTensorDescriptor(tensor_desc) }
}

pub(crate) fn create_filter_descriptor(
    filter_desc: &mut miopenTensorDescriptor_t,
) -> miopenStatus_t {
    unsafe { miopenCreateTensorDescriptor(filter_desc) }
}

pub(crate) unsafe fn set_tensor4d_descriptor(
    tensor_desc: miopenTensorDescriptor_t,
    format: miopenTensorLayout_t,
    data_type: miopenDataType_t,
    n: ::std::os::raw::c_int,
    c: ::std::os::raw::c_int,
    h: ::std::os::raw::c_int,
    w: ::std::os::raw::c_int,
) -> miopenStatus_t {
    // Even if the layout is NHWC, miopenSetNdTensorDescriptorWithLayout still expects NCHW order
    let lens = [n, c, h, w];
    miopenSetNdTensorDescriptorWithLayout(
        tensor_desc,
        data_type,
        format,
        lens.as_ptr(),
        lens.len() as i32,
    )
}

pub(crate) unsafe fn set_filter4d_descriptor(
    filter_desc: miopenTensorDescriptor_t,
    data_type: miopenDataType_t,
    format: miopenTensorLayout_t,
    n: ::std::os::raw::c_int,
    c: ::std::os::raw::c_int,
    h: ::std::os::raw::c_int,
    w: ::std::os::raw::c_int,
) -> miopenStatus_t {
    set_tensor4d_descriptor(filter_desc, format, data_type, n, c, h, w)
}

pub(crate) fn create_convolution_descriptor(
    conv_desc: &mut miopenConvolutionDescriptor_t,
) -> miopenStatus_t {
    unsafe { miopenCreateConvolutionDescriptor(conv_desc) }
}

pub(crate) unsafe fn set_convolution2d_descriptor(
    conv_desc: miopenConvolutionDescriptor_t,
    pad_h: ::std::os::raw::c_int,
    pad_w: ::std::os::raw::c_int,
    u: ::std::os::raw::c_int,
    v: ::std::os::raw::c_int,
    dilation_h: ::std::os::raw::c_int,
    dilation_w: ::std::os::raw::c_int,
    mode: miopenConvolutionMode_t,
    _compute_type: miopenDataType_t,
) -> miopenStatus_t {
    miopenInitConvolutionDescriptor(conv_desc, mode, pad_h, pad_w, u, v, dilation_h, dilation_w)
}

pub(crate) unsafe fn set_convolution_math_type(
    _conv_desc: miopenConvolutionDescriptor_t,
    _math_type: cudnnMathType_t,
) -> miopenStatus_t {
    // miopen does not have an equivalent function to set math type
    miopenStatus_t::Success
}

pub(crate) unsafe fn get_convolution_forward_algorithm_v7(
    handle: &Context,
    x_desc: miopenTensorDescriptor_t,
    w_desc: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
    y_desc: miopenTensorDescriptor_t,
    requested_algo_count: ::std::os::raw::c_int,
    returned_algo_count: &mut ::std::os::raw::c_int,
    perf_results: &mut cudnnConvolutionFwdAlgoPerf_t,
) -> Result<(), miopenError_t> {
    if requested_algo_count <= 0 {
        return Err(miopenError_t::BadParm);
    }
    let miopen_result =
        get_or_search_convolution_forward_algorithm(handle, x_desc, w_desc, conv_desc, y_desc)?;
    match miopen_result {
        Some(miopen_result) => {
            *returned_algo_count = 1;
            *perf_results = algo_perf_to_cudnn(miopen_result);
        }
        None => {
            *returned_algo_count = 0;
        }
    }
    Ok(())
}

unsafe fn get_or_search_convolution_forward_algorithm(
    handle: &Context,
    x_desc: miopenTensorDescriptor_t,
    w_desc: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
    y_desc: miopenTensorDescriptor_t,
) -> Result<Option<miopenConvAlgoPerf_t>, miopenError_t> {
    let miopen_result = {
        let mut search_workspace = handle
            .search_workspace
            .lock()
            .map_err(|_| miopenError_t::UnknownError)?;
        let search_workspace = &mut *search_workspace;
        let cache_key = ConvolutionOpCacheKey::new(x_desc, w_desc, conv_desc, y_desc)?;
        let scratchpad = &mut search_workspace.beta_buffer_cache;
        let cache = &mut search_workspace.algo_cache;
        match cache.entry(cache_key) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                Some(occupied_entry.get().clone())
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                search_convolution_forward_algorithm(
                    handle.base,
                    scratchpad,
                    x_desc,
                    w_desc,
                    conv_desc,
                    y_desc,
                )?
                .map(|x| vacant_entry.insert(x).clone())
            }
        }
    };
    Ok(miopen_result)
}

unsafe fn search_convolution_forward_algorithm(
    handle: miopenHandle_t,
    scratchpad: &mut TemporaryBufferAllocator,
    x_desc: miopenTensorDescriptor_t,
    w_desc: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
    y_desc: miopenTensorDescriptor_t,
) -> Result<Option<miopenConvAlgoPerf_t>, miopenError_t> {
    fn get_tensor_size(desc: miopenTensorDescriptor_t) -> Result<usize, miopenError_t> {
        let mut size_in_bytes = 0;
        unsafe { miopenGetTensorNumBytes(desc, &mut size_in_bytes)? };
        Ok(size_in_bytes)
    }
    let mut required_search_workspace_size = 0;
    miopenConvolutionForwardGetWorkSpaceSize(
        handle,
        w_desc,
        x_desc,
        conv_desc,
        y_desc,
        &mut required_search_workspace_size,
    )?;
    let w_size = get_tensor_size(w_desc)?;
    let x_size = get_tensor_size(x_desc)?;
    let y_size = get_tensor_size(y_desc)?;
    let fake_buffer_size = required_search_workspace_size
        .max(w_size)
        .max(x_size)
        .max(y_size);
    let fake_tensor = scratchpad
        .get_or_allocate(fake_buffer_size)
        .map_err(|_| miopenError_t::AllocFailed)?;
    let mut algo_count = 0;
    let mut perf_result = mem::zeroed();
    miopenFindConvolutionForwardAlgorithm(
        handle,
        x_desc,
        fake_tensor.data,
        w_desc,
        fake_tensor.data,
        conv_desc,
        y_desc,
        fake_tensor.data,
        1,
        &mut algo_count,
        &mut perf_result,
        fake_tensor.data,
        required_search_workspace_size,
        false,
    )?;
    if algo_count == 0 {
        Ok(None)
    } else {
        Ok(Some(perf_result))
    }
}

#[derive(PartialEq, Eq, Hash)]
struct ConvolutionOpCacheKey {
    x: TensorCacheKey,
    w: TensorCacheKey,
    conv: ConvolutionDescriptorCacheKey,
    y: TensorCacheKey,
}

impl ConvolutionOpCacheKey {
    fn new(
        x_desc: miopenTensorDescriptor_t,
        w_desc: miopenTensorDescriptor_t,
        conv_desc: miopenConvolutionDescriptor_t,
        y_desc: miopenTensorDescriptor_t,
    ) -> Result<Self, miopenError_t> {
        Ok(Self {
            x: TensorCacheKey::new(x_desc)?,
            w: TensorCacheKey::new(w_desc)?,
            conv: ConvolutionDescriptorCacheKey::new(conv_desc)?,
            y: TensorCacheKey::new(y_desc)?,
        })
    }
}

#[derive(PartialEq, Eq, Hash)]
struct TensorCacheKey {
    data_type: miopenDataType_t,
    dimensions: [i32; 5],
    strides: [i32; 5],
}

impl TensorCacheKey {
    fn new(desc: miopenTensorDescriptor_t) -> Result<Self, miopenError_t> {
        let mut data_type = unsafe { mem::zeroed() };
        let mut dimensions = [0; 5];
        let mut strides = [0; 5];
        unsafe {
            miopenGetTensorDescriptor(
                desc,
                &mut data_type,
                dimensions.as_mut_ptr(),
                strides.as_mut_ptr(),
            )?;
        }
        Ok(Self {
            data_type,
            dimensions,
            strides,
        })
    }
}

#[derive(PartialEq, Eq, Hash)]
struct ConvolutionDescriptorCacheKey {
    mode: miopenConvolutionMode_t,
    pad_h: i32,
    pad_w: i32,
    stride_h: i32,
    stride_w: i32,
    dilation_h: i32,
    dilation_w: i32,
}

impl ConvolutionDescriptorCacheKey {
    fn new(desc: miopenConvolutionDescriptor_t) -> Result<Self, miopenError_t> {
        let mut mode = unsafe { mem::zeroed() };
        let mut pad_h = 0;
        let mut pad_w = 0;
        let mut stride_h = 0;
        let mut stride_w = 0;
        let mut dilation_h = 0;
        let mut dilation_w = 0;
        unsafe {
            miopenGetConvolutionDescriptor(
                desc,
                &mut mode,
                &mut pad_h,
                &mut pad_w,
                &mut stride_h,
                &mut stride_w,
                &mut dilation_h,
                &mut dilation_w,
            )?;
        }
        Ok(Self {
            mode,
            pad_h,
            pad_w,
            stride_h,
            stride_w,
            dilation_h,
            dilation_w,
        })
    }
}

unsafe fn algo_perf_to_cudnn(result: miopenConvAlgoPerf_t) -> cudnnConvolutionFwdAlgoPerf_t {
    cudnnConvolutionFwdAlgoPerf_t {
        algo: algo_to_cudnn(&result),
        time: result.time,
        memory: result.memory,
        status: cudnnStatus_t::SUCCESS,
        determinism: cudnnDeterminism_t::CUDNN_NON_DETERMINISTIC,
        mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
        reserved: [0, 0, 0],
    }
}

unsafe fn algo_to_cudnn(result: &miopenConvAlgoPerf_t) -> cudnnConvolutionFwdAlgo_t {
    match result.__bindgen_anon_1.fwd_algo {
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoGEMM => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_GEMM
        }
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoDirect => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_DIRECT
        }
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoFFT => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_FFT
        }
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoWinograd => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD
        }
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoImplicitGEMM => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM
        }
        _ => cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_GEMM,
    }
}

pub(crate) unsafe fn get_convolution_forward_workspace_size(
    handle: &Context,
    x_desc: miopenTensorDescriptor_t,
    w_desc: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
    y_desc: miopenTensorDescriptor_t,
    _algo: miopenConvFwdAlgorithm_t,
    size_in_bytes: &mut usize,
) -> miopenStatus_t {
    let algo =
        get_or_search_convolution_forward_algorithm(handle, x_desc, w_desc, conv_desc, y_desc)?
            .ok_or(miopenError_t::UnsupportedOp)?;
    *size_in_bytes = algo.memory;
    Ok(())
}

pub(crate) unsafe fn convolution_forward(
    handle: &Context,
    alpha: *const ::std::os::raw::c_void,
    x_desc: miopenTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    w_desc: miopenTensorDescriptor_t,
    w: *const ::std::os::raw::c_void,
    conv_desc: miopenConvolutionDescriptor_t,
    _algo: miopenConvFwdAlgorithm_t,
    workspace: *mut ::std::os::raw::c_void,
    workspace_size_in_bytes: usize,
    beta: *const ::std::os::raw::c_void,
    y_desc: miopenTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
) -> miopenStatus_t {
    let do_convolution = |algo, pre_op_buffer| {
        let zero = 0u64;
        miopenConvolutionForward(
            handle.base,
            alpha,
            x_desc,
            x,
            w_desc,
            w,
            conv_desc,
            algo,
            std::ptr::from_ref(&zero).cast(),
            y_desc,
            y,
            workspace,
            workspace_size_in_bytes,
        )?;
        if let Some(pre_op_buffer) = pre_op_buffer {
            let one = 1.0f32;
            miopenOpTensor(
                handle.base,
                miopenTensorOp_t::miopenTensorOpAdd,
                beta,
                y_desc,
                pre_op_buffer,
                std::ptr::from_ref(&one).cast(),
                y_desc,
                y,
                std::ptr::from_ref(&zero).cast(),
                y_desc,
                y,
            )?;
        }
        Ok(())
    };
    // We don't allow users to select their own algorithm
    // because they might select algorithm that is not supported by miopen for this configuration
    // and there's no way to ask miopen to fall back to another algorithm
    let algo =
        get_or_search_convolution_forward_algorithm(handle, x_desc, w_desc, conv_desc, y_desc)?
            .ok_or(miopenError_t::UnsupportedOp)?;
    if algo.memory > workspace_size_in_bytes {
        return miopenStatus_t::ErrorInvalidValue;
    }
    let mut type_ = mem::zeroed();
    miopenGetTensorDescriptor(y_desc, &mut type_, ptr::null_mut(), ptr::null_mut())?;
    let beta_value = if type_ == miopenDataType_t::miopenDouble {
        *beta.cast::<f64>()
    } else {
        *beta.cast::<f32>() as f64
    };
    if beta_value != 0.0 {
        let mut y_size = 0;
        miopenGetTensorNumBytes(y_desc, &mut y_size)?;
        let mut mutable = handle
            .search_workspace
            .lock()
            .map_err(|_| miopenError_t::UnknownError)?;
        mutable.beta_buffer_cache.with_async_buffer(
            y_size,
            hipStream_t(ptr::null_mut()),
            |temp_buffer| {
                hipMemcpyDtoDAsync(
                    hipDeviceptr_t(temp_buffer),
                    hipDeviceptr_t(y),
                    y_size,
                    hipStream_t(ptr::null_mut()),
                )
                .map_err(|_| miopenError_t::InternalError)?;
                do_convolution(algo.__bindgen_anon_1.fwd_algo, Some(temp_buffer))
            },
        )
    } else {
        do_convolution(algo.__bindgen_anon_1.fwd_algo, None)
    }
}

pub(crate) unsafe fn destroy_convolution_descriptor(
    conv_desc: miopenConvolutionDescriptor_t,
) -> miopenStatus_t {
    miopenDestroyConvolutionDescriptor(conv_desc)
}

pub(crate) unsafe fn destroy_filter_descriptor(desc: miopenTensorDescriptor_t) -> miopenStatus_t {
    miopenDestroyTensorDescriptor(desc)
}

pub(crate) unsafe fn destroy_tensor_descriptor(desc: miopenTensorDescriptor_t) -> miopenStatus_t {
    miopenDestroyTensorDescriptor(desc)
}

// Activation descriptor wrapper
pub(crate) struct ActivationDescriptor {
    pub(crate) base: miopenActivationDescriptor_t,
}

impl ActivationDescriptor {
    fn new() -> Self {
        Self {
            base: miopenActivationDescriptor_t(ptr::null_mut()),
        }
    }
}

impl ZludaObject for ActivationDescriptor {
    const COOKIE: usize = 0x8a7c3f1e9d2b4065;

    type Error = cudnnError_t;
    type CudaHandle = cudnnActivationDescriptor_t;

    fn drop_checked(&mut self) -> Result<(), cudnnError_t> {
        if !self.base.0.is_null() {
            unsafe { miopenDestroyActivationDescriptor(self.base) }
                .map_err(|_| cudnnError_t::INTERNAL_ERROR)?;
        }
        Ok(())
    }
}

from_cuda_object!(ActivationDescriptor);

fn cudnn_to_miopen_activation_mode(
    mode: cudnnActivationMode_t,
) -> Result<miopenActivationMode_t, cudnnError_t> {
    Ok(match mode {
        cudnnActivationMode_t::CUDNN_ACTIVATION_SIGMOID => {
            miopenActivationMode_t::miopenActivationLOGISTIC
        }
        cudnnActivationMode_t::CUDNN_ACTIVATION_RELU => {
            miopenActivationMode_t::miopenActivationRELU
        }
        cudnnActivationMode_t::CUDNN_ACTIVATION_TANH => {
            miopenActivationMode_t::miopenActivationTANH
        }
        cudnnActivationMode_t::CUDNN_ACTIVATION_CLIPPED_RELU => {
            miopenActivationMode_t::miopenActivationCLIPPEDRELU
        }
        cudnnActivationMode_t::CUDNN_ACTIVATION_ELU => {
            miopenActivationMode_t::miopenActivationELU
        }
        cudnnActivationMode_t::CUDNN_ACTIVATION_IDENTITY => {
            miopenActivationMode_t::miopenActivationPASTHRU
        }
        cudnnActivationMode_t::CUDNN_ACTIVATION_SWISH => {
            // MIOpen doesn't have direct Swish support, use SOFTRELU as approximation
            miopenActivationMode_t::miopenActivationSOFTRELU
        }
        _ => return Err(cudnnError_t::NOT_SUPPORTED),
    })
}

pub(crate) unsafe fn create_activation_descriptor(
    activation_desc: *mut cudnnActivationDescriptor_t,
) -> cudnnStatus_t {
    if activation_desc.is_null() {
        return Err(cudnnError_t::BAD_PARAM);
    }
    let mut desc = ActivationDescriptor::new();
    miopenCreateActivationDescriptor(&mut desc.base)?;
    *activation_desc = ActivationDescriptor::wrap(desc);
    Ok(())
}

pub(crate) unsafe fn set_activation_descriptor(
    activation_desc: cudnnActivationDescriptor_t,
    mode: cudnnActivationMode_t,
    _relu_nan_opt: cudnnNanPropagation_t,
    coef: f64,
) -> cudnnStatus_t {
    // Get access to the descriptor through the live check
    let live_check = zluda_common::as_ref::<ActivationDescriptor>(&activation_desc)
        .ok_or(cudnnError_t::BAD_PARAM)?;
    let desc = live_check.as_result()?;

    let miopen_mode = cudnn_to_miopen_activation_mode(mode)?;

    // MIOpen uses alpha for the coefficient (e.g., for clipped relu ceiling or leaky relu slope)
    // beta and gamma are typically 0 for standard activations
    // NaN propagation option is not directly supported by MIOpen
    miopenSetActivationDescriptor(desc.base, miopen_mode, coef, 0.0, 0.0)?;
    Ok(())
}

pub(crate) unsafe fn get_activation_descriptor(
    activation_desc: cudnnActivationDescriptor_t,
    mode: *mut cudnnActivationMode_t,
    relu_nan_opt: *mut cudnnNanPropagation_t,
    coef: *mut f64,
) -> cudnnStatus_t {
    let desc: &ActivationDescriptor = zluda_common::FromCuda::from_cuda(&activation_desc)?;

    // Query MIOpen for the actual values
    let mut miopen_mode = mem::zeroed();
    let mut alpha = 0.0f64;
    let mut beta = 0.0f64;
    let mut gamma = 0.0f64;
    miopenGetActivationDescriptor(desc.base, &mut miopen_mode, &mut alpha, &mut beta, &mut gamma)?;

    if !mode.is_null() {
        // Map MIOpen mode back to cuDNN mode
        *mode = match miopen_mode {
            miopenActivationMode_t::miopenActivationLOGISTIC => cudnnActivationMode_t::CUDNN_ACTIVATION_SIGMOID,
            miopenActivationMode_t::miopenActivationRELU => cudnnActivationMode_t::CUDNN_ACTIVATION_RELU,
            miopenActivationMode_t::miopenActivationTANH => cudnnActivationMode_t::CUDNN_ACTIVATION_TANH,
            miopenActivationMode_t::miopenActivationCLIPPEDRELU => cudnnActivationMode_t::CUDNN_ACTIVATION_CLIPPED_RELU,
            miopenActivationMode_t::miopenActivationELU => cudnnActivationMode_t::CUDNN_ACTIVATION_ELU,
            miopenActivationMode_t::miopenActivationPASTHRU => cudnnActivationMode_t::CUDNN_ACTIVATION_IDENTITY,
            miopenActivationMode_t::miopenActivationSOFTRELU => cudnnActivationMode_t::CUDNN_ACTIVATION_SWISH,
            _ => cudnnActivationMode_t::CUDNN_ACTIVATION_RELU,
        };
    }
    if !relu_nan_opt.is_null() {
        *relu_nan_opt = cudnnNanPropagation_t::CUDNN_NOT_PROPAGATE_NAN;
    }
    if !coef.is_null() {
        *coef = alpha;
    }
    Ok(())
}

pub(crate) unsafe fn destroy_activation_descriptor(
    activation_desc: cudnnActivationDescriptor_t,
) -> cudnnStatus_t {
    zluda_common::drop_checked::<ActivationDescriptor>(activation_desc)?;
    Ok(())
}

pub(crate) unsafe fn activation_forward(
    handle: cudnnHandle_t,
    activation_desc: cudnnActivationDescriptor_t,
    alpha: *const ::std::os::raw::c_void,
    x_desc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    y_desc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    let ctx: &Context = zluda_common::FromCuda::from_cuda(&handle)?;
    let act_desc: &ActivationDescriptor = zluda_common::FromCuda::from_cuda(&activation_desc)?;
    miopenActivationForward(
        ctx.base,
        act_desc.base,
        alpha,
        miopenTensorDescriptor_t(x_desc as *mut _),
        x,
        beta,
        miopenTensorDescriptor_t(y_desc as *mut _),
        y,
    )?;
    Ok(())
}

pub mod dnn8 {
    use cuda_types::cudnn8::*;
    use std::mem;

    pub(crate) unsafe fn create_activation_descriptor(
        activation_desc: *mut cudnnActivationDescriptor_t,
    ) -> cudnnStatus_t {
        // The descriptor types are pointer-sized and compatible between versions
        status9_to_8(super::dnn9::create_activation_descriptor(
            activation_desc as *mut cuda_types::cudnn9::cudnnActivationDescriptor_t,
        ))
    }

    pub(crate) unsafe fn set_activation_descriptor(
        activation_desc: cudnnActivationDescriptor_t,
        mode: cudnnActivationMode_t,
        relu_nan_opt: cudnnNanPropagation_t,
        coef: f64,
    ) -> cudnnStatus_t {
        status9_to_8(super::dnn9::set_activation_descriptor(
            activation_desc as cuda_types::cudnn9::cudnnActivationDescriptor_t,
            mem::transmute(mode),
            mem::transmute(relu_nan_opt),
            coef,
        ))
    }

    pub(crate) unsafe fn get_activation_descriptor(
        activation_desc: cudnnActivationDescriptor_t,
        mode: *mut cudnnActivationMode_t,
        relu_nan_opt: *mut cudnnNanPropagation_t,
        coef: *mut f64,
    ) -> cudnnStatus_t {
        status9_to_8(super::dnn9::get_activation_descriptor(
            activation_desc as cuda_types::cudnn9::cudnnActivationDescriptor_t,
            mode as *mut cuda_types::cudnn9::cudnnActivationMode_t,
            relu_nan_opt as *mut cuda_types::cudnn9::cudnnNanPropagation_t,
            coef,
        ))
    }

    pub(crate) unsafe fn destroy_activation_descriptor(
        activation_desc: cudnnActivationDescriptor_t,
    ) -> cudnnStatus_t {
        status9_to_8(super::dnn9::destroy_activation_descriptor(
            activation_desc as cuda_types::cudnn9::cudnnActivationDescriptor_t,
        ))
    }

    pub(crate) unsafe fn activation_forward(
        handle: cudnnHandle_t,
        activation_desc: cudnnActivationDescriptor_t,
        alpha: *const ::std::os::raw::c_void,
        x_desc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        y_desc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t {
        status9_to_8(super::dnn9::activation_forward(
            handle as cuda_types::cudnn9::cudnnHandle_t,
            activation_desc as cuda_types::cudnn9::cudnnActivationDescriptor_t,
            alpha,
            x_desc as cuda_types::cudnn9::cudnnTensorDescriptor_t,
            x,
            beta,
            y_desc as cuda_types::cudnn9::cudnnTensorDescriptor_t,
            y,
        ))
    }

    pub(crate) fn get_error_string(
        status: cuda_types::cudnn8::cudnnStatus_t,
    ) -> *const ::core::ffi::c_char {
        super::dnn9::get_error_string(status8_to_9(status))
    }

    fn status8_to_9(
        status: cuda_types::cudnn8::cudnnStatus_t,
    ) -> cuda_types::cudnn9::cudnnStatus_t {
        match status {
            Ok(()) => Ok(()),
            Err(err) => Err(match err {
                cuda_types::cudnn8::cudnnError_t::NOT_INITIALIZED => {
                    cuda_types::cudnn9::cudnnError_t::NOT_INITIALIZED
                }
                cuda_types::cudnn8::cudnnError_t::ALLOC_FAILED => {
                    cuda_types::cudnn9::cudnnError_t::ALLOC_FAILED
                }
                cuda_types::cudnn8::cudnnError_t::BAD_PARAM => {
                    cuda_types::cudnn9::cudnnError_t::BAD_PARAM
                }
                cuda_types::cudnn8::cudnnError_t::INTERNAL_ERROR => {
                    cuda_types::cudnn9::cudnnError_t::INTERNAL_ERROR
                }
                cuda_types::cudnn8::cudnnError_t::INVALID_VALUE => {
                    cuda_types::cudnn9::cudnnError_t::INVALID_VALUE
                }
                cuda_types::cudnn8::cudnnError_t::ARCH_MISMATCH => {
                    cuda_types::cudnn9::cudnnError_t::ARCH_MISMATCH
                }
                cuda_types::cudnn8::cudnnError_t::MAPPING_ERROR => {
                    cuda_types::cudnn9::cudnnError_t::MAPPING_ERROR
                }
                cuda_types::cudnn8::cudnnError_t::EXECUTION_FAILED => {
                    cuda_types::cudnn9::cudnnError_t::EXECUTION_FAILED
                }
                cuda_types::cudnn8::cudnnError_t::NOT_SUPPORTED => {
                    cuda_types::cudnn9::cudnnError_t::NOT_SUPPORTED
                }
                cuda_types::cudnn8::cudnnError_t::LICENSE_ERROR => {
                    cuda_types::cudnn9::cudnnError_t::LICENSE_ERROR
                }
                cuda_types::cudnn8::cudnnError_t::RUNTIME_PREREQUISITE_MISSING => {
                    cuda_types::cudnn9::cudnnError_t::RUNTIME_PREREQUISITE_MISSING
                }
                cuda_types::cudnn8::cudnnError_t::RUNTIME_IN_PROGRESS => {
                    cuda_types::cudnn9::cudnnError_t::RUNTIME_IN_PROGRESS
                }
                cuda_types::cudnn8::cudnnError_t::RUNTIME_FP_OVERFLOW => {
                    cuda_types::cudnn9::cudnnError_t::RUNTIME_FP_OVERFLOW
                }
                cuda_types::cudnn8::cudnnError_t::VERSION_MISMATCH => {
                    cuda_types::cudnn9::cudnnError_t::VERSION_MISMATCH
                }
                _ => cuda_types::cudnn9::cudnnError_t::INTERNAL_ERROR,
            }),
        }
    }

    pub(crate) unsafe fn get_convolution_forward_algorithm_v7(
        handle: cudnnHandle_t,
        x_desc: cudnnTensorDescriptor_t,
        w_desc: cudnnFilterDescriptor_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        y_desc: cudnnTensorDescriptor_t,
        requested_algo_count: ::std::os::raw::c_int,
        returned_algo_count: *mut ::std::os::raw::c_int,
        perf_results: *mut cudnnConvolutionFwdAlgoPerf_t,
    ) -> Result<(), cudnnError_t> {
        if requested_algo_count <= 0 {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let mut perf_results_dnn9 = mem::zeroed();
        super::dnn9::get_convolution_forward_algorithm_v7(
            handle,
            x_desc,
            w_desc,
            conv_desc,
            y_desc,
            1,
            returned_algo_count,
            &mut perf_results_dnn9,
        )?;
        *perf_results = cudnnConvolutionFwdAlgoPerf_t {
            algo: perf_results_dnn9.algo,
            time: perf_results_dnn9.time,
            memory: perf_results_dnn9.memory,
            status: status9_to_8(perf_results_dnn9.status),
            determinism: perf_results_dnn9.determinism,
            mathType: perf_results_dnn9.mathType,
            reserved: [0, 0, 0],
        };
        Ok(())
    }

    fn status9_to_8(
        status: cuda_types::cudnn9::cudnnStatus_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t {
        match status {
            Ok(()) => Ok(()),
            Err(err) => Err(match err {
                cuda_types::cudnn9::cudnnError_t::NOT_INITIALIZED => {
                    cuda_types::cudnn8::cudnnError_t::NOT_INITIALIZED
                }
                cuda_types::cudnn9::cudnnError_t::ALLOC_FAILED => {
                    cuda_types::cudnn8::cudnnError_t::ALLOC_FAILED
                }
                cuda_types::cudnn9::cudnnError_t::BAD_PARAM => {
                    cuda_types::cudnn8::cudnnError_t::BAD_PARAM
                }
                cuda_types::cudnn9::cudnnError_t::INTERNAL_ERROR => {
                    cuda_types::cudnn8::cudnnError_t::INTERNAL_ERROR
                }
                cuda_types::cudnn9::cudnnError_t::INVALID_VALUE => {
                    cuda_types::cudnn8::cudnnError_t::INVALID_VALUE
                }
                cuda_types::cudnn9::cudnnError_t::ARCH_MISMATCH => {
                    cuda_types::cudnn8::cudnnError_t::ARCH_MISMATCH
                }
                cuda_types::cudnn9::cudnnError_t::MAPPING_ERROR => {
                    cuda_types::cudnn8::cudnnError_t::MAPPING_ERROR
                }
                cuda_types::cudnn9::cudnnError_t::EXECUTION_FAILED => {
                    cuda_types::cudnn8::cudnnError_t::EXECUTION_FAILED
                }
                cuda_types::cudnn9::cudnnError_t::NOT_SUPPORTED => {
                    cuda_types::cudnn8::cudnnError_t::NOT_SUPPORTED
                }
                cuda_types::cudnn9::cudnnError_t::LICENSE_ERROR => {
                    cuda_types::cudnn8::cudnnError_t::LICENSE_ERROR
                }
                cuda_types::cudnn9::cudnnError_t::RUNTIME_PREREQUISITE_MISSING => {
                    cuda_types::cudnn8::cudnnError_t::RUNTIME_PREREQUISITE_MISSING
                }
                cuda_types::cudnn9::cudnnError_t::RUNTIME_IN_PROGRESS => {
                    cuda_types::cudnn8::cudnnError_t::RUNTIME_IN_PROGRESS
                }
                cuda_types::cudnn9::cudnnError_t::RUNTIME_FP_OVERFLOW => {
                    cuda_types::cudnn8::cudnnError_t::RUNTIME_FP_OVERFLOW
                }
                cuda_types::cudnn9::cudnnError_t::VERSION_MISMATCH => {
                    cuda_types::cudnn8::cudnnError_t::VERSION_MISMATCH
                }
                _ => cuda_types::cudnn8::cudnnError_t::INTERNAL_ERROR,
            }),
        }
    }
}

pub mod dnn9 {
    use cuda_types::cudnn9::*;
    use zluda_common::FromCuda;

    pub(crate) unsafe fn create_activation_descriptor(
        activation_desc: *mut cudnnActivationDescriptor_t,
    ) -> cudnnStatus_t {
        super::create_activation_descriptor(activation_desc)
    }

    pub(crate) unsafe fn set_activation_descriptor(
        activation_desc: cudnnActivationDescriptor_t,
        mode: cudnnActivationMode_t,
        relu_nan_opt: cudnnNanPropagation_t,
        coef: f64,
    ) -> cudnnStatus_t {
        super::set_activation_descriptor(activation_desc, mode, relu_nan_opt, coef)
    }

    pub(crate) unsafe fn get_activation_descriptor(
        activation_desc: cudnnActivationDescriptor_t,
        mode: *mut cudnnActivationMode_t,
        relu_nan_opt: *mut cudnnNanPropagation_t,
        coef: *mut f64,
    ) -> cudnnStatus_t {
        super::get_activation_descriptor(activation_desc, mode, relu_nan_opt, coef)
    }

    pub(crate) unsafe fn destroy_activation_descriptor(
        activation_desc: cudnnActivationDescriptor_t,
    ) -> cudnnStatus_t {
        super::destroy_activation_descriptor(activation_desc)
    }

    pub(crate) unsafe fn activation_forward(
        handle: cudnnHandle_t,
        activation_desc: cudnnActivationDescriptor_t,
        alpha: *const ::std::os::raw::c_void,
        x_desc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        y_desc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t {
        super::activation_forward(handle, activation_desc, alpha, x_desc, x, beta, y_desc, y)
    }

    pub(crate) fn get_error_string(status: cudnnStatus_t) -> *const ::core::ffi::c_char {
        match status {
            Ok(()) => "CUDNN_STATUS_SUCCESS\0",
            Err(cudnnError_t::NOT_INITIALIZED) => "CUDNN_STATUS_NOT_INITIALIZED\0",
            Err(cudnnError_t::ALLOC_FAILED) => "CUDNN_STATUS_ALLOC_FAILED\0",
            Err(cudnnError_t::BAD_PARAM) => "CUDNN_STATUS_BAD_PARAM\0",
            Err(cudnnError_t::INTERNAL_ERROR) => "CUDNN_STATUS_INTERNAL_ERROR\0",
            Err(cudnnError_t::INVALID_VALUE) => "CUDNN_STATUS_INVALID_VALUE\0",
            Err(cudnnError_t::ARCH_MISMATCH) => "CUDNN_STATUS_ARCH_MISMATCH\0",
            Err(cudnnError_t::MAPPING_ERROR) => "CUDNN_STATUS_MAPPING_ERROR\0",
            Err(cudnnError_t::EXECUTION_FAILED) => "CUDNN_STATUS_EXECUTION_FAILED\0",
            Err(cudnnError_t::NOT_SUPPORTED) => "CUDNN_STATUS_NOT_SUPPORTED\0",
            Err(cudnnError_t::LICENSE_ERROR) => "CUDNN_STATUS_LICENSE_ERROR\0",
            Err(cudnnError_t::RUNTIME_PREREQUISITE_MISSING) => "CUDNN_STATUS_RUNTIME_PREREQUISITE_MISSING\0",
            Err(cudnnError_t::RUNTIME_IN_PROGRESS) => "CUDNN_STATUS_RUNTIME_IN_PROGRESS\0",
            Err(cudnnError_t::RUNTIME_FP_OVERFLOW) => "CUDNN_STATUS_RUNTIME_FP_OVERFLOW\0",
            Err(cudnnError_t::VERSION_MISMATCH) => "CUDNN_STATUS_VERSION_MISMATCH\0",
            _ => "CUDNN_STATUS_UNKNOWN\0",
        }
        .as_ptr() as *const ::core::ffi::c_char
    }

    pub(crate) unsafe fn get_convolution_forward_algorithm_v7(
        handle: cudnnHandle_t,
        x_desc: cudnnTensorDescriptor_t,
        w_desc: cudnnFilterDescriptor_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        y_desc: cudnnTensorDescriptor_t,
        requested_algo_count: ::std::os::raw::c_int,
        returned_algo_count: *mut ::std::os::raw::c_int,
        perf_results: *mut cudnnConvolutionFwdAlgoPerf_t,
    ) -> Result<(), cudnnError_t> {
        super::get_convolution_forward_algorithm_v7(
            FromCuda::<_, cudnnError_t>::from_cuda(&handle)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&x_desc)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&w_desc)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&conv_desc)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&y_desc)?,
            requested_algo_count,
            FromCuda::<_, cudnnError_t>::from_cuda(&returned_algo_count)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&perf_results)?,
        )?;
        Ok(())
    }
}
