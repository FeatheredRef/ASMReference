> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHA1MSG2

SHA1MSG2 performs the second step of the SHA-1 message schedule. It computes the sum of three 32-bit values, applies a right rotate of 7 bits to the result, and XORs the result with a fourth 32-bit value.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor is in 64-bit mode or compatibility mode. It requires the processor to support the SHA extensions.

The instruction operates on 128-bit XMM registers. To avoid incorrect results, the programmer MUST ensure that the input XMM registers contain valid 32-bit words aligned with the SHA-1 specification, as the operation is performed on the four 32-bit elements within the registers. Failure to manage the data layout within the xmm registers will lead to logic errors in the message schedule calculation.