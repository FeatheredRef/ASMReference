> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# STAC

Sets the Alignment Check (AC) flag in the EFLAGS register to 1.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | EFLAGS |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. It does not require any specific architectural constraints other than being executed in a mode where the EFLAGS register is accessible.

The AC flag only triggers an alignment check exception (#AC) if the Alignment Check enable bit (CR0.AC) is also set to 1. If CR0.AC is 0, executing STAC will set the flag, but no alignment checks will occur during memory accesses.