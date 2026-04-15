> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD231SD

Performs a fused multiply-subtract operation on scalar double-precision floating-point values. It computes the result as $dest = (src1 \times src2) - src3$ and stores it in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f64, f64, f64 | f64 |
| m8, f64, f64 | f64 |
| f64, m8, f64 | f64 |
| f64, f64, m8 | f64 |

DO NOT support LOCK

This instruction requires the AVX support bit to be enabled in the `CR4` register; otherwise, it SHALL trigger an `#UD` exception. It operates exclusively in 64-bit mode or compatibility mode when using YMM registers.

The instruction is subject to the floating-point control word settings in the `MXCSR` register. Specifically, rounding mode and exception masking SHALL affect the final result. Inexact results SHALL trigger #P, and overflows SHALL trigger #O according to the current rounding mode. To avoid performance degradation due to "denormal-as-zero" or "flush-to-zero" behaviors, ensure the `MXCSR` register is configured to handle denormalized operands (`#D`) consistently.