> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TILERELEASE

Releases the tile registers specified by the immediate value, allowing the hardware to reclaim the resources associated with those tiles. This instruction ensures that any pending tile operations are completed before the release occurs.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |

DO NOT support LOCK

This instruction is ONLY available when the processor supports AMX (Advanced Matrix Extensions) and the corresponding feature flag is enabled. It requires the processor to be in 64-bit mode; it is NOT supported in compatibility mode.

The `TILEREASE` instruction MUST be executed after the final use of a tile register set before the tile configuration is changed or the tiles are no longer needed. Failure to release tiles before certain power-state transitions or context switches MAY lead to undefined behavior or performance degradation in specific hardware implementations.