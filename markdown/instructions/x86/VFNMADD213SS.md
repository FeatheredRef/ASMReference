> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD213SS

Computes a fused multiply-add operation on scalar single-precision floating-point values. The instruction calculates the result of `-(a * c) + b` and stores the result in the destination operand. Specifically, it multiplies the first operand by the third operand, negates the product, and adds the second operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| reg, reg, m32 | reg |
| reg, m32, reg | reg |
| m32, reg, reg | reg |

DO NOT support LOCK

This instruction requires the FMA3 feature flag to be supported by the processor. It is available in 64-bit mode and compatibility mode.

To avoid precision loss or unexpected behavior, ensure that the MXCSR register is correctly configured for rounding modes and denormal handling, as the operation is performed as a single fused step with only one rounding operation at the end. The instruction will trigger #P if the result is inexact, and #O or #U depending on the magnitude of the fused result.