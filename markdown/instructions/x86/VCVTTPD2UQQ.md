> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTPD2UQQ

Converts packed double-precision floating-point values to unsigned 64-bit integers. The operation truncates the floating-point values toward zero.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m16 | r64 |
| xmm | r64 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is not supported in compatibility mode.

The destination register `r64` must be different from the source `xmm` register. If the converted value is too large to be represented in a `u64` (overflow), the destination is filled with the indefinite integer value $2^{64}-1$. Precision exceptions (#P) may be raised if the result is rounded.