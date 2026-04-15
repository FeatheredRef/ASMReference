> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ROUNDPD

Rounds the double-precision floating-point values in the destination operand to the rounding direction specified by the immediate operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m64 | xmm |

DO NOT support LOCK

This instruction is available only when SSE4.1 is supported. It operates on double-precision floating-point values and is available in both 32-bit and 64-bit modes.

The immediate operand MUST be a value between 0 and 3 to specify the rounding direction (00: round to nearest, 01: round toward negative infinity, 10: round toward positive infinity, 11: round toward zero). Any other immediate value SHALL result in an #I exception. The instruction MAY trigger #P if the rounded result is not exact.