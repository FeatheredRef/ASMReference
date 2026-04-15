> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSRLVD

Shifts the unsigned doublewords in the destination vector right by the number of bits specified by the corresponding doubleword in the source vector. The result is stored in the destination vector.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m128/m256/m512 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. If executed in compatibility mode, it will trigger an invalid opcode exception.

The shift count is masked to 31 bits; any bits beyond the 5th bit of the shift count are ignored. If the shift count is greater than or equal to 32, the result is zero. Ensure that the input vector is correctly aligned to the corresponding vector length to avoid general protection faults.