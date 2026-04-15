> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# UNPCKLPS

Unpacks the lower single-precision floating-point numbers from two source operands. It interleaves the lowest single-precision value from the first source operand with the lowest single-precision value from the second source operand, then the second-lowest from the first source, and the second-lowest from the second source, storing the result in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m128 | reg |

DO NOT support LOCK

This instruction requires SSE support. It is available in both 64-bit mode and compatibility mode.

The instruction operates on XMM registers. If a memory operand is used, it MUST be 128-bit aligned to avoid alignment check exceptions or performance penalties. The instruction does not modify the floating-point status register (MXCSR) and does not trigger floating-point exceptions such as #D, #O, #U, or #P.