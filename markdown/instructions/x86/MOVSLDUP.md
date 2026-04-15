> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVSLDUP

Copies a doubleword value from a source to two packed doublewords in a destination. Specifically, it reads a dword from the source and duplicates it across the low and high dwords of a 64-bit destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m4 | xmm |
| r32 | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the destination operand to be an XMM register.

To avoid alignment exceptions, the memory source m4 SHOULD be aligned on a 4-byte boundary. Failure to do so MAY result in performance degradation or a general protection fault depending on the alignment check (AC) flag in the EFLAGS register.