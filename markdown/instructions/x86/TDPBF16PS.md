> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TDPBF16PS

Convert packed bfloat16 floating-point values to packed single-precision floating-point values.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires a processor supporting the AVX-512 BF16 extension.

When executing this instruction, the conversion of bfloat16 to single-precision is performed by padding the mantissa with zeros. This process is lossless and does not trigger floating-point exceptions; however, if the input values are signaling NaNs, they are converted to quiet NaNs. Failure to ensure the target register is properly initialized may lead to the preservation of upper bits in the destination xmm register if the instruction is used in a non-merged masking operation.