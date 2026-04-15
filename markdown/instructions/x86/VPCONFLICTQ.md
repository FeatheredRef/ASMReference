> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPCONFLICTQ

VPCONFLICTQ detects conflicts between quadword elements within two zmm registers. For each element in the first source operand, the instruction compares it against all subsequent elements in the same register and the second source operand. It produces a mask identifying which elements are duplicate values.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| zmm reg | zmm reg |
| zmm reg | zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 Foundation (AVX-512F) instruction set.

The destination register is overwritten; therefore, the destination must not be used as a source operand if the original data is required. The instruction uses the `k` mask registers for conditional execution if a mask is specified; if the mask bit is 0, the corresponding element in the destination register is either zeroed or left unchanged depending on the masking mode (zeroing or merging).