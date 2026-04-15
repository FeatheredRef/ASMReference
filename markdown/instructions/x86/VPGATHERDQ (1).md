> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPGATHERDQ (1)

Gathers 64-bit signed integer values from memory using a base address and a set of indices provided in a vector register. For each element in the index vector, the instruction calculates the effective address by adding the base address to the index scaled by 8, then loads the 64-bit value from that address into the destination vector register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| r64, zmm / ymm / xmm | zmm / ymm / xmm |
| m64 | zmm / ymm / xmm |

DO NOT support LOCK

The instruction REQUIRES AVX2 and AVX-512 support. It operates in 64-bit mode or compatibility mode. If the index vector contains values that result in an out-of-bounds memory access, the behavior is defined by the paging mechanism; however, the instruction utilizes a mask register to track completed loads.

The mask register is updated upon completion of each element load. If a mask bit is 0, the corresponding load is skipped. If a fault occurs during a load, the mask bit remains 0 for that element, and the exception is deferred until all previous elements are processed. To avoid infinite loops or hangs in software, the user MUST ensure the mask register is correctly initialized and that the base address plus scaled index does not exceed the canonical address range. Improper handling of the mask register will result in the instruction not loading all intended elements.