> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTQQ2PD

Converts a signed qword integer value to a double-precision floating-point value. The instruction converts a signed 64-bit integer to a 64-bit floating-point number and stores the result in the destination.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | xmm |
| reg (r64) | xmm |

DO NOT support LOCK

This instruction requires the AVX CPU feature. It is only available in 64-bit mode or 32-bit mode (if AVX is supported).

The conversion is performed according to the rounding control in the MXCSR register. If the result cannot be represented exactly in the destination format, the #P exception is signaled. Since the range of a 64-bit signed integer is within the representable range of a double-precision float (though precision loss may occur), #O is not typically triggered for this specific conversion.