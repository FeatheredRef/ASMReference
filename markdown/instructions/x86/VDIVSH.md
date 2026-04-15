> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VDIVSH

Divides the lower 16 bits of the first source operand by the lower 16 bits of the second source operand. The result is sign-extended to the size of the destination operand.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m16 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE4.1 instruction set extension.

The operation is performed on signed integers (i16). If the divisor is zero, the result is set to an indefinite value or NaN depending on the floating-point environment, but it does not trigger a CPU exception like the integer `DIV` instruction. Overflow occurs if the quotient cannot be represented within a signed 16-bit integer (e.g., dividing -32768 by -1), in which case the result is the signed saturation value.