> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FSTCW

Stores the FPU Control Word into the specified destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| FPU Control Word | m16 |
| FPU Control Word | r16 |

DO NOT support LOCK

This instruction is available in 64-bit mode, but it operates on the x87 FPU state. In 64-bit mode, the x87 FPU is supported for compatibility.

The destination MUST be a 16-bit register or memory location. Using a larger destination size (e.g., dword) WILL NOT affect the remaining bytes of the destination, but only the lowest 16 bits are overwritten by the FPU Control Word.