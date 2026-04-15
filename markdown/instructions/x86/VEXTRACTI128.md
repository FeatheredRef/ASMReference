> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXTRACTI128

Extracts 128 bits of integer data from the source operand and stores them in the destination operand. The extracted 128 bits are selected based on the immediate value, which specifies the index of the 128-bit lane to be extracted from the source.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm reg |
| imm | #I |
| mN | #I |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or compatibility mode. It REQUIRES the AVX extension.

The immediate value MUST be within the valid range based on the size of the source register (e.g., for a zmm register, the immediate can be 0, 1, 2, or 3). If the immediate value is out of range for the specified source register size, it SHALL result in an invalid operation. The destination register is an xmm register; any bits in the destination register beyond the 128 bits extracted SHALL be ignored or overwritten depending on the specific operand size of the instruction variant.