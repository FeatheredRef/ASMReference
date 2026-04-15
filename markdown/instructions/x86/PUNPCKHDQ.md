> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUNPCKHDQ

Unpacks high quadwords of two 128-bit XMM operands into a single 128-bit XMM destination. Specifically, it selects the high 64 bits of the first source operand and the high 64 bits of the second source operand and concatenates them into the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m16 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE3 extension set to be supported by the processor.

The destination register MUST NOT be the same as the second source operand if the first source is a memory operand, to avoid potential data hazards, although the architectural definition allows the destination to be the same as either source. Since this is a SIMD operation, it does not affect the EFLAGS register.