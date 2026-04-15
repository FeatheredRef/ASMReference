> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ANDNPD

Performs a bitwise logical AND operation between the bitwise NOT of the first source operand and the second source operand, storing the result in the destination operand.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires SSE4.1 support.

The operation is performed on the low 128 bits of the XMM registers. If a memory operand is used, the memory address SHALL be aligned to 16 bytes to avoid potential performance penalties or faults depending on the alignment check (AC) flag in the EFLAGS register.