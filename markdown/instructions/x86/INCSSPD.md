> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INCSSPD

Increments the scalar double-precision floating-point value in the destination operand by 1.0.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Implicit | xmm |
| Implicit | m8 |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE3 instruction set extension.

The destination operand MUST be a double-precision floating-point value. If the destination is a memory operand, the memory access MUST be aligned to 8 bytes to avoid potential performance penalties or faults depending on the alignment check flag.

If the result of the operation cannot be represented in the destination format, it MAY trigger #O. If the result is not exact, it MAY trigger #P.