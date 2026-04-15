> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUBADD132PH

Performs a fused multiply-subtract and add operation on half-precision floating-point values. It computes the result of (a * b) - c + d for each element in the source operands.

The following table describes the supported source and destination types.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction requires the AVX-512 FP16 extension. It is NOT available in compatibility mode; it MUST be executed in 64-bit mode.

To avoid precision loss or unexpected behavior, ensure that the destination register is not used as a source for a different operand unless the intended behavior is accumulation. Be aware that this instruction may trigger floating-point exceptions: #O, #U, and #P depending on the rounding mode and the magnitude of the intermediate product. Underflow (#U) and Inexact (#P) flags are set if the rounded result differs from the mathematical result.