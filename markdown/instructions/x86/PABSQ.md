> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PABSQ

PABSQ packs the lower 64 bits of a 128-bit XMM register into a general-purpose register. It extracts the low quadword from the source XMM register and stores it in the destination general-purpose register.

The table below covers the supported source and destination operands.

| Source | Destination(s) |
| :--- | :--- |
| xmm | r64 |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode or 32-bit mode.

To avoid data loss or unexpected behavior, ensure the source XMM register contains the intended 64-bit value in its lower bits, as the upper 64 bits of the XMM register are ignored during the operation.