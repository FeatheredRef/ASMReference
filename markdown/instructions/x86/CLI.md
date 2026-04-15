> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CLI

Clears the Clear Interrupt Flag (IF) in the EFLAGS register, masking maskable external interrupts.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | EFLAGS |

DO NOT support LOCK

This instruction is available in 64-bit mode, compatibility mode, and 32-bit mode. In 64-bit mode, the instruction operates on the RFLAGS register.

The instruction is a privileged operation if the processor is operating in a mode where the CPL (Current Privilege Level) is greater than the IOPL (I/O Privilege Level). If executed at an insufficient privilege level, the processor SHALL generate a General Protection exception (#GP).