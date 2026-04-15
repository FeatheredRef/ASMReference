> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTPH2UQQ

Converts packed half-precision floating-point values to packed unsigned quadword integers with truncation toward zero.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512 FP16 instruction set. It requires the processor to be operating in 64-bit mode.

To avoid precision-related issues or unexpected results, the following details are relevant:
- If the source value is NaN, the result is the indeterminate value (all bits set to 1).
- If the result of the conversion exceeds the range of a u64, the result is the maximum representable value for a u64.
- This instruction triggers the #P exception if the result is inexact.
- The operation performs a truncation toward zero, meaning it does not round to the nearest even integer.