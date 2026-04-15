> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPS2UQQ

Converts packed single-precision floating-point values to packed unsigned 64-bit integers. The conversion is performed by rounding the f32 values to the nearest even integer toward zero, and the result is stored as u64 values.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm-reg | xmm-reg |
| m128 | xmm-reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires AVX-512 foundation support (AVX-512F).

The destination register is written to as a packed 64-bit integer vector; however, because a single f32 value expands to a u64 value, the instruction typically targets a destination register that reflects the expanded size. If the rounding results in a value that cannot be represented within a u64, the result is the maximum unsigned 64-bit integer value. #P is raised if the rounded result is inexact.