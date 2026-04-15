> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MAXSS

Compares two scalar single-precision floating-point values and stores the maximum of the two in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m4 | xmm |

DO NOT support LOCK

The instruction is available in both 64-bit mode and compatibility mode. It requires the SSE support feature.

When comparing a Quiet NaN with a numeric value, the numeric value is returned. If both operands are NaNs, the destination is set to the source operand if it is a NaN, or the destination operand if it is a NaN, following the IEEE 754 specification for `max`. Specifically, if any operand is a Signaling NaN, a floating-point exception shall be raised.

The operation may set the following flags:
- #D: If any operand is denormal.
- #O: If the result exceeds the maximum representable value.
- #U: If the result is smaller than the minimum representable value.
- #P: If the result is rounded.