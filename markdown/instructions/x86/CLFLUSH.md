> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CLFLUSH

Invalidates the cache line containing the specified linear address from all levels of the processor cache and writes any modified data back to system memory.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m64 | #I |
| reg | #I |
| imm | #I |

DO NOT support LOCK

The instruction requires a linear address. It operates regardless of the current privilege level; however, the address must be present in the page tables. If the address is not present, the instruction does not trigger a page fault but simply does not flush the cache line. It is supported in 64-bit mode, 32-bit mode, and compatibility mode.

To ensure that the data is flushed and visible to other processors or devices, the programmer SHOULD follow the `CLFLUSH` instruction with an `SFENCE` or `MFENCE` instruction, as `CLFLUSH` is not ordered with respect to other memory operations. Failure to do so MAY result in memory inconsistency in multi-processor environments.