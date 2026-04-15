> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPS2QQ

Converts packed single-precision floating-point values to packed signed 64-bit integers. The conversion is performed using the rounding mode specified in the MXCSR register.

The following table describes the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| xmm (f32 $\times$ 4) | xmm (i64 $\times$ 2) |
| m128 (f32 $\times$ 4) | xmm (i64 $\times$ 2) |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires AVX-512 support.

The destination register is overwritten. Because the output elements (i64) are twice the size of the input elements (f32), only the first two elements of the source xmm register are converted and stored in the destination xmm register. Attempting to use this instruction in 32-bit compatibility mode SHALL result in an invalid opcode exception.

If the converted value is too large or too small to be represented as an i64, the result is the integer value corresponding to the indefinite value, and #O is signaled. Denormal inputs are handled according to the MXCSR settings, potentially signaling #D. Inexact results SHALL signal #P.