> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VINSERTI32x4

Inserts four 32-bit doublewords from the source operand into the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode and 32-bit mode. It requires the AVX instruction set extension.

The destination operand is modified in-place; therefore, the destination register must not be used as a source if the original data needs to be preserved. This instruction does not affect any EFLAGS bits.