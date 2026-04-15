> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGETEXPSS

Extracts the exponent of a single-precision floating-point value from a source and stores the result as an unsigned integer.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m32 | xmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX support feature to be enabled.

The instruction operates on single-precision floating-point values (f32). If the input is a signaling NaN, a #I exception shall be generated. If the input is a quiet NaN, no exception is generated, and the exponent is extracted based on the IEEE 754 representation. The result is stored as a u32 within the destination xmm register, with the upper 31 bits of the destination element set to zero. Failure to ensure the CPU supports the AVX instruction set before execution SHALL result in an invalid opcode exception.