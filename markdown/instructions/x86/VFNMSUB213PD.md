> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB213PD

Computes the fused multiply-subtract result for two pairs of double-precision floating-point values. It calculates $(a \times b) - c$ and $(d \times e) - f$ using the formula $dest = (src1 \times src2) - src3$, where $src1$ and $src2$ are the multipliers and $src3$ is the subtrahend.

The following table covers what the source and destinations can be:

| Source | Destination(s) |
| :--- | :--- |
| f128, f128, f128 | f128 |
| m16, f128, f128 | f128 |
| f128, m16, f128 | f128 |
| f128, f128, m16 | f128 |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or compatibility mode. It REQUIRES the AVX support to be enabled in the processor.

The operation is performed with a single rounding step at the end; intermediate products are not rounded, which prevents cumulative rounding errors and avoids #O and #U for the intermediate product. If the result cannot be represented in the destination format, it MAY trigger #O, #U, or #P. If any operand is a signaling NaN, it SHALL trigger #I.