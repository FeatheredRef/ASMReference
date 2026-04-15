> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVNTQ

Moves a quad word from the source to the destination using a non-temporal store, bypassing the cache hierarchy to avoid polluting the cache when the data is not expected to be reused soon.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r64 | m8 |
| m8 | #I |
| imm | #I |

DO NOT support LOCK

This instruction SHALL only be used in 64-bit mode or compatibility mode. It REQUIRES the destination operand to be aligned on an 8-byte boundary; otherwise, a general-protection exception (#GP) MAY occur.

The use of `MOVNTQ` results in a write-combining store. Because it bypasses the cache, the store is not immediately visible to other processors. A programmer SHALL execute an `SFENCE` instruction to ensure that the non-temporal stores are globally visible before subsequent operations depend on them. Failure to use a memory fence MAY lead to data inconsistency in multi-core environments.