> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CDQ

Extends a dword register into a qword value across two registers. It copies the most significant bit (sign bit) of the source register into the destination register to perform sign-extension.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r32 | eax, edx |
| imm | #I |
| m4 | #I |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It specifically targets the `eax` register as the source and utilizes `edx` as the implicit destination for the upper 32 bits of the resulting sign-extended value.

The instruction is essential for preparing the dividend for the `IDIV` instruction. Failure to use `CDQ` (or `CQO` for 64-bit registers) before `IDIV` will result in an incorrect quotient or a #Z / #O exception because `IDIV` expects the dividend to be spread across `edx:eax` (for 32-bit) or `rdx:rax` (for 64-bit).