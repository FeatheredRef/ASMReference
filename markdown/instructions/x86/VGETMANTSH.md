> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGETMANTSH

Extracts the significand (mantissa) from a packed double-precision floating-point value in a YMM register and shifts it left by the specified immediate value, storing the result in a YMM register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| ymm | ymm |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or compatibility mode. It REQUIRES the AVX instruction set to be enabled in the processor.

To avoid incorrect result interpretation, ensure that the immediate value is within the valid range (0 to 63). Providing an immediate value outside this range MAY result in undefined behavior or an invalid operation. Since this instruction operates on packed double-precision values, it will process four 64-bit elements simultaneously; failure to align the source YMM register to a 32-byte boundary when loading from memory prior to this operation MAY cause performance degradation or general protection faults depending on the memory access mode.