> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FXTRACT

Extracts a specified number of bits from a floating-point value and stores the result as an integer in a destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f80 | reg/m32 |

DO NOT support LOCK

This instruction is available only in compatibility mode. It operates exclusively on the ST(0) register of the x87 FPU stack.

The destination MUST be a 32-bit register or memory location. If the extracted value exceeds the capacity of a dword, the behavior is undefined. The bit-field specification is provided via an immediate value or a register, defining the starting bit position and the length of the field to be extracted.