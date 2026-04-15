> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSRAVD

Shifts the signed doubleword elements in the source operand to the right by the count specified by the second source operand. The sign bit is replicated to fill the vacated leftmost positions of each element. The result is stored in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| ymm / m128 | ymm |
| imm / xmm | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX CPU feature to be enabled.

The shift count is masked to 5 bits (0-31). If the shift count is greater than or equal to 32, the result for those elements is defined as zero, except for the sign bit which is preserved based on the original value's sign. Use of this instruction in 32-bit mode is prohibited.