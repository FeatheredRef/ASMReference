> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRCP28PD

Computes an approximation of the reciprocal of the lower 28 bits of the mantissa for each 64-bit double-precision floating-point element in the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm register | xmm/ymm/zmm register |

DO NOT support LOCK

This instruction is available only when the processor supports the AVX-512 foundation instructions. It requires the processor to be in 64-bit mode or a compatibility mode that supports the corresponding vector extension.

The result is an approximation; users MUST perform a Newton-Raphson iteration or a similar refinement method if higher precision than the 28-bit mantissa approximation is required. The instruction does not check for division by zero or handle denormals according to standard IEEE 754 requirements in a way that produces a mathematically exact reciprocal; it is designed specifically for speed over precision.