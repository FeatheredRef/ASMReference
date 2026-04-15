> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# POPAD

Pops the general-purpose registers EAX, ECX, EDX, EBX, ESP, EBP, ESI, and EDI from the stack.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m32 | r32 |

DO NOT support LOCK

This instruction is only available in legacy mode or compatibility mode. It SHALL NOT be used in 64-bit mode.

The stack pointer (ESP) is incremented by 32 bytes after the operation. If the operation results in a stack overflow or alignment fault, a general-protection exception or stack-segment fault MAY occur depending on the processor mode.