> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPTESTNMQ

Performs a bitwise logical AND operation between two 512-bit YMM/ZMM registers or a register and a memory operand, and sets the flags based on the result. If the result of the AND operation is non-zero, the Zero Flag (ZF) is cleared; otherwise, it is set.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| zmm | r64 (flags) |
| m64 | r64 (flags) |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 foundation extensions to be enabled.

The instruction updates the RFLAGS register; specifically, it modifies the ZF flag. Users MUST ensure that the target memory operand is aligned to 64 bytes to avoid potential performance penalties or general protection faults depending on the alignment check settings.