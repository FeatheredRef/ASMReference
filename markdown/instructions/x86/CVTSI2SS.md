> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTSI2SS

Converts a signed integer source to a scalar single-precision floating-point value. The conversion is performed according to the rounding control in the MXCSR register.

The table below covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| r32 / m32 | f32 |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit modes. It requires SSE support.

The destination must be an XMM register; memory-to-memory operations are NOT supported. When the source value is too large to be represented exactly in the single-precision format, the result is rounded according to the rounding mode specified in MXCSR. If the result cannot be represented, it MAY trigger #O or #P.