> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTSS2SH

Converts a scalar single-precision floating-point value to a signed 16-bit integer using rounding. The result is stored in the destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f32 | r16 |
| f32 | m16 |

DO NOT support LOCK

This instruction requires SSE3 support. It operates on scalar values; only the lowest 32 bits of the source XMM register are used.

The operation follows the rounding mode specified in the MXCSR register. If the source value is NaN, a #I exception is raised and the destination is set to the indeterminate value. If the converted value exceeds the range of a signed 16-bit integer, a #O exception is raised and the result is saturated to the maximum or minimum representable signed 16-bit value.