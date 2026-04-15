> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVLHPS

Moves the low 32 bits of a packed single-precision floating-point value from the source operand to the high 32 bits of the destination operand. The low 32 bits of the destination operand remain unchanged.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m32 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE extension to be supported by the processor.

To avoid unintended data corruption, the destination operand MUST be an XMM register; memory-to-memory operations are NOT supported. Since the instruction only modifies the upper 32 bits of the destination XMM register, the lower 32 bits are preserved, which is critical when using the register for packed operations. No floating-point exceptions (#I, #Z, #D, #O, #U, #P) are generated as this is a data movement operation.