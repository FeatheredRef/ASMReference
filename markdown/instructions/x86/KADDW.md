> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KADDW

Adds two 16-bit signed integers from the source and destination operands and stores the result in the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It operates on the lower 16 bits of the specified registers.

The destination register must be a 64-bit register (r64) or a 32-bit register (r32) depending on the specific encoding, but the operation only affects the lowest 16 bits (word). Users MUST ensure that the operands are treated as signed integers (i16) to avoid logic errors, although the bitwise addition remains identical to unsigned addition.