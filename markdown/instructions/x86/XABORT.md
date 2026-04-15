> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XABORT

Aborts a transactional memory region, causing the current transaction to terminate and roll back all architectural changes to the state prior to the `XBEGIN` instruction.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |

DO NOT support LOCK

XABORT is only available in 64-bit mode. It is not supported in compatibility mode.

The instruction requires the processor to support Intel® TSX (Transactional Synchronization Extensions). If the processor does not support TSX or if the feature is disabled in the BIOS/OS, executing XABORT may result in an invalid opcode exception.

The immediate value provided to XABORT is used as the abort status code. This value is returned by the `XBEGIN` instruction to the fallback handler, allowing the software to determine the specific reason for the transactional abort. Use a unique `u32` value for different abort conditions to avoid ambiguity during error recovery.