> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVLPS

Moves the lowest 32 bits of a packed single-precision floating-point value from the source to the lowest 32 bits of the destination. The upper 32 bits of the destination are set to zero.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m32 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE support.

To avoid unexpected behavior, it SHALL be noted that MOVLPS specifically clears the upper 128 bits of the destination XMM register if the processor is executing in a context where YMM/ZMM registers are used (AVX), though for the standard XMM destination, only the upper 32 bits of the 128-bit register are zeroed. Failure to account for the zeroing of the upper bits may lead to the loss of previously stored data in the destination register.