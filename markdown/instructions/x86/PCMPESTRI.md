> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCMPESTRI

PCMPESTRI compares two operand strings (XMM registers) using a specific comparison mode defined by an immediate byte. Depending on the mode, it performs either equal-any, equal-range, or equal-all operations, returning the index of the first character that does not satisfy the condition.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| xmm | xmm |
| imm | xmm |

DO NOT support LOCK

This instruction requires the SSE4.1 extension. It operates exclusively on XMM registers and is supported in both 64-bit mode and compatibility mode.

To avoid incorrect index results, the user SHALL ensure the immediate value is correctly configured for the desired comparison mode (Equal Any, Equal Range, or Equal All). Because the instruction updates the ECX register with the index of the first non-matching character, any pre-existing value in ECX is overwritten. The user MUST verify the state of the MXCSR register, as the instruction's behavior regarding alignment and floating-point exceptions may be affected, although it primarily performs integer comparisons.