> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADDSUB132PH

Performs a fused multiply-add or multiply-subtract operation on half-precision floating-point values. Specifically, it computes the result of $(a \times b) + c$ or $(a \times b) - c$ for each corresponding pair of elements in the provided vectors, where the operand order is defined by the "132" pattern (dest = dest $\pm$ (src1 $\times$ src2)).

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg, m16 | reg |
| reg, reg | reg |

DO NOT support LOCK

This instruction requires AVX-512 FP16 support. It is only available in 64-bit mode or compatibility mode. It operates on zmm or ymm registers; use of xmm registers is not supported for this specific mnemonic.

To avoid precision loss or incorrect results, the user SHALL ensure that the floating-point control word is configured for the desired rounding mode. The instruction may trigger floating-point exceptions including #P, #O, #U, and #D. If the operation results in a value that cannot be represented within the f16 format, the result SHALL be rounded according to the current MXCSR register settings.