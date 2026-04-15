> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MULPD

Multiplies two packed double-precision floating-point values. The instruction multiplies the lower 64 bits of the first operand by the lower 64 bits of the second operand, and the upper 64 bits of the first operand by the upper 64 bits of the second operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE3 support.

To avoid precision loss or unexpected exceptions, ensure that the MXCSR register is configured correctly for rounding and denormal handling. Results that exceed the maximum representable value for a 64-bit floating-point number SHALL trigger #O. Results that are too small to be represented SHALL trigger #U. If the result is not exact, #P SHALL be set.