# ZLUDA-Phork

A fork of [ZLUDA](https://github.com/vosen/ZLUDA) with extended CUDA library support targeting ML/LLM workloads on AMD GPUs.

ZLUDA is a drop-in replacement for CUDA on non-NVIDIA GPUs. It allows running unmodified CUDA applications using AMD GPUs with near-native performance via ROCm/HIP.

---

## [IN PROGRESS] Current Development Status & Issues

### Flash Attention MMA Implementation (In Progress)

We've implemented hardware-accelerated m16n8k8 MMA (Matrix Multiply-Accumulate) support for RDNA3 GPUs to enable flash attention in llama.cpp and other ML workloads. **The implementation is functionally complete but currently blocked by environment compatibility issues.**

#### [DONE] What's Implemented

**Complete PTX to AMD GPU Pipeline:**
1. **PTX Parser** (`ptx_parser/`) - Added `MovMatrix` instruction variant and parser rules for `movmatrix.sync.aligned`
2. **Instruction Replacement** (`ptx/src/pass/replace_instructions_with_functions.rs`) - Maps MovMatrix to runtime functions
3. **C++ Runtime** (`ptx/lib/zluda_ptx_impl.cpp`) - Implemented both scalar fallback and hardware intrinsic paths:
   - `movmatrix_m8n8_trans_b16()` - 8×8 matrix transpose using AMD's `__builtin_amdgcn_ds_bpermute`
   - `mma_sync_aligned_m16n8k8_*()` - MMA entry points with RDNA3 hardware acceleration
4. **LLVM Intrinsics** (`ext/llvm-project/llvm/include/llvm/IR/IntrinsicsZLUDA.td`) - Defined `int_zluda_mma_m16n8k8_f32_f16_f16_f32`
5. **LLVM Lowering Pass** (`ext/llvm-project/llvm/lib/Transforms/ZLUDA/CombineMMA.cpp`) - Lowers to AMD `amdgcn_wmma_f32_16x16x16_f16`

**Hardware Support:**
- ISA detection: Correctly identifies RDNA3 (gfx1100-gfx1103, ISA 11.0.0-11.0.3)
- Bitcode compilation: Properly generates LLVM bitcode with hardware paths
- Falls back to scalar implementation on older GPUs

#### [BLOCKED] Current Blockers

**ROOT CAUSE IDENTIFIED: ROCm 6.2 Code Object V6 Incompatibility**

ZLUDA is fundamentally incompatible with ROCm 6.0+ due to Code Object format changes.

**The Error:**
```
CUDA error: named symbol not found
  in function ggml_cuda_compute_forward (SOFT_MAX)
```

**Why This Happens:**
- ZLUDA targets ROCm 5.7.x and generates **Code Object V5** binaries via `libamd_comgr`
- ROCm 6.2 expects **Code Object V6** with different symbol mangling/visibility rules
- When ZLUDA JIT-compiles PTX kernels, the symbol names don't match what the 6.2 runtime expects
- This is NOT a gfx1103-specific issue - it affects all GPUs on ROCm 6.0+

**Attempted Workarounds (All Failed):**
- [FAIL] Installing community gfx1103 rocBLAS libraries (they're 5.7-based, wrong runtime)
- [FAIL] HSA_OVERRIDE_GFX_VERSION environment variables (doesn't fix Code Object mismatch)
- [FAIL] LD_PRELOAD library override (causes libstdc++ conflicts)
- [FAIL] Custom bitcode compilation (unrelated to runtime symbol resolution)

**The Solution: Downgrade to ROCm 5.7.1**

ROCm 5.7.1 is the "Golden Version" for ZLUDA compatibility:
```bash
# 1. Completely remove ROCm 6.2
sudo apt purge rocm-* hip-* --autoremove

# 2. Install ROCm 5.7.1 (instructions at https://rocm.docs.amd.com)
# 3. Set environment for gfx1103:
export HSA_OVERRIDE_GFX_VERSION=11.0.0  # Treat as gfx1100 (RX 7900)
export LD_LIBRARY_PATH=/opt/rocm-5.7.1/lib:$LD_LIBRARY_PATH
```

**Testing Status:**
- [BLOCKED] Cannot test MMA implementation until ROCm downgrade
- [READY] Code is complete and awaiting compatible environment

**2. LLVM Intrinsic Lowering Integration**

The hardware MMA path requires our custom LLVM with the CombineMMA pass. Currently disabled (using scalar fallback) because:
- ZLUDA uses system LLVM (`/opt/rocm/llvm`)
- Our modifications are in `ext/llvm-project` submodule
- Building and integrating custom LLVM is a 1-2 hour process

To enable hardware acceleration, need to either:
- Build ZLUDA against our modified LLVM, OR
- Upstream the LLVM changes to ROCm's LLVM

#### Diagnostic Commands (For ROCm 6.2 Debugging)

If you must stay on ROCm 6.2 for other reasons, these may help identify the symbol mismatch:

```bash
# Enable verbose logging
export AMD_LOG_LEVEL=3
export ZLUDA_LOG=trace

# Inspect JIT-compiled kernels
find ~/.cache/zluda -name "*.hsaco" -mmin -5  # Find recent compilations
/opt/rocm/llvm/bin/llvm-objdump -t path/to/kernel.hsaco  # Check symbol table
```

Look for mismatched kernel name suffixes (e.g., `.kd` appended/missing).

#### Questions for the Community

1. **Has anyone successfully run ZLUDA on ROCm 5.7.1 with RDNA3 hardware?**
   - Does gfx1103 work with HSA_OVERRIDE_GFX_VERSION=11.0.0?
   - Are there any quirks with APU vs discrete GPUs?

2. **ROCm 6.x Porting Effort:**
   - Is anyone working on updating ZLUDA for Code Object V6 compatibility?
   - Would this require changes to `libamd_comgr` integration or LLVM backend?

#### Technical Notes

**Bitcode Compilation:**
The PTX runtime must be compiled to proper LLVM bitcode:
```bash
hipcc --std=c++20 --offload-arch=gfx1103 -c -emit-llvm -O3 \
  ptx/lib/zluda_ptx_impl.cpp -o ptx/lib/zluda_ptx_impl_temp.bc

/opt/rocm/llvm/bin/clang-offload-bundler \
  --type=bc --targets=hip-amdgcn-amd-amdhsa-gfx1103 \
  --input=ptx/lib/zluda_ptx_impl_temp.bc \
  --output=ptx/lib/zluda_ptx_impl.bc \
  --unbundle
```
The offload bundler extracts raw bitcode from HIP's bundle format. Without this, ZLUDA fails with "file doesn't start with bitcode header".

**Hardware Path Toggle:**
To switch between hardware and scalar MMA, modify the ISA version checks in `ptx/lib/zluda_ptx_impl.cpp`:
```cpp
if (__oclc_ISA_version >= 11000 && __oclc_ISA_version < 12000)  // Hardware
if (false && __oclc_ISA_version >= 11000 && ...)  // Force scalar fallback
```

#### Help Wanted

If you have experience with:
- ZLUDA on RDNA3 APUs (especially gfx1103)
- rocBLAS symbol resolution / TensileLibrary integration
- LLVM pass integration in ROCm builds
- Alternative ROCm versions that work better with gfx1103

Please open an issue or discussion! The MMA implementation is ready - we just need the environment sorted out.

---

## What This Fork Adds

This fork extends upstream ZLUDA with implementations of CUDA libraries that ML frameworks depend on. All implementations use AMD's native ROCm backend libraries.

| Library | Backend | Functions Implemented | Status |
|---------|---------|----------------------|--------|
| **cuBLAS** | rocBLAS | GEMM (S/D/H/Ex/Batched), Level 1 (axpy, scal, dot, nrm2, asum, copy), Level 2 (gemv), Level 3 (trsm, symm, syrk) | ~70% |
| **cuBLASLt** | hipBLASLt | MatMul, MatmulDescCreate/Set/Get, MatLayoutCreate/Set/Get, version/error functions | ~35% |
| **cuDNN** | MIOpen | Activation (fwd/bwd), Pooling (fwd/bwd), Softmax (fwd/bwd), BatchNorm (fwd train/infer, bwd), LayerNorm (fwd), Conv (fwd), N-dim tensor descriptors | ~55% |
| **cuSPARSE** | hipSPARSE | Handle mgmt, SpMV, SpMM, dense/sparse matrix descriptors, CSR/COO formats | ~45% |
| **cuFFT** | hipFFT | Create/Destroy/Plan (1d/many/Nd), SetStream/SetWorkArea, Exec (C2C/R2C/C2R/Z2Z/D2Z/Z2D) | ~80% |
| **cuRAND** | hipRAND | CreateGenerator (host/device), Generate (uniform/normal/logNormal/Poisson), SetSeed/Stream/Offset/Ordering | ~90% |

## Quick Start

### Prerequisites

- AMD GPU with ROCm 6.x support
- ROCm 6.2+ installed (`/opt/rocm`)
- Rust toolchain (for building from source)
- LLVM/Clang (pulled by cargo build)

### Building

```bash
git clone https://github.com/mixelpixx/ZLUDA-Phork.git
cd ZLUDA-Phork

# IMPORTANT: Fetch Git LFS files (LLVM device libraries)
git lfs pull

# Build all libraries
cargo build --release
```

The build produces shared libraries in `target/release/`:
- `libnvcuda.so` — CUDA driver API
- `libcublas.so` — cuBLAS
- `libcublaslt.so` — cuBLASLt
- `libcudnn64_8.so` / `libcudnn64_9.so` — cuDNN
- `libcufft.so` — cuFFT
- `libcurand.so` — cuRAND
- `libcusparse.so` — cuSPARSE
- `libzluda_ld.so` — LD_AUDIT loader (intercepts CUDA library loading)

### Running CUDA Applications

ZLUDA uses Linux's `LD_AUDIT` mechanism to transparently redirect CUDA library loads to ZLUDA implementations:

```bash
# Set up ZLUDA
export LD_AUDIT=/path/to/ZLUDA/target/release/libzluda_ld.so
export LD_LIBRARY_PATH="/path/to/ZLUDA/target/release:$LD_LIBRARY_PATH"

# For RDNA3 iGPUs (gfx1103) — needed for rocBLAS Tensile library compatibility
export HSA_OVERRIDE_GFX_VERSION=11.0.0

# Run any CUDA application
./your-cuda-app
```

## llama.cpp Usage

llama.cpp runs successfully through ZLUDA. Requires building with specific flags:

### Building llama.cpp for ZLUDA

```bash
git clone https://github.com/ggerganov/llama.cpp
cd llama.cpp

# PTX-only build (no SASS) + cuBLAS path (no MMQ kernels)
cmake -B build \
  -DGGML_CUDA=ON \
  -DCMAKE_CUDA_ARCHITECTURES="75-virtual" \
  -DGGML_CUDA_FORCE_CUBLAS=ON

cmake --build build -j$(nproc)
```

### Running llama.cpp

```bash
export LD_AUDIT=/path/to/ZLUDA/target/release/libzluda_ld.so
export LD_LIBRARY_PATH="/path/to/ZLUDA/target/release:/path/to/llama.cpp/build/bin"
export HSA_OVERRIDE_GFX_VERSION=11.0.0  # Adjust for your GPU

./build/bin/llama-cli \
  -m model.gguf \
  -p "Hello, how are you?" \
  -n 100 \
  -ngl 99 \
  -fa off
```

### Tested Performance

| Model | GPU | Prompt (t/s) | Generation (t/s) |
|-------|-----|-------------|-------------------|
| TinyLlama 1.1B Q4_K_M | Radeon 780M (gfx1103) | 29.3 | 64.8 |

### llama.cpp Build Flags Explained

| Flag | Why |
|------|-----|
| `CMAKE_CUDA_ARCHITECTURES="75-virtual"` | Generates PTX only (no SASS binaries). ZLUDA's compiler translates PTX to AMDGPU. |
| `GGML_CUDA_FORCE_CUBLAS=ON` | Routes matrix multiplication through cuBLAS (our rocBLAS backend) instead of specialized MMQ CUDA kernels that use PTX instructions ZLUDA doesn't yet support. |
| `-fa off` | Disables flash attention. The flash attention kernels use advanced PTX features that ZLUDA's compiler doesn't yet handle. |

## HSA_OVERRIDE_GFX_VERSION Reference

If rocBLAS doesn't have Tensile kernels for your exact GPU, override to a compatible architecture:

| Your GPU | Architecture | Override |
|----------|-------------|----------|
| Radeon RX 7900 XTX | gfx1100 | Not needed |
| Radeon RX 7800 XT | gfx1101 | `11.0.0` |
| Radeon RX 7700 XT | gfx1102 | `11.0.0` |
| Radeon 780M (iGPU) | gfx1103 | `11.0.0` |
| Radeon RX 7600 | gfx1102 | `11.0.0` |

## Tracing / Debugging

ZLUDA includes trace libraries for capturing CUDA API calls:

```bash
# Use trace mode (logs all CUDA calls)
export LD_AUDIT=/path/to/ZLUDA/target/release/trace/zluda_ld
export LD_LIBRARY_PATH="/path/to/ZLUDA/target/release"
export ZLUDA_CUDA_LIB=/path/to/ZLUDA/target/release/libnvcuda.so
export ZLUDA_BLAS_LIB=/path/to/ZLUDA/target/release/libcublas.so
export ZLUDA_LOG_DIR=/tmp/my_trace

./your-cuda-app
# Trace output in /tmp/my_trace/<app-name>/log.txt
```

## Known Limitations

- **Flash attention kernels** — ZLUDA's PTX compiler doesn't yet support all PTX instructions used by flash attention (MMA intrinsics, `cp.async`). Use `-fa off` with llama.cpp.
- **MMQ kernels** — Specialized matrix-multiply-quantized kernels in llama.cpp don't compile. Use `GGML_CUDA_FORCE_CUBLAS=ON`.
- **cuDNN training** — Convolution backward pass needs workspace/algorithm caching (partially implemented).
- **Git LFS required** — The LLVM device libraries (`ockl.bc`, `ocml.bc`) are stored in Git LFS. Without `git lfs pull`, builds will succeed but crash at runtime with "Invalid bitcode signature".
- **`%envreg` PTX registers** — Some CUDA runtime internal PTX modules use `%envreg` special registers that ZLUDA's parser doesn't support. These modules are non-critical.

## Fork Commits

| Commit | Description |
|--------|-------------|
| `3276533` | Implement cuRAND via hipRAND backend (25 functions) |
| `3346689` | Implement cuFFT via hipFFT backend (32 functions) |
| `def2b21` | Implement N-dimensional tensor descriptors for cuDNN |
| `49d8b51` | Implement cuDNN normalization forward inference |
| `00c0685` | Implement cuDNN backward passes and batch norm forward training |
| `c4a98cd` | Implement cuBLAS Level 1/3 and half-precision batched operations |
| `c1525d1` | Implement cuSPARSE via hipSPARSE backend |
| `7f13a81` | Implement cuBLAS Level 2 GEMV operations |
| `b4ec4d6` | Implement cuDNN softmax and batch normalization operations |
| `049fa40` | Implement cuDNN pooling operations |
| `9693359` | Implement cuDNN activation functions |
| `1632f81` | Implement cuBLAS Level 1 ops and async memory functions |
| `f3c013d` | cuGraphInstantiateWithParams and cuSPARSE matrix descriptors |
| `09c4506` | Version and error functions in cuBLASLt and cuSPARSE |
| `5655148` | Critical version and error functions in cuBLAS and cuDNN |
| `66eca7f` | get_module_from_cubin and get_module_from_cubin_ext2 |
| `1b384dc` | get_module_from_cubin_ext1 for CUDA Runtime API support |

## Upstream

This fork is based on [vosen/ZLUDA](https://github.com/vosen/ZLUDA). Upstream focuses on the PTX compiler and CUDA driver API. Our fork extends the library coverage for ML workloads.

- [Upstream documentation](https://zluda.readthedocs.io)
- [Upstream Discord](https://discord.gg/sg6BNzXuc7)
