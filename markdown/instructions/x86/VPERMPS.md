> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMPS

Permutes single-precision floating-point values from a source vector to a destination vector according to an index specified by a second source vector.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m32/m64/m128/m256/m512 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or 32-bit mode. It is NOT supported in compatibility mode if the AVX-512 extensions are not enabled.

When using the EVEX encoding, the instruction supports masking. If the mask bit is 0, the corresponding destination element SHALL be merged or zeroed based on the masking policy.

To avoid undefined behavior or unexpected results, ensure that the index values in the second source register are within the range of the number of elements in the source vector. Indices that exceed the vector length SHALL be ignored, and the corresponding destination element SHALL be set to zero.