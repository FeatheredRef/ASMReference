> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPGATHERQQ

Gathers 64-bit signed integer values from memory using a base address and a vector of 64-bit signed offsets. For each element in the index vector, the instruction calculates an address by adding the base address to the scaled index, loads the 64-bit value from that address, and stores it in the destination vector. A mask register tracks which elements are processed; bits set to 0 in the mask prevent the corresponding load and leave the destination element unchanged.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg, m64 | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It requires AVX2 and AVX-512 support. If the index vector is not properly aligned or the mask is not handled correctly, it may result in general protection faults (#GP).

Faulting addresses MUST be handled by the mask register; the instruction SHALL NOT trigger a fault for elements where the mask bit is 0. Users SHOULD ensure that the memory regions accessed by the calculated addresses are valid to avoid segmentation faults. Memory ordering is not guaranteed across the gathered elements.