> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TDPBUSD

This instruction converts a signed integer value to a packed unsigned saturated 8-bit integer. It converts the source operand to a signed 8-bit integer, saturating the result to the minimum or maximum representable value of a signed 8-bit integer (-128 to 127), and then treats that result as an unsigned 8-bit integer.

The table below covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the SSE4.1 instruction set extension.

To avoid unexpected results, ensure the source register is an XMM register containing packed signed integers; providing floating-point values or incorrect types will result in a bit-pattern interpretation that may lead to saturation at the boundaries of the signed 8-bit range.