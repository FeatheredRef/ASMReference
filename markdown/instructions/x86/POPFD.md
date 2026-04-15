> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# POPFD

Pops the top word from the stack and loads it into the EFLAGS register. The stack pointer (RSP) is incremented by 2 bytes after the operation.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m2 | EFLAGS |

DO NOT support LOCK

This instruction is only available in 32-bit mode or in 64-bit mode when operating in compatibility mode. In 64-bit mode, this instruction is not supported; POPRQ must be used to manipulate the RFLAGS register.

The POPFD instruction does not affect the IF (Interrupt Enable Flag), the NT (Nested Task) flag, or the IOPL (I/O Privilege Level) field. Attempting to modify these protected flags via POPFD will result in those specific bits remaining unchanged in the EFLAGS register.