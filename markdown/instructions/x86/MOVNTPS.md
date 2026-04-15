> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVNTPS

Moves packed single-precision floating-point values from an XMM register to memory without using the cache, utilizing a non-temporal hint to minimize cache pollution.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | m128 |

DO NOT support LOCK

The destination memory address MUST be aligned on a 16-byte boundary; otherwise, a general-protection exception (#GP) is generated. This instruction is available in 64-bit mode and 32-bit mode.

The instruction performs a non-temporal store, which bypasses the cache hierarchy. This behavior means that the store is not coherent with other cached copies of the data until the store is flushed from the write-combining buffers. To ensure that the data is visible to other processors or devices, a `SFENCE` instruction SHOULD be executed. Failure to use a memory fence may result in memory ordering violations.