> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPTESTNMD

Performs a logical AND operation between the first source operand and the second source operand, then tests the result against zero. If the result of the AND operation is non-zero, the Carry Flag (CF) is cleared; otherwise, the Carry Flag (CF) is set. The Zero Flag (ZF) is updated based on the result of the operation.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | None |
| m128/m256/m512 | None |

DO NOT support LOCK

This instruction is only available in 64-bit mode. In compatibility mode, attempting to execute this instruction will result in an invalid opcode exception.

The instruction uses the mask register (k-register) if the EVEX prefix is present to control which elements of the vector registers are processed. If a mask is used and the mask bit for a specific element is 0, the corresponding element in the source operands is not accessed, and the result for that element does not affect the flags. Ensure that the appropriate mask register is initialized to avoid unexpected behavior in the conditional flags.