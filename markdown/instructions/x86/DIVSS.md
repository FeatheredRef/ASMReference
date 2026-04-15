> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# DIVSS

Divides a scalar single-precision floating-point value by another scalar single-precision floating-point value. The result is stored in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m4 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires SSE support.

The instruction follows the IEEE 754 standard for floating-point arithmetic. If the divisor is zero, it SHALL trigger #Z. If the result cannot be represented within the destination precision, it SHALL trigger #P. The precision and rounding control are governed by the MXCSR register.