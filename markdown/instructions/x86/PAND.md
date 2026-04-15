> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PAND

Performs a bitwise logical AND operation between the source operand and the destination operand, storing the result in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| memory | reg |
| reg | memory |
| imm | reg |
| imm | memory |

DO NOT support LOCK

This instruction is available in both 64-bit mode and compatibility mode. When operating on r64 operands, the instruction performs the AND operation on the full 64 bits.

If an immediate is used as the source, it MUST be a sign-extended 32-bit value for r64 operands. Performing PAND on memory operands MAY trigger a general-protection exception (#GP) if the memory access is not aligned and alignment checking is enabled in the EFLAGS register.