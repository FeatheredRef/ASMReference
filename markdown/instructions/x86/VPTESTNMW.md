> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPTESTNMW

Performs a bitwise AND operation between two packed vectors and sets the ZF flag in the EFLAGS register if the result is zero. Unlike VPTEST, this instruction specifically tests for a "non-zero" result by setting the ZF flag to 0 if any bit in the resulting AND operation is 1, and setting ZF to 1 if the result is all zeros.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| zmm / m512 | k-reg |
| zmm / m512 | k-reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 foundation extensions to be enabled.

The instruction is subject to alignment requirements for memory operands; accessing memory that is not aligned to the operand size MAY result in a general protection exception (#GP) if alignment checking is enabled. When using zmm registers, ensure the processor supports the EVEX encoding to avoid illegal instruction exceptions.