> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KXNORQ

Performs a bitwise XNOR operation on the source and destination operands. The operation is defined as the logical negation of the XOR result: the resulting bit is 1 if both corresponding bits are the same, and 0 if they are different.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r64 | r64 |
| m64 | r64 |
| r64 | m64 |
| imm | r64 |
| imm | m64 |

DO NOT support LOCK

This instruction is ONLY available when the processor is operating in 64-bit mode or compatibility mode. It SHALL NOT be executed in 32-bit protected mode.

Ensure that the immediate value provided is treated as a 64-bit sign-extended value to avoid unexpected bit patterns in the upper 32 bits of the r64 register or m64 memory location. Failure to account for sign extension MAY lead to incorrect XNOR results when operating on values where the 31st bit is set.