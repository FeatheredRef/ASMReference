> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CLDEMOTE

CLDEMOTE provides a hint to the processor that the cache line containing the specified linear address is no longer needed. The processor may evict the cache line from all levels of the cache hierarchy to make room for other data.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m64 | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is not supported in compatibility mode. The instruction is treated as a NOP if the processor does not support the CLDEMOTE feature.

The instruction serves as a hint only; the processor SHALL NOT guarantee that the cache line is actually evicted. To ensure data is written back to main memory, CLFLUSH or CLFLUSHOPT SHALL be used. Failure to synchronize memory using appropriate fencing instructions (such as SFENCE) after cache operations may lead to memory inconsistency in multiprocessor systems.