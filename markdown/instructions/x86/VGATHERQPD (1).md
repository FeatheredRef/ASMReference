> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERQPD (1)

Gathers 64-bit floating-point values from memory using a vector of 64-bit indices. For each element in the destination register, the instruction calculates a memory address by adding the scaled index from the index register to a specified base address, then loads the 64-bit floating-point value into the destination. A mask register is used to determine which elements are loaded; elements with a mask bit set to 0 are skipped and their corresponding destination elements remain unchanged.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m64 | f512/f256 |
| reg (index) | f512/f256 |
| reg (mask) | f512/f256 |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It REQUIRES AVX2 or AVX-512 support. The instruction does not support memory-to-memory operations.

Faults during the gather operation are handled by the mask register. If a memory fault occurs for an element whose mask bit is 0, the fault SHALL NOT be reported. If a fault occurs for an element whose mask bit is 1, the instruction is interrupted and the mask bit for that element is cleared to 0 to prevent repeated faults upon resumption. Users MUST ensure the base address and scaled indices do not result in out-of-bounds memory access to avoid general protection faults.