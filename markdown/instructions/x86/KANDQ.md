> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KANDQ

Performs a bitwise AND operation on a quadword operand and the destination operand, then stores the result in the destination.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| reg | m8 |
| imm | m8 |

DO NOT support LOCK

The instruction is only available in 64-bit mode. It SHALL NOT be used in compatibility mode.

The destination register must be a 64-bit register (r64). If a 32-bit register is specified, the operation is invalid. When the destination is a memory operand, the size is fixed to qword.