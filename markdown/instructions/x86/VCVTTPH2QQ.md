> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTPH2QQ

Converts packed signed half-precision (16-bit) floating-point values from a source to packed signed 64-bit integers in a destination. The conversion uses the truncation rounding mode (round-toward-zero).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode.

The destination register is overwritten; therefore, the source and destination operands shall not overlap if a memory operand is used. If the converted value cannot be represented within the range of a signed 64-bit integer, the result is the integer minimum (i64 min) or maximum (i64 max) depending on the sign. 

Ensure that the target XMM register is correctly aligned if using memory operands to avoid alignment check exceptions. Because this instruction performs truncation, it does not trigger #P unless the value is too large to be represented as an i64.