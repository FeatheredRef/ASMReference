> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MAXPD

Compares two packed double-precision floating-point values and stores the maximum value from each pair in the destination.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction requires the SSE3 extension. It is available in both 64-bit mode and compatibility mode.

If either operand is a Quiet NaN, the result is the Quiet NaN. If one operand is a Signaling NaN and the other is a Quiet NaN, the result is the Quiet NaN. If one operand is a Signaling NaN and the other is not a NaN, the result is the Signaling NaN. In the case of comparing a positive zero (+0.0) and a negative zero (-0.0), the positive zero is considered the maximum.

The instruction may trigger #P if the result is inexact.