> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTSD2USI

Converts a scalar double-precision floating-point value to an unsigned 64-bit integer. The conversion is performed by truncating the result toward zero.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | r64 |
| m8 | r64 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The result of the truncation MUST be within the range of a u64. If the input value is NaN or exceeds the range of a u64, the destination register is set to the indefinite integer value (0x8000000000000000). If the input is negative, the destination register is also set to the indefinite integer value.

The instruction does NOT trigger floating-point exceptions. Precision (#P) and Overflow (#O) flags are NOT set during the truncation process.