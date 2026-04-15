> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MFENCE

The `MFENCE` instruction serializes memory operations. It ensures that every load and store operation that precedes the `MFENCE` instruction in program order is globally visible before any load or store operation that follows the `MFENCE` instruction in program order.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

`MFENCE` is supported in 32-bit and 64-bit mode. It is not available in 16-bit real address mode.

To avoid memory consistency errors in multi-processor environments, `MFENCE` SHALL be used when a program MUST guarantee that previous writes are visible to other processors before subsequent reads or writes are executed. Note that `MFENCE` is more restrictive than `SFENCE` (which only orders stores) or `LFENCE` (which only orders loads). Use of `MFENCE` may impact performance due to the complete pipeline stall required to ensure global visibility of all pending memory operations.