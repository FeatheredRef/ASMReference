> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD231PH

Performs a fused multiply-add operation on packed half-precision floating-point values. The instruction computes the result of (a * b) - c, where a and b are the first two operands and c is the third operand. The result is rounded once at the end of the operation.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm | zmm/ymm/xmm |
| zmm/ymm/xmm | zmm/ymm/xmm |
| m128/m256/m512 | zmm/ymm/xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires a processor supporting the AVX-512 Fused Multiply-Add (AVX512F) and AVX-512 Half-Precision (AVX512FP16) instruction sets.

The operation is subject to the floating-point control word settings in the MXCSR register. Specifically, the rounding mode determines how the final result is rounded. If an exception occurs, the corresponding flags (#O, #U, #P) are set in the MXCSR register. The instruction SHALL treat signaling NaNs according to the current floating-point exception masking. Since this is a fused operation, intermediate overflow or underflow during the multiplication phase does not trigger an exception if the final result is representable.