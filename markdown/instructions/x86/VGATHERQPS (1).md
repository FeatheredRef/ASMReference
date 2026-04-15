> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERQPS (1)

Gathers packed single-precision floating-point values from non-contiguous memory locations into a destination register. It uses a base address, a vector of indices, and a mask register to determine which elements to load. Loaded values are stored in the destination register at positions corresponding to the index vector.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8 | reg |

DO NOT support LOCK

This instruction requires the AVX2 instruction set to be supported by the processor. It operates only in 64-bit mode or compatibility mode.

The mask register is updated during execution; elements that are successfully loaded have their corresponding mask bit cleared to 0. If a memory access causes a fault or is not performed due to the mask, the mask bit remains 1. Users SHALL ensure that the memory addresses calculated from the base and indices are aligned according to the requirements of the memory model to avoid performance degradation or faults. The instruction MAY exhibit variable latency depending on the number of elements to be gathered and the cache state of the memory locations.