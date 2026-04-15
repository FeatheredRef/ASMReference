> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMPSB

Compares the byte at the memory location pointed to by RSI with the byte at the memory location pointed to by RDI. The comparison is performed by subtracting the source byte from the destination byte, updating the CPU flags (CF, ZF, SF, AF, PF, PF) without modifying the operands.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m1 | m1 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates exclusively on the memory locations pointed to by the RSI and RDI registers.

The instruction MUST be used in conjunction with the Direction Flag (DF). If DF is 0, RSI and RDI are incremented after the comparison; if DF is 1, they are decremented. Failure to set the DF correctly via `CLD` or `STD` will result in unintended memory traversal. This instruction is typically used with the `REP` prefix to compare blocks of memory.