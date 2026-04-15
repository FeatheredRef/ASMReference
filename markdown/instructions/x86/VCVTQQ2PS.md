> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTQQ2PS

Converts packed signed qword integers to a packed single-precision floating-point vector. Each 64-bit signed integer element from the source is converted to a 32-bit floating-point element in the destination.

The following table covers the supported source and destinations:

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 foundation.

The destination register is overwritten; therefore, the source register must not be the same as the destination if the value is needed after the operation. The conversion follows the rounding mode specified in the MXCSR register. If the signed integer value cannot be represented exactly in the destination floating-point format, the result is rounded according to the current rounding mode, potentially triggering #P.