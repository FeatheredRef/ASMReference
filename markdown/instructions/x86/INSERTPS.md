> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INSERTPS

Inserts two packed single-precision floating-point values from the source operand into the destination operand. The immediate value determines which elements are inserted and whether the insertion is performed as a replacement or a merge.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m32 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE support.

The immediate value MUST be a 8-bit immediate; however, only the lower 4 bits are used. Bits 7:4 are MUST be zero. If the immediate value is not correctly formatted, the instruction behavior is undefined.

The instruction does NOT affect any EFLAGS flags. It does NOT trigger floating-point exceptions such as #D, #O, #U, or #P. Memory operands MUST be aligned to 16 bytes if using aligned move variants; otherwise, a general-protection fault may occur depending on the CPU's alignment check settings.