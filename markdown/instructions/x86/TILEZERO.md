> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TILEZERO

Sets all elements of a specified tile to zero.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |
| #I | r64 |

DO NOT support LOCK

The instruction IS ONLY available when the processor supports the AMX (Advanced Matrix Extensions) feature set. It MUST be executed in 64-bit mode.

The instruction requires the tile register to be configured via `TCONFIG` before use; otherwise, the operation may result in undefined behavior or a General Protection fault depending on the state of the AMX configuration. Ensure that the `TILEZERO` instruction targets a valid tile register defined in the current tile configuration to avoid architectural exceptions.