> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVDQ2Q

Moves a quadword from a 128-bit XMM register to a 64-bit general-purpose register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | r64 |
| imm | #I |
| m128 | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

The source XMM register is accessed such that only the lower 64 bits are moved to the destination r64 register. This operation does not affect the upper 64 bits of the XMM register. To avoid data loss, the programmer MUST ensure that the desired data is located in the low quadword of the XMM register before execution.