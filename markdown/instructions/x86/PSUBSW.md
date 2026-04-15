> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSUBSW

Subtracts packed signed 16-bit integers from packed signed 16-bit integers, saturating the result to the minimum signed 16-bit integer value (-32768) if an underflow occurs. The result is then zero-extended to 32 bits.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit modes. It operates exclusively on XMM registers and does not affect the EFLAGS register.

To avoid unexpected results, ensure that the input operands are treated as signed i16 values. Because the result is zero-extended to 32 bits, the high 16 bits of each 32-bit element in the destination register will always be set to 0, regardless of the sign of the result.