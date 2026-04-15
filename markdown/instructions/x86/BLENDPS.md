> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BLENDPS

BLENDPS blends four pairs of single-precision floating-point values from two XMM registers into a destination register based on a mask provided by an immediate value. For each of the four double words, if the corresponding bit in the immediate mask is set to 1, the value from the second source operand is selected; otherwise, the value from the first source operand is selected.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, imm | reg |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode or compatibility mode. It requires SSE4.1 support.

The immediate byte must be specified; if the immediate is omitted or invalid, the assembler SHALL signal an error. The instruction does not trigger any floating-point exceptions (#I, #Z, #D, #O, #U, #P) as it performs a bitwise selection rather than arithmetic operations. The destination register MUST be an XMM register; memory operands are NOT supported for the destination.