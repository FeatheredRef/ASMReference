> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTPD2UDQ

Converts the lower two double-precision floating-point values from a source to unsigned 64-bit integer values, truncating toward zero. The results are stored in the destination.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode.

If the source value is NaN, the destination is set to the indefinite unsigned integer value (0xFFFFFFFFFFFFFFFF). If the value is too large to be represented as a u64, the destination is set to the maximum representable unsigned 64-bit integer. If the source value is negative, the result is set to 0.

The instruction utilizes the MXCSR register for rounding control, although the truncation behavior is hard-coded for this specific mnemonic. Failure to initialize the XMM registers or accessing unaligned memory operands may result in general protection faults or alignment check exceptions depending on the processor configuration.