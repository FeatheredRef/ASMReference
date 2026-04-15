> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD231PD

Computes a fused multiply-add operation for double-precision floating-point values. It calculates the result as: destination = -(a * b) + c, where the operands are mapped according to the 231 pattern (source1 * source2) is subtracted from source3.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm reg, zmm/ymm/xmm reg, zmm/ymm/xmm reg | zmm/ymm/xmm reg |
| zmm/ymm/xmm reg, zmm/ymm/xmm reg, m128/m256/m512 | zmm/ymm/xmm reg |
| zmm/ymm/xmm reg, m128/m256/m512, zmm/ymm/xmm reg | zmm/ymm/xmm reg |
| m128/m256/m512, zmm/ymm/xmm reg, zmm/ymm/xmm reg | zmm/ymm/xmm reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or 32-bit mode. It REQUIRES the AVX-512 foundation extensions. The instruction is not supported in compatibility mode.

The instruction is subject to floating-point exception flags. Results MAY trigger #P, #O, #U, or #D based on the MXCSR register settings. If the result is a NaN, the specific NaN type depends on the operands. To avoid precision loss or unexpected rounding, the user MUST ensure the rounding control field in MXCSR is correctly configured, as FMA operations perform a single rounding step at the end of the combined multiply-add sequence.