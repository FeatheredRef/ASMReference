> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ADDSUBPD

Adds or subtracts packed double-precision floating-point values. If the sign bit of the destination operand is 0, the instruction adds the source to the destination. If the sign bit is 1, the instruction subtracts the source from the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction requires SSE4.1. It is not supported in 16-bit mode.

To avoid unexpected behavior, ensure the destination register is correctly initialized, as the operation (addition vs subtraction) is determined solely by the sign bit of the destination operand. Precision and rounding are governed by the MXCSR register settings.

Possible exceptions:
- #O
- #U
- #P
- #D
- #Z