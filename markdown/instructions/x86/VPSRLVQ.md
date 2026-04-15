> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSRLVQ

Shifts the unsigned 64-bit integers in the destination vector by the number of bits specified in the source vector. The shift count is taken from the low 6 bits of the source element, and the result is stored in the destination vector.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| rN | rN |
| m8 | rN |

DO NOT support LOCK

This instruction requires AVX-512 support. It is only available in 64-bit mode.

The shift count is masked to 6 bits (0-63); any bits beyond the 6th bit in the source register/memory operand are ignored. Failure to ensure the shift count is within the 0-63 range may lead to unexpected behavior if the programmer assumes a larger shift is possible. This instruction operates on zmm registers; using it with smaller vector registers (ymm/xmm) is not supported by this specific mnemonic.