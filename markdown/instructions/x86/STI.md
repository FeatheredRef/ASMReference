> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# STI

Sets the IF (Interrupt Flag) bit in the RFLAGS register to 1, enabling maskable hardware interrupts.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | RFLAGS |

DO NOT support LOCK

The instruction is available in 64-bit mode, compatibility mode, and 32-bit protected mode. It does not require any specific privilege level to execute in real mode, but in protected mode or 64-bit mode, it is a privileged instruction and SHALL only be executed by code running at CPL 0.

Execution of STI is not immediate; it is delayed until the instruction following the STI has been executed. This prevents a race condition where an interrupt could occur between the execution of a CLI and the subsequent instruction that requires atomicity.