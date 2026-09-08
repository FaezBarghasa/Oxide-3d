#include <metal_stdlib>
using namespace metal;

kernel void saxpy_f32(
    device float* y [[buffer(0)]],
    device const float* x [[buffer(1)]],
    constant float& a [[buffer(2)]],
    constant uint& n [[buffer(3)]],
    uint id [[thread_position_in_grid]]
) {
    if (id < n) {
        y[id] = a * x[id] + y[id];
    }
}
