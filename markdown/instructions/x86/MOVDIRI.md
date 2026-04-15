> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVDIRI

Moves the immediate value into the specified directory entry.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | m8 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The instruction MUST be used with the `CLFLUSH` or `CLFLUSHOPT` instructions to ensure visibility of the write to other processors, as the write is not guaranteed to be coherent across all caches immediately. The memory operand MUST be aligned to the size of the directory entry being written to avoid alignment faults.