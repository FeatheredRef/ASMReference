> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVSD

Moves a doubleword (4 bytes) from the source operand to the destination operand.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| r32 | r32 |
| r32 | m4 |
| m4 | r32 |
| m4 | m4 |
| imm | r32 |
| imm | m4 |

DO NOT support LOCK

The instruction operates on doubleword operands. In x86-64 mode, if the instruction is used in 64-bit mode, it specifically targets the lower 32 bits of the registers. When targeting a register, the upper 32 bits of the destination r64 register are zeroed.

If the source and destination memory regions overlap, the behavior is undefined. Ensure that the source and destination addresses do not overlap to avoid data corruption. Memory operands MUST be aligned to 4-byte boundaries to avoid performance degradation or alignment check exceptions if AC flag is set in EFLAGS.