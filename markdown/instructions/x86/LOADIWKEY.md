> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LOADIWKEY

Loads a 64-bit value from a memory location into a specified register, typically used in the context of loading instrumentation or isolation keys.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | r64 |
| imm | #I |

DO NOT support LOCK

This instruction is ONLY available when the processor is operating in 64-bit mode. It SHALL NOT be used in compatibility mode.

The memory operand MUST be naturally aligned to an 8-byte boundary; otherwise, a General Protection exception (#GP) MAY occur depending on the processor's alignment checking configuration. Ensure the source address is valid and accessible to avoid a Page Fault (#PF).