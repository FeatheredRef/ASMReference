> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPH2QQ

Converts packed half-precision floating-point values to packed quad-precision floating-point values. The instruction converts two 16-bit floating-point values from a source to two 64-bit floating-point values in the destination, using the rounding control in the MXCSR register.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode. It is not supported in compatibility mode.

The instruction requires AVX-512 support. Specifically, it is part of the AVX-512 FVP (Floating-Point and Vector Precision) subset.

Failure to set the correct rounding mode in the MXCSR register may result in precision loss or incorrect rounding of the converted values, potentially triggering #P. Ensure that the source register contains valid half-precision formats to avoid unexpected behavior.