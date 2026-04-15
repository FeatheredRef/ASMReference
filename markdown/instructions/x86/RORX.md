> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RORX

Rotates the bits of the source operand to the right by the count specified by the immediate operand. The result is stored in the destination register. Unlike ROR, this instruction is non-destructive, meaning the source operand is not modified.

The following table covers the supported source and destinations:

| source | destination(s) |
| :--- | :--- |
| reg / m64 | r64 |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode or 32-bit mode.

The rotate count is specified by an immediate value of 5 bits (0-31). If a value outside this range is provided in the immediate field, it will result in an encoding error as the opcode specifically allocates 5 bits for the shift/rotate count. Because it does not use the CL register, it avoids the data dependency on the flags register and does not affect the EFLAGS register.