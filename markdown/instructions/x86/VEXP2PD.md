> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXP2PD

Converts packed 64-bit floating-point values to 64-bit floating-point values by calculating the base-2 exponential ($2^x$) of each element.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| m64/m128/m256/m512 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or 32-bit mode when the AVX-512 foundation instructions are supported. It requires the AVX-512DQ extension.

The result is rounded according to the rounding control (RC) field in the MXCSR register. The following exceptions may be raised:
- #O: Raised if the result is larger than the maximum representable f64 value.
- #U: Raised if the result is smaller than the minimum representable f64 value.
- #P: Raised if the result is not exact.

When the input is a signaling NaN, #I is raised. If the input is a quiet NaN, the result is a quiet NaN. If the input is $\pm\infty$, the result is $\pm\infty$ respectively.