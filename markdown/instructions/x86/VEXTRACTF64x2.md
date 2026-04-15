> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXTRACTF64x2

Extracts two 64-bit floating-point values from a 128-bit YMM register and stores them into a XMM register.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm reg | xmm reg |
| imm | #I |
| mN | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX support.

To avoid undefined behavior or exceptions, ensure that the immediate operand is within the valid range for the specified register size; however, for `VEXTRACTF64x2`, the index is typically implicit or handled via specific AVX encoding. Using this instruction in 32-bit mode will result in an invalid opcode exception.