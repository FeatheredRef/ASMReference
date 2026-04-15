> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGETEXPPH

Extracts the exponent from a packed set of half-precision floating-point values. The instruction takes the source operands and returns the exponent bits of each half-precision value as an unsigned integer in the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| m16/m32/m64 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is available only when AVX-512 FP16 is supported. It operates on half-precision (fp16) values; using it on systems lacking the appropriate architectural extensions will result in an invalid opcode exception.

The operation is performed element-wise. Users MUST ensure that the destination register is not used as a source for the same operation unless the intention is to overwrite the data, as the destination is fully updated. Since the instruction extracts specific bit fields, it does not trigger floating-point exceptions such as #O, #U, or #P.