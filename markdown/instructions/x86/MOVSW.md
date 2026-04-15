> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVSW

Copies a word from the memory location pointed to by RSI to the memory location pointed to by RDI, then increments or decrements RSI and RDI by 2 depending on the direction flag (DF).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m2 | m2 |

DO NOT support LOCK

The instruction operates using the RSI and RDI registers as implicit pointers. It is available in 64-bit mode and compatibility mode. The behavior is governed by the Direction Flag (DF) in the RFLAGS register; if DF=0, pointers are incremented; if DF=1, pointers are decremented.

The instruction performs a memory-to-memory copy, which is not permitted for general-purpose data movement instructions. To avoid unintended behavior, the user MUST ensure the DF is explicitly set using CLD or STD before execution. Failure to account for the DF state will result in incorrect memory addressing.