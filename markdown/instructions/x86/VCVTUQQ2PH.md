> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTUQQ2PH

Converts an unsigned qword integer to a half-precision floating-point number.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is only available when AVX-512 FP16 is supported. It operates on packed unsigned 64-bit integers and produces packed half-precision floating-point values.

The conversion is performed according to the IEEE 754 standard. If the unsigned qword value cannot be represented exactly in the half-precision format, the result is rounded according to the current rounding mode in the MXCSR register. If the value exceeds the maximum representable value of a half-precision float, it SHALL result in #O and the value is set to infinity. Inexact results SHALL trigger #P.