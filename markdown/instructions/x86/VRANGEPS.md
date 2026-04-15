> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRANGEPS

Compares each packed single-precision floating-point value in the first source operand against the lower and upper bounds specified in the second source operand. For each element, if the value is greater than or equal to the lower bound and less than or equal to the upper bound, the corresponding bit in the destination mask is set to 1; otherwise, it is set to 0.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm | opmask |
| zmm/ymm/xmm | opmask |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the AVX-512 foundation.

The destination is an opmask register; failure to specify a valid mask register will result in an invalid opcode exception. This instruction does not modify the floating-point status register (MXCSR) and does not trigger floating-point exceptions such as #P, #O, or #U. Precision and rounding modes defined in MXCSR do not affect the comparison result.