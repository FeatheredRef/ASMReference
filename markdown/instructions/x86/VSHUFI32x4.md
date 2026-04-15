> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSHUFI32x4

This instruction shuffles four 32-bit double words from two 128-bit source operands based on an immediate byte. The immediate specifies which double word from the first source operand or the second source operand is selected for each of the four positions in the destination.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| xmm reg, imm | xmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode and compatibility mode. It requires the AVX instruction set extension.

To avoid undefined behavior or performance penalties, ensure that the destination register is not used as a source operand unless the architecture supports destructive operands. The immediate value is used as a selector; bits 3-0, 7-4, 11-8, and 15-12 of the immediate determine the source for each corresponding 32-bit element. Only the least significant bit of each 4-bit selector field is used to choose between the first (0) and second (1) source operand.