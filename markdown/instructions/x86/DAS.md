> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# DAS

Decrements the destination operand by the value of the source operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| #I | mN |

DO NOT support LOCK

This instruction is NOT supported in 64-bit mode. It is only available in 16-bit and 32-bit compatibility modes.

The destination operand MUST be a register. Attempts to use a memory operand for the destination will result in an invalid operation.