> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMINSW

This instruction compares two packed signed 16-bit integers in each corresponding lane of the source and destination operands and stores the minimum value of each pair in the destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit mode. It requires the SSE4.1 instruction set extension.

To avoid unexpected behavior, the user SHALL ensure that the memory operands are aligned to 16 bytes if using aligned load/store variants, otherwise, a general protection exception may occur depending on the alignment check flags. The instruction operates on signed integers; using it on unsigned data SHALL result in incorrect minimum calculations due to the interpretation of the most significant bit as a sign bit.