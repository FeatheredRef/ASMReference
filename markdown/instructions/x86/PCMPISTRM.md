> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCMPISTRM

Compares two operand strings based on a specified immediate value (IMM8) to find the first character in the first operand that matches a condition relative to the second operand. The instruction searches for the first character in the first operand that is either equal to, less than, or greater than the corresponding character in the second operand, or matches a specific range, depending on the IMM8 value. The result is stored as an index in the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m16 | xmm |

DO NOT support LOCK

The instruction SHALL be executed only if SSE4.2 is supported by the processor. It operates on XMM registers and is available in 64-bit mode and compatibility mode.

The destination register is overwritten by the index of the first character that satisfies the comparison condition; if no such character is found, the index reflects the end of the string. Because the instruction modifies the destination register, the programmer MUST ensure the register is not used for other purposes without being saved. Memory operands MUST be 16-byte aligned to avoid performance penalties or faults depending on the specific processor implementation.