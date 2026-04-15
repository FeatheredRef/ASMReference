> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTPH2DQ

Converts packed half-precision floating-point values to packed signed 64-bit integers. The conversion is performed by rounding the half-precision floating-point value to the nearest signed integer using the rounding mode specified in the immediate operand.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

The instruction requires the AVX support bit to be enabled in the CPU. It is not available in compatibility mode for 32-bit operating systems if the AVX bit is not set.

The immediate operand MUST be used to specify the rounding mode; if the immediate is not provided, the instruction is invalid. Failure to specify a valid rounding mode will result in a general protection fault or invalid opcode exception.

Because the destination is a 64-bit integer (qword) and the source is a 16-bit float (word), the instruction expands the data. Consequently, only the lower 128 bits of the destination xmm register are updated, and only two half-precision values from the source are converted to two 64-bit integers. This leads to a partial update of the destination register if the source contains more than two elements.

If the converted value is too large to be represented as a signed 64-bit integer, the result is the integer representation of the indefinite value, and #O is signaled.