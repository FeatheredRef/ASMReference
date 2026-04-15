> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VREDUCESH

VREDUCESH reduces a packed floating-point value to a single-precision floating-point value by subtracting the largest multiple of $2^{16}$ that is less than or equal to the input. The result is a value in the range $[-2^{15}, 2^{15})$.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| f128 | f128 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It operates exclusively on registers; memory operands are not supported.

The instruction SHOULD be used in conjunction with VREDUCEPS or VREDUCEPD to implement high-precision reduction of floating-point numbers. Failure to properly handle the resulting range may lead to precision loss if the input exceeds the supported range of the reduction.

The following exceptions may occur:
- #P: Inexact result.