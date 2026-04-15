> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVD

Moves a doubleword value from a source to a destination.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m4 | xmm reg |
| xmm reg | m4 |
| reg | xmm reg |

DO NOT support LOCK

This instruction is available in both 64-bit mode and compatibility mode. When moving data between a general-purpose register and an XMM register, the move is performed between the general-purpose register and the lowest 32 bits of the XMM register.

The upper 96 bits of the destination XMM register remain unchanged when moving a dword from a general-purpose register or memory into an XMM register. Users MUST ensure that the destination XMM register is properly initialized if the upper bits are required to be zero, as MOVD does not clear them.