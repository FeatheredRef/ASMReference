> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MINSD

Subtracts the minimum of two scalar double-precision floating-point values and stores the result in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm8(64) | xmm8(64) |
| m8 | xmm8(64) |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE3 support.

The operation is performed based on the floating-point rounding control in the MXCSR register. If the result cannot be represented exactly, the #P exception may be raised. The instruction handles special values (NaN, infinity) according to IEEE 754 standards. Only the lowest 64 bits of the destination xmm register are updated; the upper 64 bits remain unchanged.