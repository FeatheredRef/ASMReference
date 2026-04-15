> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# OUTSW

Outputs the word at the memory location pointed to by rSI (or rSIndex) to the I/O port specified by rDX. The instruction then increments or decrements rSI (or rSIndex) based on the Direction Flag (DF) and decrements rCX. This process repeats until rCX is 0.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m2 | rDX |

DO NOT support LOCK

This instruction is NOT supported in 64-bit mode. It is only available in 32-bit mode and compatibility mode.

The instruction SHALL be used with rCX initialized to a non-zero value; otherwise, the operation is skipped. Because rSI/rSIndex and rCX are modified during execution, the programmer MUST ensure these registers contain the correct starting address and count to avoid memory access violations or infinite loops. If a page fault occurs during the string operation, the instruction is restartable.