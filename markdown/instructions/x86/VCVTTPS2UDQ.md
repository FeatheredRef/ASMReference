> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTPS2UDQ

Converts packed single-precision floating-point values to packed unsigned 64-bit integers with truncation. The instruction converts each f32 value in the source to a u64 value by rounding toward zero.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m32 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX-512 support (specifically AVX-512F).

If the source f32 value is NaN, the result is the integer value $2^{64}-1$. If the value is too large to fit in a u64, the result is the integer value $2^{64}-1$. These conditions trigger the #O exception. Precision (#P) and underflow (#U) exceptions may be triggered depending on the rounding behavior of the truncation.