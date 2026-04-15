> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRNDSCALEPH

Rounds half-precision floating-point values to a specified precision by scaling them by a power of two and rounding to the nearest integer, then scaling back. It operates on half-precision floating-point elements within the source operand and stores the results in the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm | zmm |
| ymm | ymm |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor supports AVX-512 FP16. It requires the processor to be in 64-bit mode or compatibility mode.

The rounding mode is controlled by the `rounding` field within the immediate byte; if the immediate is not provided, the rounding is governed by the MXCSR register. The scale factor is determined by the `scale` field in the immediate. Users MUST ensure the scale factor is within the supported range to avoid unexpected precision loss or saturation. An incorrect immediate encoding will result in an invalid opcode exception.

The following floating-point exceptions may be signaled:
- #P: If the rounded result is not exact.
- #O: If the result exceeds the representable range of f16.
- #U: If the result is too small to be represented as a normalized f16.