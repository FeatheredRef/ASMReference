> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VINSERTI32x8

Inserts eight 32-bit doublewords from the source operand into the destination vector operand based on a specified immediate index.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX-512 support (specifically AVX512F).

If the destination register is an xmm register, the instruction will zero the upper bits of the ymm or zmm register, depending on the vector length used. When using the EVEX encoding, the destination register size is determined by the LPL (Length Parameter) field; if the destination is smaller than the source, the behavior is governed by the masking and length rules of the AVX-512 ISA. Users SHALL ensure that the immediate index is within the valid range for the destination register size to avoid undefined behavior.