> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPH2UDQ

Converts packed half-precision floating-point values to packed unsigned 64-bit integers. The instruction converts each half-precision value to an unsigned 64-bit integer by rounding it to the nearest even integer.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX support.

The destination register is partially overwritten; for `VCVTPH2UDQ`, only the lower 64 bits of the destination are updated if the source contains a single half-precision value, or the lower 128 bits are updated if converting multiple values. To avoid data corruption in the upper bits of the destination register, ensure the target `xmm` register is correctly managed. Precision is handled according to the rounding mode specified in the MXCSR register, which may trigger #P.