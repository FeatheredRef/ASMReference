> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPD2UDQ

Convert packed double-precision floating-point values to packed unsigned 64-bit integers. The instruction truncates the floating-point values toward zero.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m16 | xmm reg |

DO NOT support LOCK

The instruction requires AVX support. It operates on XMM registers and is available in 64-bit mode and compatibility mode.

If the source value is NaN, the destination is set to the indefinite integer value (all bits set to 1). If the source value is too large to be represented as a u64, #O is signaled and the destination is set to the maximum u64 value. Precision #P is signaled if the result is not exact. All operations are subject to the rounding control in the MXCSR register, though truncation is the default behavior for this specific opcode.