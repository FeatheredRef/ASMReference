> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTDQ2PD

Converts a signed doubleword integer value from a source to a double-precision floating-point value in a destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m4 | f64 |
| r32 | f64 |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE3 extension.

The destination register must be an XMM register. Since the operation converts a 32-bit signed integer to a 64-bit float, only the lower 64 bits of the destination XMM register are modified; the upper bits remain unchanged. Precision and rounding are governed by the MXCSR register settings.

If the integer value exceeds the representable range of a double-precision floating-point number, the instruction SHALL signal #O. If the result is not exact, it SHALL signal #P.