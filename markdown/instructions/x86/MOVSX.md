> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVSX

Sign-extends a smaller source operand to a larger destination operand. The most significant bit of the source is replicated to fill the remaining high-order bits of the destination.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r8 | r64 / m64 |
| m1 | r64 / m64 |
| r16 | r64 / m64 |
| m2 | r64 / m64 |
| r32 | r64 / m64 |
| m4 | r64 / m64 |
| imm | #I |

DO NOT support LOCK

The destination operand MUST be larger than the source operand. In x86-64 mode, the destination MUST be a qword (r64 or m64). The instruction is supported in 64-bit mode and compatibility mode.

To avoid undefined behavior or incorrect results, ensure the source operand is treated as a signed integer (iN), as the instruction performs sign-extension rather than zero-extension. If zero-extension is required, the MOZX instruction SHOULD be used instead.