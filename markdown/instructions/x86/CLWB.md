> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CLWB

Writes a modified cache line back to main memory without changing the cache line's state (it remains in the Modified state if it was already modified), provided the line is present in the cache.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| #I | m64 |

DO NOT support LOCK

The instruction is only available in 64-bit mode. It is not supported in compatibility mode.

CLWB is a hint to the processor; the hardware MAY ignore the request to write back the cache line. To ensure the data is persisted to the memory controller, a SFENCE instruction SHALL be executed after CLWB. This instruction does not evict the line from the cache, unlike CLFLUSH.