> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INSD

Reads a double word from the memory location pointed to by the ECX register (or EDI/ESI depending on the direction flag) and stores it into the AX register, then increments or decrements the pointer.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m4 | r32 |

DO NOT support LOCK

This instruction is available in both 64-bit mode and compatibility mode. When operating in 64-bit mode, the instruction uses the 32-bit ECX register (or EDI/ESI) for the address, meaning it accesses the lower 4GB of the address space unless the address is treated as an offset from a base.

The Direction Flag (DF) MUST be checked to determine the pointer update: if DF=0, the pointer is incremented by 4; if DF=1, the pointer is decremented by 4. Failure to initialize the DF via `CLC` or `STD` will result in unpredictable memory access patterns.