> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVS

Copies a value from the source memory location (pointed to by RSI) to the destination memory location (pointed to by RDI). Depending on the operand size and the Direction Flag (DF), the pointers are incremented or decremented.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| mN | mN |
| #I | #I |

Support LOCK

This instruction is available in 64-bit mode, 32-bit mode, and 16-bit mode. When executing in 64-bit mode, the instruction uses the RDX, RSI, and RDI registers for addressing unless the operand size is overridden.

The Direction Flag (DF) MUST be configured prior to execution; if DF is 0, the indices are incremented, and if DF is 1, the indices are decremented. The specific increment/decrement value is determined by the operand size (1, 2, 4, or 8 bytes).

When used with a REP prefix, the instruction will repeat until the RCX register reaches 0. If RCX is initially 0, the instruction will NOT execute the copy operation and will immediately terminate. Users SHALL ensure RCX is loaded with a non-zero value to avoid skipping the operation.