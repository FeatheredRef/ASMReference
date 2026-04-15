> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KANDW

Performs a bitwise AND operation on a word-sized operand and a destination, then writes the result to the destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| reg | m2 |
| imm | m2 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is not supported in compatibility mode or 32-bit mode.

The operand size is fixed to word (2 bytes). Using a different operand size specifier will result in an invalid operation. The instruction does not affect any EFLAGS register bits.