> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RDSSPQ

Reads the model-specific register (MSR) associated with the current processor's state and stores the result into the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | r64 |
| #I | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It SHALL NOT be executed in compatibility mode or 32-bit protected mode.

The destination register MUST be a 64-bit general-purpose register. Attempting to use a 32-bit register as the destination will result in an invalid operation. Because this instruction accesses MSRs, it SHALL only be executed with CPL=0; execution at a higher privilege level SHALL trigger a General Protection exception (#GP).