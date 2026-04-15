> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MINPD

MINPD computes the minimum of two packed double-precision floating-point values. It compares the two values and stores the minimum of each pair in the destination.

The following table describes the supported source and destination operands.

| Source | Destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction requires the SSE2 instruction set. It is supported in 64-bit mode and compatibility mode.

If either operand is a Quiet NaN, the result is the non-NaN operand; if both are NaNs, the result is one of the NaNs. If one operand is a Signaling NaN, it SHALL trigger a floating-point exception. Precision (#P) and Underflow (#U) exceptions may be raised depending on the rounding mode and the magnitude of the operands. Ensure that the memory operands are 16-byte aligned to avoid performance penalties or general protection faults on certain implementations.