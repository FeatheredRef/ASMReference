> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOV (1)

Copies the value from the source operand to the destination operand.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| mN | reg |
| reg | mN |
| imm | mN |

DO NOT support LOCK

The instruction SHALL NOT be used to move data from one memory location directly to another memory location; such an operation is #I. In 64-bit mode, if the destination is a 64-bit register (r64), the source SHALL be a qword.

When moving an immediate value to a register, the immediate size MUST match the register size. Moving a 32-bit immediate to a 32-bit register (r32) will zero-extend the result into the upper 32 bits of the corresponding r64 register. Moving an immediate to a memory location requires the use of a displacement or a specific operand size override to ensure the correct width of the write.