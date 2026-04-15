> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPCMPUQ

Compares two unsigned quadword integers from the first source operand and the second source operand and stores the result in the destination operand. The comparison is performed according to the condition specified by the immediate byte.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm | zmm/ymm/xmm |
| m64 | zmm/ymm/xmm |
| imm | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires a processor supporting the AVX-512 foundation.

The immediate byte determines the comparison predicate. Using an unsupported immediate value will result in an invalid opcode exception. When operating on YMM or XMM registers, the upper bits of the ZMM register are zeroed. The operation is performed element-wise; the number of elements processed depends on the vector length (k-masking may be applied if the EVEX prefix is used).