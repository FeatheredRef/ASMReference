> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TESTUI

Performs a logical AND operation between the source operand and the destination operand, then sets the Zero Flag (ZF) in the RFLAGS register if the result is zero; otherwise, it clears ZF. The destination operand is not modified.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| imm | reg |
| reg | reg |
| #I | mN |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode or 32-bit mode.

The destination operand MUST be a register. Attempts to use a memory operand as the destination SHALL result in an encoding error or invalid opcode exception. Since this instruction does not modify the destination operand, it is primarily used for conditional branching based on bitmasking.