> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPGATHERDQ

Gathers 64-bit signed integers from memory using a base address and a vector of 32-bit signed indices. For each element, the instruction calculates the address as `base + (index * 8)` and loads the resulting qword into the destination vector. A mask register is used to track which elements have been successfully loaded; bits in the mask are cleared upon successful completion of the corresponding load.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r64 | zmm/ymm |
| m8 | zmm/ymm |
| k-register | k-register |

DO NOT support LOCK

This instruction SHALL ONLY be executed in 64-bit mode. It is NOT supported in compatibility mode.

The mask register SHALL be used to control the gathering process. If a bit in the mask is 0, the corresponding element in the destination register is NOT updated. Because the mask is updated atomically relative to each element load, the instruction is restartable; if a fault occurs, the mask reflects which elements were already processed.

Memory accesses SHALL be aligned to the requirements of the data type. However, if the memory access crosses a page boundary, it MAY trigger a page fault. Users MUST ensure that the base address and indices do not result in a memory address outside the canonical address range to avoid a General Protection exception (#GP).