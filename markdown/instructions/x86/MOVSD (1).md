> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVSD (1)

Copies a doubleword value from the source operand to the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| r32 | r32 |
| r32 | m4 |
| m4 | r32 |
| m4 | m4 |
| imm | r32 |
| imm | m4 |
| #I | #I |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. In 64-bit mode, if a 64-bit register is specified as the destination for a 32-bit move, the upper 32 bits of the destination register SHALL be zeroed.

The destination operand SHALL NOT overlap with the source operand if they are both memory references to avoid undefined behavior. When using `MOVSD` to move data between memory locations, the operation is NOT atomic.