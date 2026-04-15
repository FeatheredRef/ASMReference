> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MAXSD

Compares two double-precision floating-point values and stores the maximum of the two in the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm8 (f64) | xmm8 (f64) |
| m8 (f64) | xmm8 (f64) |

DO NOT support LOCK

This instruction requires SSE3 support. It operates on the low 64 bits of the XMM registers; the upper 64 bits of the destination XMM register remain unchanged.

The behavior of the instruction is subject to the current MXCSR register settings. Specifically, the rounding mode does not affect the result of the comparison, but the precision exception (#P) may be triggered if the result cannot be represented exactly. If any operand is a Signaling NaN, a floating-point exception is generated. If both operands are QNaNs, the result is the destination operand.