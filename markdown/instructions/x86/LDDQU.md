> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LDDQU

Loads a 64-bit value from a memory location into a XMM register, treating the memory address as potentially unaligned, and performs no alignment check.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | xmm |
| reg | #I |
| imm | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is not supported in compatibility mode.

The memory operand MUST be a qword. Because this instruction is specifically designed for unaligned loads, it does not trigger a general-protection exception (#GP) if the address is not aligned to an 8-byte boundary. Users SHOULD use LDDQU instead of MOVQ (XMM, m64) when the memory alignment of the source data cannot be guaranteed at compile time to avoid potential alignment faults on certain hardware configurations.