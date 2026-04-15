> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTPI2PS

Converts a packed signed integer vector to a packed single-precision floating-point vector. Each i32 element of the source is converted to an f32 element in the destination.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m32 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction requires SSE3 support. It is available in both 32-bit and 64-bit mode.

The conversion process follows the rounding control in the MXCSR register. If the integer value is too large to be represented as an f32, a #O exception is generated and the result is set to the corresponding infinity. If the result cannot be represented exactly, a #P exception is generated.