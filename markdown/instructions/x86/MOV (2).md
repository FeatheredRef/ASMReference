> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOV (2)

Copies the value from the source operand to the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| mN | reg |
| reg | mN |
| imm | mN |

DO NOT support LOCK

This instruction is supported in 64-bit mode, 32-bit mode, and 16-bit mode. In 64-bit mode, if the destination is a register, it MUST be a 64-bit register unless a prefix is used to override the operand size. 

When moving an immediate to a register, the immediate value is sign-extended to the size of the destination register if the operand size is 32 bits or less and the destination is a 64-bit register. Writing to a 32-bit register in 64-bit mode implicitly zeros the upper 32 bits of the destination register.