> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CLD

Clears the Direction Flag (DF) to 0.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is available in 16-bit, 32-bit, and 64-bit operand size modes. It does not have architectural constraints regarding compatibility mode.

When DF is 0, string instructions (such as MOVS, SCAS, LODS, and STOS) increment the index registers (RSI/RDI) after each iteration. Failure to execute CLD before a string operation that expects an upward increment SHALL result in the index registers being decremented, leading to incorrect memory access patterns.