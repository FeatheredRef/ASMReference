> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVQ2DQ

Moves a 64-bit quadword from a 128-bit XMM register to a 64-bit general-purpose register, or from a 64-bit general-purpose register to a 128-bit XMM register.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| xmm | r64 |
| r64 | xmm |
| #I | imm |
| #I | m8 |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode.

When moving from xmm to r64, ONLY the lower 64 bits of the xmm register are used. When moving from r64 to xmm, the 64-bit value is moved into the lower 64 bits of the xmm register, and the upper 64 bits of the xmm register are set to zero. Users SHALL ensure the destination register is 64-bit to avoid operand size mismatches.