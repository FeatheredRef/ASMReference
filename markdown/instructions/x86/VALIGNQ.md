> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VALIGNQ

VALIGNQ aligns a 128-bit doubleword value from a source to a destination based on a specified byte offset. It extracts a 64-bit quad word from the concatenation of two 128-bit XMM registers (or one XMM register and its successor in some contexts) starting at the byte offset provided by an immediate value.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The immediate operand SHALL be a u8 value. If the immediate is greater than 15, the instruction will trigger a general-protection exception (#GP). Ensure that the destination register is not used as the source for the alignment offset to avoid data corruption.