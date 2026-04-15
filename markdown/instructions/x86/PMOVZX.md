> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMOVZX

Moves a byte or word from a general-purpose register or memory location to a 128-bit XMM register, zero-extending the value to 32 bits before treating it as a packed element.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | xmm |
| m1 / m2 | xmm |
| imm | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The destination xmm register is partially modified; only the low 32 bits of the first doubleword element are updated. The remaining bits of the xmm register are preserved. Failure to account for the persistence of the upper 96 bits of the xmm register may lead to incorrect results in subsequent SIMD operations.