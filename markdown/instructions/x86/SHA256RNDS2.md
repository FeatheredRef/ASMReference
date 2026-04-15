> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHA256RNDS2

Performs two rounds of the SHA-256 message schedule update. It operates on the provided xmm registers to update the SHA-256 hash state using the specified constants and current state values.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or 32-bit mode when the Intel SHA Extensions are supported by the processor. It requires the use of XMM registers; any attempt to use general-purpose registers or memory operands directly as the primary operands for the transformation logic will result in an invalid opcode.

The instruction requires that the input XMM registers are properly initialized with the correct SHA-256 state and message schedule data. Failure to align the data according to the SHA-256 specification before execution will result in incorrect hash computations without triggering an architectural exception. All operands MUST be XMM registers.