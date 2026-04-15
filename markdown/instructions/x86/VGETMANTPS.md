> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGETMANTPS

Extracts the mantissa from each packed single-precision floating-point value in the source operand. The mantissa is extracted by removing the sign bit and the exponent bits, and the result is stored as a packed set of unsigned integers.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512F instruction set. It operates on vector registers and is not available in compatibility mode if the required AVX-512 features are not enabled via CR4.MSR.

The result is returned as a u32 integer. If the input value is a NaN or infinity, the mantissa is extracted according to the IEEE 754 representation. Users SHALL ensure that the destination register is of the same vector length as the source to avoid undefined behavior or corruption of higher-order bits depending on the specific VEX/EVEX encoding used.