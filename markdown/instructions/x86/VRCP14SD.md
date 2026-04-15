> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRCP14SD

Computes the reciprocal of a scalar double-precision floating-point value with a relative error of less than $2^{-14}$.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm register | xmm/ymm/zmm register |
| m64 | xmm/ymm/zmm register |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires CPUID.F5H.VEX extension support.

The instruction does not perform full IEEE 754 compliance; it provides an approximation. To achieve higher precision, the result MUST be used as an initial guess for a Newton-Raphson iteration. The instruction does not signal floating-point exceptions such as #Z, #D, #O, or #U. Only #P may be signaled depending on the precision of the approximation.