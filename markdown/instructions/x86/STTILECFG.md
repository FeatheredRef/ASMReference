> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# STTILECFG

Stores the Tile Configuration value from a specified register into a specified memory location.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r64 | m8 |
| #I | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It is NOT supported in compatibility mode.

The memory destination MUST be aligned to an 8-byte boundary; otherwise, a general-protection exception (#GP) SHALL be generated. Ensure that the source register contains a valid tile configuration value as defined by the AMX (Advanced Matrix Extensions) specification to avoid unpredictable behavior in subsequent tile operations.