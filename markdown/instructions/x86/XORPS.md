> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XORPS

Performs a bitwise logical exclusive OR (XOR) operation on the low 128 bits of two XMM operands. The operation is performed on each 32-bit single-precision floating-point element independently.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE extension to be enabled.

The memory operand MUST be 16-byte aligned. If the memory address is not aligned on a 16-byte boundary, an alignment check (#AC) exception SHALL occur if alignment check is enabled, or a general protection exception (#GP) MAY occur depending on the processor implementation. This instruction does not affect the floating-point status register flags (e.g., #P, #U, #O).