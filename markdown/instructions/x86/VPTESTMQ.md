> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPTESTMQ

Performs a bitwise logical AND between two qword-sized elements of the source operands. If the result of the AND operation is non-zero, the corresponding bit in the destination register is set to 1; otherwise, it is set to 0.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm / m64 | kx |
| zmm / m64 | kx |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 foundation.

The instruction updates the mask register `kx`. If the destination mask register is specified as a general-purpose register, the result is written to the lower bits of that register. Ensure the target mask register is correctly sized to avoid unintended data loss or incorrect conditional branching.