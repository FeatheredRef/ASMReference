> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSRLW

Shifts a doubleword value to the right by the number of bits specified in the source operand. Vacant bits are filled with zeros.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | r32 |
| r32 | r32 |
| #I | m32 |

DO NOT support LOCK

This instruction is available only in 32-bit mode or compatibility mode. It cannot be used in 64-bit mode.

The shift count is masked to 5 bits (u5), meaning only values from 0 to 31 are effective. If the source is a register, only the low-order 5 bits of that register are used to determine the shift distance. Failure to mask the shift count before providing it in a register may lead to unexpected results due to this implicit masking.