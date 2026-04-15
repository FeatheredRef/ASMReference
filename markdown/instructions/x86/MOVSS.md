> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVSS

Moves a scalar single-precision floating-point value from a source to a destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m4 | xmm reg |
| xmm reg | m4 |

DO NOT support LOCK

This instruction requires SSE support. It is available in 64-bit mode, 32-bit mode, and compatibility mode.

When the destination is a memory operand (m4), the instruction only updates the first 4 bytes of the destination address; the remaining bytes of the memory location remain unchanged. The instruction does not trigger floating-point exceptions (#I, #Z, #D, #O, #U, #P) as it performs a raw data move without arithmetic processing. Using a non-aligned memory address may result in performance degradation or faults depending on the alignment check (AC) flag in the EFLAGS register.