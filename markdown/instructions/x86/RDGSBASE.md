> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RDGSBASE

Reads the GS base address from the GS base MSR and stores it in the specified register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| GS base MSR | r64 |
| #I | imm |
| #I | m64 |

DO NOT support LOCK

This instruction is only available if the `FSGSBASE` feature flag is enabled in the processor. It operates in 64-bit mode.

The instruction is not available in compatibility mode. If executed in a processor that does not support `FSGSBASE`, it will trigger an invalid opcode exception (#UD).