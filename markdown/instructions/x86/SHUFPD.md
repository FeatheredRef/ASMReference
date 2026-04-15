> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHUFPD

Shuffles two double-precision floating-point values from a source operand based on an immediate byte and stores the result in a destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |
| imm | #I |

DO NOT support LOCK

This instruction requires SSE3 support. It operates only on XMM registers or 128-bit memory regions. It is available in both 64-bit mode and compatibility mode.

The immediate operand MUST be a valid shuffle selector; if the immediate value is not 0x00 or 0x40, the behavior is undefined or may result in an invalid operation depending on the specific processor implementation. The instruction does not trigger floating-point exceptions (#Z, #D, #O, #U, #P) as it performs data movement rather than arithmetic calculations.