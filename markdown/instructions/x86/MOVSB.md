> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVSB

Copies a byte from the memory location pointed to by RSI to the memory location pointed to by RDI, then increments or decrements RSI and RDI by 1 based on the Direction Flag (DF).

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m1 | m1 |

DO NOT support LOCK

The instruction SHALL be used in conjunction with the DF flag; if DF=0, RSI and RDI are incremented, if DF=1, they are decremented. It is available in 64-bit mode and compatibility mode.

To avoid unintended memory corruption or infinite loops when using the `REP` prefix, the user MUST ensure that the RCX register is initialized with the correct count and that the memory regions pointed to by RSI and RDI do not overlap in a manner that violates the intended copy logic.