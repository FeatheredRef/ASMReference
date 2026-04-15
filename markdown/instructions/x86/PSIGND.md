> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSIGND

Sign-extends a signed double word (32-bit) value to a quad word (64-bit) value.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (r32) | reg (r64) |
| m4 | reg (r64) |
| imm | #I |

DO NOT support LOCK

This instruction is available in 64-bit mode. In compatibility mode, the instruction is not supported and will trigger an invalid opcode exception.

The destination register MUST be a 64-bit register to accommodate the sign-extended result of the 32-bit source. Attempting to use a 32-bit register as the destination is invalid.