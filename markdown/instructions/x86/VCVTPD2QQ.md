> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPD2QQ

Converts packed double-precision floating-point values to packed unsigned 64-bit integers with truncation. Each f64 value in the source is converted to a u64 value; if the value is too large or too small to fit in the destination, the result is set to the maximum or minimum representable u64 value respectively, or triggers an exception depending on the rounding mode.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m128 | r128 |
| r128 | r128 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires AVX support.

The instruction uses the rounding-control field in the MXCSR register to determine the rounding mode. If the source value is NaN, the result is undefined and may trigger #I. If the value is out of range for a u64, #O is signaled. Precision loss during truncation may trigger #P.