> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSUBB

Subtracts the byte-sized elements of the source operand from the byte-sized elements of the destination operand.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| imm | xmm |

Support LOCK: DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates on packed byte integers within XMM registers.

The source immediate SHALL be sign-extended to 8 bits before the subtraction is performed. This instruction does not affect any EFLAGS register bits. Alignment of memory operands is not applicable as this instruction only supports register and immediate sources.