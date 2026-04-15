> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MULSD

Multiplies a scalar double-precision floating-point value from a source operand by a scalar double-precision floating-point value in a destination operand and stores the result in the destination operand.

The following table specifies the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m8 | xmm |

DO NOT support LOCK

This instruction requires the SSE3 extension set. It is supported in 64-bit mode, compatibility mode, and 32-bit mode.

The operation is performed according to the IEEE 754 standard. The result is rounded according to the rounding control bits in the MXCSR register. If the result exceeds the maximum representable value for a double-precision float, it SHALL trigger #O. If the result is smaller than the minimum representable positive value, it SHALL trigger #U. Precision loss SHALL trigger #P.

Ensure that the memory operand `m8` is aligned to an 8-byte boundary to avoid potential performance penalties or alignment check exceptions depending on the processor configuration.