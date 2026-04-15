> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTPH2UDQ

Converts the packed signed 16-bit floating-point values (half-precision) from the source operand to packed unsigned 64-bit integers in the destination operand. The conversion is performed using the truncation rounding-mode.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 extension (specifically AVX-512 FP16) to be supported by the processor.

The result of the conversion is based on the truncation rounding-mode regardless of the current rounding-mode setting in the MXCSR register. If the source value is NaN or exceeds the range of a u64, the result is the indefinite integer value ($2^{64}-1$). The operation may trigger #O or #P exceptions depending on the precision and magnitude of the input value.