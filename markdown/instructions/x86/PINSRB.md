> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PINSRB

Inserts a byte from the source operand into a destination XMM register at the byte index specified by the immediate value.

The table below covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| r8 | xmm |
| m1 | xmm |
| imm | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode and compatibility mode. It requires the SSE4.1 instruction set extension to be supported by the processor.

The immediate byte index SHALL be within the range of 0 to 15. If the immediate value is outside this range, the instruction will trigger a general protection fault (#GP). The source operand MUST be a register or memory location containing at least one byte.