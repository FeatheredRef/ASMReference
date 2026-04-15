> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSCATTERQD

VPSCATTERQD scatters 64-bit quadword elements from a zmm register to memory locations specified by a vector of 64-bit indices. For each active element in the mask, the instruction calculates the destination address by adding the scaled index to a base address and stores the corresponding quadword from the source register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| zmm / m512 | m64 |

DO NOT support LOCK

This instruction requires AVX-512 foundation support. It SHALL only operate in 64-bit mode or compatibility mode.

Masking is REQUIRED for this instruction; if the mask bit for an element is 0, the corresponding memory location is not accessed. This prevents unnecessary memory faults.

The instruction DOES NOT guarantee the order of stores; multiple indices pointing to the same memory address SHALL result in undefined behavior regarding which element is stored last. To ensure deterministic behavior when duplicate indices are present, the software MUST handle collisions manually. Memory disambiguation and potential alignment issues may affect performance, as the operation performs multiple non-contiguous stores.