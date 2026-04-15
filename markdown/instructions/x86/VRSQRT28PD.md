> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRSQRT28PD

Computes the approximate reciprocal square root of packed double-precision floating-point values. It calculates $1/\sqrt{x}$ for each double-precision element in the source operand with a relative error of less than or equal to $2^{-28}$.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm register | xmm/ymm/zmm register |
| m8/m16/m32/m64 | xmm/ymm/zmm register |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX support feature to be enabled.

The result is an approximation; therefore, the precision of the result is limited. Users SHOULD follow this instruction with Newton-Raphson iterations if higher precision is required. If the input is $\pm\infty$, the result is $+0$. If the input is $\text{NaN}$, the result is the same $\text{NaN}$. If the input is $\pm 0$, the result is $\infty$.

Floating-point exceptions are handled as follows:
- #P: May be set.
- #Z: Not set.
- #D: Not set.
- #O: Not set.
- #U: Not set.
- #I: Set if the input is negative.