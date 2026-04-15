> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMADD52LUQ

Multiplies two 52-bit unsigned integers from the source operands, adds the resulting 104-bit product to the destination operand, and stores the result in the destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m128/m256/m512 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the AVX-512 BW (Bit Wide) extension to be supported by the processor.

The instruction operates on the lower 52 bits of the 64-bit unsigned integers. The upper 12 bits of the input operands are ignored. To avoid precision loss or incorrect calculations, the user MUST ensure that the input values are within the range of $0$ to $2^{52}-1$. The destination register MUST be initialized or handled as a 64-bit accumulator to accommodate the potential 104-bit intermediate product before overflow occurs in the 64-bit destination.