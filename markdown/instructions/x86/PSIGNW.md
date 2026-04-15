> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSIGNW

Sign-extends a word-sized integer from a source to a destination. Specifically, it extracts the sign bit of the word in the source operand and propagates it to the upper bits of the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m2 | xmm |
| reg | xmm |
| imm | #I |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE4.1 instruction set extension.

The destination must be an XMM register. Using a memory operand as a destination is not supported. Failure to provide a valid XMM register as the destination will result in an invalid operand exception.