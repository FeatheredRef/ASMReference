> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB213PS

Performs a fused multiply-subtract operation on packed single-precision floating-point values. The instruction computes the result of (a * c) - b for each corresponding element of the input vectors, where the source operands are mapped as follows: the first operand is the multiplier, the second is the subtrahend, and the third is the multiplicand.

The following table specifies the supported source and destination operand types.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m128 |
| m128 | reg |
| m128 | m128 |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX support bit to be enabled in the CPU and the XMM registers must be used.

The operation is performed as a single rounding step; the product of the multiplier and multiplicand is not rounded before the subtraction of the subtrahend. This prevents intermediate precision loss and avoids the #P and #U exceptions that would otherwise occur during a separate multiply and subtract sequence. All operations are subject to the current MXCSR rounding mode and floating-point exception masks.