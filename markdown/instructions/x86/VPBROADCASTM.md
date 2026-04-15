> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPBROADCASTM

Broadcasts a doubleword value from a memory location to all doubleword elements of the destination vector register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m4 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction requires AVX support. It is not available in compatibility mode; it MUST be executed in 64-bit mode or protected mode.

The destination register is overwritten by the broadcasted value. If the memory operand is not aligned to a 4-byte boundary, a general-protection exception (#GP) MAY occur depending on the alignment check (AC) flag in the EFLAGS register. To avoid performance degradation or faults, ensure the source m4 is naturally aligned.