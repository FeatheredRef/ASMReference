> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BNDMOV

BNDMOV copies the value from the source operand to the destination operand. If the destination operand is a register, the instruction performs a standard move. If the destination operand is a memory location, the instruction also updates the corresponding bound register (BND0-BND15) based on the memory address accessed.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | mN |
| mN | reg |
| mN | mN |
| imm | reg |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode.

When the destination is a memory operand, BNDMOV updates the bound register associated with the memory address. If the memory address is used, the processor MUST check the bounds and update the bound registers accordingly to ensure memory safety. Failure to ensure the processor is in 64-bit mode will result in an invalid opcode exception.