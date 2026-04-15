> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRCP14PS

Computes an approximation of the reciprocal of each single-precision floating-point value in the source and stores the result in the destination. The result is an approximation with a relative error of at most $2^{-14}$.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm / ymm / zmm register | xmm / ymm / zmm register |
| m32 / m64 / m128 / m256 / m512 | xmm / ymm / zmm register |

DO NOT support LOCK

This instruction requires SSE4.1 or AVX/AVX2/AVX-512 support depending on the register width used. It operates in 32-bit precision (single precision); using it in 64-bit precision is not supported.

The result is an approximation and NOT a precise reciprocal. To achieve higher precision, the result MUST be used as an initial guess for a Newton-Raphson iteration. The instruction does not set the precision (#P) or underflow (#U) exceptions. Special values such as $\pm\infty$ and NaNs are handled according to the IEEE 754 standard, where the reciprocal of $\pm\infty$ is $\pm 0$.