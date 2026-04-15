> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTPS2PD

Converts the lowest two single-precision floating-point values from a packed source to two double-precision floating-point values in a destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE3 instruction set extension.

The instruction only operates on the lowest 64 bits of the destination xmm register; the upper bits of the destination register are undefined. Precision and rounding are controlled by the MXCSR register. If the conversion results in a value that cannot be represented in the destination format, it SHALL trigger #O. If the result is inexact, it SHALL trigger #P.