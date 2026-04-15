> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# STR

Stores the Segment Descriptor Cache of the specified segment register into the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | m32 |
| reg | #I |
| #I | #I |

DO NOT support LOCK

This instruction is not supported in 64-bit mode. It is only available when the processor is operating in compatibility mode.

The destination must be a memory operand; attempting to use a register as a destination will result in an invalid operation. Because this instruction accesses the hidden portion of the segment register (the descriptor cache), the destination memory must be aligned to a dword boundary to avoid potential alignment check exceptions.