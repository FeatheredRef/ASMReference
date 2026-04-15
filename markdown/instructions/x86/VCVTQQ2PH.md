> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTQQ2PH

Converts a signed 64-bit integer to a half-precision floating-point number. The instruction converts the signed integer value in the source operand to a 16-bit floating-point format and stores the result in the destination operand.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It requires AVX-512 support (specifically AVX-512F).

The conversion follows the rounding mode specified in the MXCSR register. If the result cannot be represented within the half-precision format, the following exceptions may be raised: #O for overflow or #P if the result is inexact. Values that are too small to be represented as normalized half-precision numbers may result in #U.