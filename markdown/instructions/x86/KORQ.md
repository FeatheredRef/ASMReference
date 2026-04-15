> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KORQ

Performs a bitwise logical exclusive OR operation on the destination operand and the source operand, producing a qword result.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8 | reg |
| imm | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The destination register must be an r64 register. Using this instruction in 32-bit mode or compatibility mode will result in an invalid opcode exception.