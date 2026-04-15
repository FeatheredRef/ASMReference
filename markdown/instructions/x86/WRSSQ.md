> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# WRSSQ

Writes the current value of the Time Stamp Counter (TSC) to the specified destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal TSC | r64 |
| Internal TSC | m8 |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It SHALL NOT be used in compatibility mode or 32-bit protected mode.

The destination operand MUST be 64 bits wide. If a memory destination is used, it MUST be aligned to an 8-byte boundary to avoid potential performance penalties or alignment checks. Writing the TSC to a register (r64) is the preferred method to ensure atomicity of the read operation.