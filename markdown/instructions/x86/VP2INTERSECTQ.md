> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VP2INTERSECTQ

Computes the intersection of two sets of 64-bit integers stored in the source operands. The instruction identifies elements present in both input vectors and stores the resulting intersection in the destination vector.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| memory | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX-512 support and specific feature flags depending on the implementation (e.g., AVX-512BW or AVX-512DQ).

The destination register must not overlap with the source registers to avoid undefined behavior. This instruction utilizes opmask registers for conditional updates; failure to properly set the mask register may result in unexpected elements being retained in the destination register from previous operations.