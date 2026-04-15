> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADDSUB231PH

This instruction performs a fused multiply-add or multiply-subtract operation on packed half-precision floating-point values. It computes the result as $res = (a \times b) + c$ or $res = (a \times b) - c$ depending on the immediate operand, where $a$ and $b$ are the source operands and $c$ is the accumulator.

The following table specifies the valid source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m128 |
| m128 | reg |
| m128 | m128 |

DO NOT support LOCK

This instruction is only available in 64-bit mode and 32-bit mode. It REQUIRES the AVX-512 FP16 extension to be enabled. The instruction operates on zmm or ymm registers; using it in compatibility mode without proper AVX-512 support will result in an invalid opcode exception.

The operation is performed according to the IEEE 754-2008 standard for fused multiply-add. Only one rounding operation is performed at the end of the computation, which prevents intermediate rounding errors. Precision exceptions #P and underflow #U may be triggered based on the MXCSR register settings. If the result exceeds the representable range of f16, a numeric overflow #O occurs. Denormalized inputs are handled according to the flush-to-zero (FTZ) and denormals-are-zero (DAZ) flags in MXCSR; otherwise, they may trigger #D.