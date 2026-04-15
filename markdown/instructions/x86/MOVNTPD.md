> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVNTPD

Moves a packed double-precision floating-point value from a register to a memory location using non-temporal hints, which bypasses the cache hierarchy to minimize cache pollution.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | m16 |

DO NOT support LOCK

This instruction SHALL only be used when the destination memory address is aligned on a 16-byte boundary. If the address is not aligned, an exception may occur or performance may be severely degraded depending on the processor implementation.

The use of non-temporal stores implies that the data is not expected to be reused soon. Consequently, this instruction DOES NOT maintain memory ordering with respect to other stores. A `SFENCE` instruction SHALL be executed to ensure that all non-temporal stores are globally visible before subsequent memory operations that depend on the stored data. Failure to use a memory fence may result in data inconsistency in multi-processor systems.