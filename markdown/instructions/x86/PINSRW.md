> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PINSRW

Inserts a word (16 bits) from the source operand into a destination XMM register at the index specified by the immediate operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m2 | xmm |
| r16 | xmm |
| imm | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode or 32-bit mode. It requires the SSE3 extension set to be supported by the processor.

The immediate operand MUST be within the range of 0 to 7. If the immediate operand is outside this range, it will trigger a general protection exception (#GP). The insertion is performed by treating the XMM register as a vector of 16-bit elements; therefore, the index represents the 16-bit element position rather than the byte offset.