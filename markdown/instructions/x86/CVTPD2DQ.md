> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTPD2DQ

Converts a double-precision floating-point value to a signed 64-bit integer value with rounding.

The following table specifies the supported sources and destinations:

| source | destination(s) |
| :--- | :--- |
| m64/f64 | r64/m64 |

DO NOT support LOCK

This instruction is available in 64-bit mode. It is NOT supported in compatibility mode.

The rounding method is controlled by the rounding control field in the MXCSR register. If the result exceeds the representable range of a signed 64-bit integer (i64), the operation shall result in an #O exception and the destination shall be set to the maximum or minimum representable signed 64-bit integer (saturation). If the input is a NaN, the operation shall result in an #I exception and the destination shall be set to an undefined value.