> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTUQQ2PS

Converts two unsigned 64-bit integers (u64) from a qword-sized source to two single-precision floating-point values (f32) and stores the result in a destination register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction requires the AVX-512 foundation (AVX-512F) to be supported and enabled. It is not available in legacy 32-bit mode.

The source operand MUST be a 128-bit XMM register or memory location containing two u64 values. Precision loss may occur during the conversion from u64 to f32 because a 32-bit floating-point mantissa cannot represent all 64-bit integer values exactly; in such cases, the result is rounded according to the current rounding control in MXCSR, potentially triggering #P.