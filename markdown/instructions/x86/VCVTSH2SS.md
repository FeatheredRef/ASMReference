> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTSH2SS

Converts a signed 16-bit integer value from a source operand to a single-precision floating-point value and stores the result in the destination operand.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| m16 | xmm |
| reg (xmm) | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode (provided SSE3 is supported). It requires the source operand to be a signed 16-bit integer (`i16`).

The conversion follows the IEEE 754 standard for rounding. If the result cannot be represented exactly as a single-precision floating-point number, the result is rounded according to the rounding control bits in the MXCSR register. This may trigger the #P exception. No other floating-point exceptions (#Z, #D, #O, #U) are triggered by this specific conversion.