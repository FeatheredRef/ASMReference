> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KANDNB

Performs a bitwise AND operation between a quadword source and a quadword destination, and stores the result in the destination. The operation is performed on the operands as unsigned integers.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| m8 | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It is NOT available in compatibility mode or 32-bit protected mode.

The destination register MUST be a 64-bit register (r64). Using a 32-bit register as a destination will result in an invalid opcode exception.