> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TILELOADD

Loads a tile from memory into a tile register.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| mN | reg |

DO NOT support LOCK

This instruction is only available when the processor supports the AMX (Advanced Matrix Extensions) feature set and is executing in 64-bit mode. It is NOT supported in compatibility mode.

The instruction SHALL be used only when the tile configuration has been previously initialized via `TILECFG`. If the tile configuration is invalid or not set, the instruction MAY trigger a general protection exception (#GP). Ensure that the memory address is aligned to the required boundary for the specific tile data type to avoid alignment check exceptions.