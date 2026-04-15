> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMAXPH

Compares two packed half-precision floating-point values and stores the maximum of the two values in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| m16 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is only available when the AVX-512 FP16 extension is supported. It operates on packed half-precision floating-point values (f16) within the specified vector register width.

The operation follows the IEEE 754-2008 standard for floating-point comparison. If one of the operands is a Quiet NaN, the result is the other operand; if both are NaNs, the result is a Quiet NaN.

Precision and rounding are governed by the current rounding mode in the MXCSR register. Improper configuration of the MXCSR may result in unexpected rounding behavior for edge cases.