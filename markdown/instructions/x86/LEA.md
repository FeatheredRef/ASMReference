> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LEA

Calculates the effective address of the source operand using the specified addressing mode and stores the result in the destination operand without accessing the memory location.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| mN | #I |
| reg | reg |
| imm | #I |

DO NOT support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and compatibility mode. The size of the destination register MUST match the size of the address calculated; in 64-bit mode, the result is typically stored in an r64 register.

Users SHOULD NOT use LEA to perform memory accesses, as the instruction does not dereference the calculated address. When using LEA for arithmetic, be aware that it is limited to the formula: `Base + (Index * Scale) + Displacement`. The scale MUST be 1, 2, 4, or 8. Overflows in the calculation are ignored, and the result is truncated to the size of the destination register.