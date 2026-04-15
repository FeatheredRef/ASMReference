> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LDTILECFG

Loads the tile configuration from a memory location into the internal tile configuration registers.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m64 | Internal Tile Configuration Registers |

DO NOT support LOCK

This instruction is only available when the processor supports AMX (Advanced Matrix Extensions). It SHALL only be executed in CPL 0 or when the `CR0.AMX` bit is set to 1, otherwise, it SHALL trigger a General Protection Exception (#GP).

The memory location pointed to by the source operand SHALL be aligned on a 64-byte boundary; failure to do so SHALL result in an alignment check exception (#AC) if alignment check is enabled, or a general protection exception (#GP). Users SHALL ensure the memory region contains a valid `tilecfg` structure to avoid undefined behavior in subsequent `TDPBSSD` or `TDPBSSW` operations.