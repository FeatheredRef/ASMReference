> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPD2PH

Converts packed double-precision floating-point values to packed half-precision floating-point values using a specified rounding mode.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/opmem (m64) | xmm |

DO NOT support LOCK

This instruction requires the AVX and F16C instruction set extensions. It is not available in compatibility mode if the processor does not support these extensions.

The conversion is subject to the rounding control specified in the immediate operand (brm) or the MXCSR register. If the conversion results in a value that cannot be represented in the destination format, it may trigger #O or #U. Precision loss will trigger #P.

The destination register is overwritten. Since a double-precision value (64-bit) is converted to a half-precision value (16-bit), the instruction packs multiple results into the destination register. Users MUST ensure the destination register is properly initialized if they intend to preserve higher-order bits beyond the range of the converted elements.