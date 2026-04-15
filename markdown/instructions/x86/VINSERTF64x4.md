> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VINSERTF64x4

Inserts four 64-bit floating-point values from a source into a destination YMM register, governed by an immediate byte that specifies the insertion index.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | ymm reg |
| m64 | ymm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX support.

To avoid undefined behavior or crashes, ensure that the destination register is a YMM register; using XMM registers as the primary destination for this specific 4-element insertion is invalid. The immediate value MUST be 0, 1, 2, or 3 to select the target lane; any other value is an invalid opcode.