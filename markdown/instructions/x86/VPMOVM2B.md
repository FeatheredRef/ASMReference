> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVM2B

Moves data from a xmm register to a general-purpose register. Specifically, it extracts the lowest byte of the lowest 64-bit element of the source xmm register and stores it into the destination general-purpose register, with the upper bits of the destination register being zeroed.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | r8 |
| #I | m8 |
| #I | imm |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It cannot be used in compatibility mode.

The destination register is treated as a 64-bit register (r64); while only the lowest 8 bits are populated from the source, the remaining 56 bits are cleared to zero. Failure to account for the zero-extension of the destination register may lead to incorrect results in subsequent arithmetic operations.