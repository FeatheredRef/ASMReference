> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTNE2PS2BF16

Converts two packed single-precision floating-point values from the source to two bfloat16 floating-point values in the destination. The conversion is performed by rounding the single-precision values to the nearest bfloat16 value using the rounding mode specified in the MXCSR register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm (f32 $\times$ 2) | xmm (bf16 $\times$ 2) |
| m8 (f32 $\times$ 2) | xmm (bf16 $\times$ 2) |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires a processor supporting the AVX-512 BF16 extension.

The instruction performs truncation of the mantissa from 23 bits to 7 bits. If the result cannot be represented as a bfloat16, it SHALL trigger #O or #P depending on the rounding mode and magnitude of the value. Because the bfloat16 format shares the same exponent range as IEEE 754 single-precision, overflow occurs only if the input is a NaN or Infinity that cannot be mapped, though typically these are preserved. Precision loss is expected during the reduction of the significand.