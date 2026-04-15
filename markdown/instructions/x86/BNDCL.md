> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BNDCL

Checks the lower bound of a bound register against the value of a source operand. If the source operand is less than the lower bound, the #BR exception is raised.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | #I |
| m64 | #I |
| imm | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

The instruction specifically checks the lower bound stored in the bound register (BND0-BND3) specified by the opcode. Since the bound register is implicit to the opcode, no destination register is modified.

To avoid unexpected #BR exceptions, ensure that the bound register has been properly initialized using `BNDMKL` or `BNDMKU` before executing `BNDCL`. If the bound register contains a value that is not a valid bound, the behavior may result in an immediate exception.