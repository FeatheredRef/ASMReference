> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVDQA

Moves aligned double-quadword data from a source to a destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm register | xmm register |
| m16 | xmm register |
| xmm register | m16 |

DO NOT support LOCK

The memory operand MUST be aligned on a 16-byte boundary. If the linear address of the memory operand is not 16-byte aligned, the processor SHALL generate a general-protection exception (#GP). This instruction is available in 64-bit mode and compatibility mode.

Failure to ensure 16-byte alignment of the memory address will result in an immediate application crash via #GP. Use `MOVDQU` if the alignment of the memory region cannot be guaranteed at compile-time or runtime.