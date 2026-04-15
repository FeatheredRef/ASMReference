> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGETMANTPD

Extracts the significand (mantissa) from each 64-bit double-precision floating-point value in the source operand and stores the result in the destination operand. The extracted mantissa is represented as an integer value where the implicit leading bit is restored.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| m64 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX extension to be enabled.

The destination register must be different from the source register to avoid undefined behavior in certain implementations, although the ISA generally allows overlapping. The instruction does not trigger floating-point exceptions (#Z, #D, #O, #U, #P) as it performs a bitwise extraction and integer conversion of the mantissa rather than an arithmetic operation. If the input is a NaN or Infinity, the resulting integer value reflects the raw significand bits.