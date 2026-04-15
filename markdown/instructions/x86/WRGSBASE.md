> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# WRGSBASE

Writes a value to the GS base register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | GS base |
| imm | #I |
| mN | #I |

DO NOT support LOCK

This instruction is only available if the `GS_BASE` feature is supported by the processor and the `IA32_GS_BASE` MSR is configured to allow the use of `WRGSBASE`. It is supported in 64-bit mode.

The `WRGSBASE` instruction is ignored if the `IA32_GS_BASE.WRGSBASE` bit is not set to 1. In such cases, the instruction is treated as a `NOP`, and the GS base register remains unchanged. This ensures backward compatibility with software that does not utilize this feature.