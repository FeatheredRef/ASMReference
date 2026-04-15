> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVHLPS

Moves the high-order 64 bits of a packed double-precision floating-point value from a memory location to the high-order 64 bits of an XMM register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE3 instruction set extension.

The source operand MUST be a memory location; using a register as a source is not supported and will result in an invalid encoding. The instruction only modifies the upper 64 bits of the destination xmm register, leaving the lower 64 bits undisturbed. Failure to ensure the lower 64 bits of the destination register are correctly initialized may result in corrupted packed data when the register is later used in SIMD operations.