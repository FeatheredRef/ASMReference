> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VINSERTI64x4

Inserts four 64-bit integer values from a source operand into a destination YMM register, controlled by an immediate byte that specifies the insertion index.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | ymm reg |
| m128 | ymm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is not supported in compatibility mode.

The immediate operand SHALL be a value between 0 and 3. If the immediate value is outside this range, the instruction is invalid. The source operand MUST be a 128-bit register or memory location; however, only the relevant 64-bit elements are inserted into the 256-bit destination register. Using an unsupported immediate value SHALL result in an invalid opcode exception.