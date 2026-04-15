> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMT2PD

Permutes two double-precision floating-point values from two sources based on an immediate. It selects two double-precision values from two 128-bit source operands and writes them to the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg / m128 | xmm reg |
| xmm reg / m128 | xmm reg |
| imm | - |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires AVX support.

The immediate serves as an index to select the specific double-precision values: bits 0-1 select the first element and bits 2-3 select the second element. An index of 0 or 1 refers to the first source, while 2 or 3 refers to the second source. Using an immediate value outside the range of 0-15 is not possible as the immediate is encoded as a 4-bit value; however, the architectural mapping only utilizes 4 bits. Failure to align the m128 memory operand to a 16-byte boundary may result in performance degradation or a general protection fault depending on the alignment check setting.