> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPH2W

Converts packed half-precision floating-point values to packed single-precision floating-point values. The operation converts each f16 value in the source to an f32 value in the destination, using the rounding mode specified in the MXCSR register.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m16/m32/m64 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or 32-bit mode. It requires the AVX-512 FP16 extension for zmm register support.

To avoid precision loss or unexpected rounding, the user SHALL ensure the MXCSR rounding control bits are correctly configured before execution. Since the instruction converts from a lower-precision format (f16) to a higher-precision format (f32), it typically does not trigger #O or #U, but may trigger #P if the value cannot be represented exactly in the destination format (though rare for f16 to f32). Memory operands MUST be aligned to the natural boundary of the vector length to avoid performance degradation or alignment faults.