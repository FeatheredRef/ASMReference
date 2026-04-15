> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# STOSD

Stores the value from the EAX register into the memory location pointed to by the EDI or RDI register and then increments or decrements the index register by 4 bytes, depending on the direction flag (DF) state.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r32 | m4 |

DO NOT support LOCK

The instruction is available in 32-bit mode and 64-bit mode (including compatibility mode). In 64-bit mode, the destination address is determined by the RDI register.

The behavior of the pointer increment/decrement is dependent on the Direction Flag (DF). If DF=0, the destination index is incremented; if DF=1, it is decremented. Failure to correctly set the DF via `CLD` or `STD` before execution SHALL result in unintended memory corruption. The instruction is typically used with the `REP` prefix to fill a block of memory.