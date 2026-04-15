> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMPD

VPERMPD selects 64-bit double-precision floating-point values from the first source operand based on indices specified in the second source operand and stores the result in the destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m64 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX extension to be enabled.

The indices provided in the second source operand MUST be within the range of the number of elements available in the first source operand for the specific vector length (e.g., 0-3 for YMM, 0-7 for ZMM). Indices that are out of range SHALL result in the corresponding destination element being set to all zeros.