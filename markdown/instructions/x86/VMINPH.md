> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMINPH

Computes the minimum of two packed half-precision (16-bit) floating-point values. The instruction compares the values and returns the smaller of the two. If one of the operands is a NaN, the result is the other operand; if both are NaN, the result is a NaN.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| m16 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or compatibility mode. It REQUIRES the AVX-512 F floating-point extensions (AVX-512 FP16) to be supported by the processor.

When using the evaporated form of the instruction (without masking), the destination register SHALL be overwritten. When using the masked version, the merge-masking behavior ensures that only the elements corresponding to the set bits in the mask register are updated in the destination; other elements SHALL remain unchanged. To avoid unexpected behavior, ensure the mask register is correctly initialized, as uninitialized mask bits may lead to inconsistent data in the destination register.