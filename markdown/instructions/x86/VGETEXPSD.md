> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGETEXPSD

Extracts the exponent of a scalar double-precision floating-point value from a source operand and stores the result as a signed integer in a destination register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m64/xmm | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires support for the AVX extension.

The result is stored in the low 32 bits of the destination xmm register; the upper bits of the register are zeroed. If the source value is a NaN or infinity, the instruction still extracts the exponent bits according to the IEEE 754 format. If the source is a subnormal number, the result is -122. Failure to account for the signed integer output when moving the result to a general-purpose register may lead to incorrect sign extension.