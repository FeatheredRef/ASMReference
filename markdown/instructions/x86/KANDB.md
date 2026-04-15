> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KANDB

Performs a bitwise logical AND operation between a byte-sized source operand and a byte-sized destination operand, storing the result in the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r8 | r8 |
| m1 | r8 |
| imm | r8 |
| r8 | m1 |
| m1 | m1 |
| imm | m1 |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode or compatibility mode when operating on 8-bit registers/memory.

The ZF flag is set if the result is 0; otherwise, it is cleared. The CF, OF, SF, and PF flags are undefined.

To avoid data corruption, ensure the destination m1 is properly aligned to a byte boundary. Since this instruction does not support the LOCK prefix, it is NOT atomic for memory-to-memory operations.