> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# STOSQ

Stores the value contained in the RAX register into the memory location pointed to by RDI, then increments or decrements RDI by 8 bytes according to the current state of the Direction Flag (DF).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r64 (RAX) | m64 ([RDI]) |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. In 32-bit mode, it is not available; the corresponding instruction is STOSD.

The operation depends on the Direction Flag (DF). If DF = 0, RDI is incremented; if DF = 1, RDI is decremented. To avoid unintended memory corruption or buffer overflows, the programmer SHALL ensure the DF is explicitly set using `CLD` or `STD` before execution.

The instruction is typically used with the `REP` prefix to fill a block of memory. When using `REP`, the RCX register SHALL contain the count of qword elements to be stored.