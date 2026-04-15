> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTTPD2PI

Converts the two double-precision floating-point values in the source operand to signed integers using the truncation rounding mode. The results are stored in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is available in both 64-bit mode and compatibility mode. It requires the SSE4.1 instruction set extension.

The conversion is performed by truncating the fractional part (rounding toward zero). If the converted value is too large to be represented as a signed 32-bit integer (i32), the operation SHALL result in an integer indispensable value (typically the most negative i32 value) and trigger a #O exception. Any NaN (Not-a-Number) input SHALL result in the integer value undefined by the architecture, typically 0x80000000, and trigger a #O exception. Use of this instruction on an unsupported processor SHALL result in an invalid opcode exception.