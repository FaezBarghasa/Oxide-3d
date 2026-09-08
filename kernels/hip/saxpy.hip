// SAXPY FP64 ROCm / HIP Kernel for Oxide-3D
#include <hip/hip_runtime.h>

extern "C" __global__ void saxpy_f64(
    double* __restrict__ y,
    const double* __restrict__ x,
    double a,
    size_t n
) {
    size_t i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        y[i] = a * x[i] + y[i];
    }
}
