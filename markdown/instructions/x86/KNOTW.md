> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KNOTW

Performs a bitwise logical NOT operation on a word-sized operand.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m2 | reg |
| imm | #I |

DO NOT support LOCK

This instruction is ONLY available when the processor is operating in 64-bit mode or compatibility mode. It operates specifically on word-sized (2 bytes) operands; attempting to use this instruction with dword or qword operands shall result in an invalid opcode exception.

The instruction does not affect any EFLAGS register bits. Ensure that the destination register is correctly sized as r16 to avoid unintentional truncation or corruption of adjacent bits in the register file.