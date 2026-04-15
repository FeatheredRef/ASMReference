> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# DIVPS

Divides the lowest 32 bits of a packed single-precision floating-point value by the lowest 32 bits of another packed single-precision floating-point value. The operation is performed for each of the four corresponding single-precision floating-point values in the source and destination operands.

The following table specifies the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m32 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires SSE support.

The instruction adheres to the IEEE 754 standard for single-precision floating-point arithmetic. If the divisor is zero, a #Z exception is generated. If the result is too large to be represented, a #O exception is generated. Precision and underflow flags (#P, #U) are set according to the rounding mode defined in the MXCSR register.

To avoid unexpected behavior, the user SHALL ensure that the MXCSR register is correctly configured for the desired rounding mode and exception masking, as the precision and underflow flags may be triggered without a hardware exception if masked.