> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# V4FNMADDSS

Multiplies the low 32 bits of the first source operand by the low 32 bits of the second source operand, subtracts the result from the low 32 bits of the third source operand, and stores the result in the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| reg, reg, m4 | reg |
| reg, m4, reg | reg |
| m4, reg, reg | reg |

DO NOT support LOCK

This instruction requires the AVX instruction set. It is only available in 64-bit mode or 32-bit mode; it is NOT supported in compatibility mode.

To avoid performance penalties associated with AVX-SSE transitions, it is recommended to avoid mixing AVX instructions with legacy SSE instructions without using `VZEROUPPER`. Failure to do so MAY cause the processor to incur a transition penalty when transitioning from AVX state to SSE state.