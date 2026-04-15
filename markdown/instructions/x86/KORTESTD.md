> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KORTESTD

Performs a bitwise logical AND between the destination operand and the source operand, updates the EFLAGS register based on the result, and then performs a bitwise logical OR between the destination operand and the source operand, storing the result in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| reg | m4 |
| imm | m4 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode.

To avoid unexpected behavior, note that the EFLAGS updates (specifically ZF, SF, and PF) are based on the result of the AND operation, not the final OR result stored in the destination.