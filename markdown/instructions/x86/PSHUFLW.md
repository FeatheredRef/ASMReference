> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSHUFLW

The instruction shuffles 32-bit double words from the source operand to the destination operand based on an immediate byte. For each 32-bit element in the destination, the corresponding 8-bit field in the immediate byte selects which 32-bit element from the source is copied.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |
| imm | #I |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It REQUIRES the SSE2 instruction set extension.

The immediate operand MUST be a byte; using an immediate value outside the range of 0-255 is invalid. The destination register is overwritten, and since the instruction uses a source and destination register, the destination MUST NOT be the same as the source if memory is involved, though xmm register-to-register operations allow overlap.