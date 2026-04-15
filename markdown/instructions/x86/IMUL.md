> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# IMUL

Multiplies a signed integer source by a signed integer destination. Depending on the operand size and form, it either stores the result in the destination operand or in a register pair to preserve the full product of the multiplication.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | mN |
| imm | reg |
| imm | mN |
| reg | implicit (AL/AX/EAX/RAX) |

DO NOT support LOCK

In the single-operand form, the source is implicit (AL, AX, EAX, or RAX depending on the operand size), and the result is stored in a register pair (AX, DX:AX, EDX:EAX, or RDX:RAX). In the two-operand form, the instruction supports 64-bit operands in 64-bit mode; however, it is NOT permitted to use a memory operand as both source and destination.

When using the single-operand form, the product may exceed the size of the destination register. If the result is too large to fit in the destination register, the Overflow Flag (OF) and Carry Flag (CF) SHALL be set to 1. In the two-operand form, if the result exceeds the destination register size, the upper bits are truncated, and OF and CF SHALL be set to 1.

Users MUST ensure the correct operand size is specified to avoid truncation of the product. When performing 64-bit multiplications that require the full 128-bit result, the two-operand `IMUL` form is insufficient; the `MUL` or `IMUL` (single-operand) instructions MUST be used to utilize the RDX:RAX register pair.