> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AESENC

Performs a single round of AES encryption on a 128-bit state. This includes the ShiftRows, SubBytes, and MixColumns transformations, followed by an addition (XOR) with the round key.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction REQUIRES the AES-NI feature flag to be enabled in the processor. It is supported in 64-bit mode and compatibility mode. It MUST be executed on a processor that supports the SSE4.1 instruction set or later.

The destination register MUST NOT be the same as the source memory operand if the implementation does not support memory-to-register operations for this specific opcode. Ensure that the input data is correctly aligned to 16-byte boundaries when using memory operands to avoid performance penalties or general protection faults depending on the alignment check settings.