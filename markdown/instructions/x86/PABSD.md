> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PABSD

PABSD packs the most significant 32 bits of the source operand into the least significant 32 bits of the destination operand, and clears the most significant 32 bits of the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r64 | r64 |
| m8 | r64 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The destination register MUST be a 64-bit register. Using this instruction in a context where the register size is restricted to 32 bits will result in incorrect behavior or an invalid opcode exception depending on the environment. Since the upper 32 bits of the destination are zeroed, the result is always an unsigned 32-bit value stored in a 64-bit register.