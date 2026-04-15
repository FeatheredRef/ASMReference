> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTPS2UQQ

Convert packed single-precision floating-point values to packed unsigned quadword integers with truncation.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 Foundation (AVX-512F) instruction set.

To avoid precision loss or unexpected results, note that the instruction performs truncation toward zero. If the input floating-point value is NaN, the resulting unsigned quadword is the indeterminate value (all bits 1). If the value is outside the range of a 64-bit unsigned integer, the result is the maximum representable value ($2^{64}-1$).

The operation may trigger floating-point exceptions: #I if the operand is NaN, or #O if the value exceeds the representable range of u64.