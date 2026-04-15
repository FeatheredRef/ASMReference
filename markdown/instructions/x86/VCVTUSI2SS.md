> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTUSI2SS

Converts an unsigned 32-bit integer to a single-precision floating-point number. The result is stored in the destination operand.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| r32 | f32 |
| m32 | f32 |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It is not available in compatibility mode if the processor does not support the SSE3 instruction set.

To avoid precision loss, note that the conversion of a $u32$ to an $f32$ may result in the #P exception if the integer value cannot be represented exactly in the destination floating-point format. The rounding is governed by the rounding control bits in the MXCSR register.