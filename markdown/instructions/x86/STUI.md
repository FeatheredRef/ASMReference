> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# STUI

Stores the upper portion of a register into the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | mN |
| reg | reg |
| imm | #I |
| mN | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in compatibility mode. It SHALL NOT be used in 64-bit mode.

Ensure that the destination operand size matches the expected width of the upper portion being stored to avoid memory corruption or unintended data truncation.