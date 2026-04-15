> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SBB

Subtracts the source operand and the carry flag (CF) from the destination operand. The result is stored in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| reg | mN |
| imm | reg |
| imm | mN |

DO NOT support LOCK

The instruction is available in 16-bit, 32-bit, and 64-bit operand sizes. In 64-bit mode, if a 32-bit register is used as the destination, the upper 32 bits of the 64-bit register are zeroed.

SBB cannot be used with an immediate value as the destination. The destination MUST be either a register or a memory location. When using an immediate value as a source, the immediate is treated as a signed integer.

The CF is used as an implicit input and updated as an output. Ensure that the CF is correctly set by a previous arithmetic or comparison instruction to avoid incorrect subtraction results.