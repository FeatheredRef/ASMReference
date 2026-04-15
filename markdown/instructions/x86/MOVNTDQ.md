> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVNTDQ

Moves 128 bits of data from the source to the destination using a non-temporal store, bypassing the cache hierarchy to minimize cache pollution.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | m16 |
| m16 | xmm |

DO NOT support LOCK

The destination memory address MUST be aligned on a 16-byte boundary. If the address is not aligned, the instruction will trigger a general protection exception (#GP). This instruction is available in 64-bit mode and compatibility mode.

To ensure data consistency and visibility to other processors or devices, a `SFENCE` instruction SHOULD be executed after `MOVNTDQ` stores. Because this instruction uses write-combining, stores are not guaranteed to be committed to memory in program order. Failure to use a memory fence MAY lead to data corruption or stale reads in multi-threaded environments.