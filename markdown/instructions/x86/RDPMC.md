> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RDPMC

Reads a performance-monitoring counter into a register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | r64 |
| imm | #I |
| m64 | #I |

DO NOT support LOCK

The instruction is only available in 64-bit mode. If executed in compatibility mode or 32-bit mode, it shall trigger an invalid opcode exception.

The `RDPMC` instruction is disabled by default. To enable its use in user-mode (CPL=3), the `CR4.PME` (Performance Monitoring Enable) bit MUST be set to 1. Additionally, the `IA32_PERF_GLOBAL_CTRL` register MUST be configured to allow user-mode access to the specific counter being read. If the `CR4.PME` bit is 0 or the appropriate permissions are not set, the instruction SHALL cause a general-protection exception (#GP).