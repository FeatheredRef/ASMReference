> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PADDB

Adds a byte from the source operand to the destination operand and packs the result into the destination. This instruction is part of the SIMD extensions for processing packed data.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is NOT available in 64-bit mode; it is only supported in compatibility mode as part of the SSE legacy instructions.

The operation is performed on 8-bit signed integers. When the result of the addition exceeds the range of a signed 8-bit integer (-128 to 127), the result is saturated to the maximum or minimum representable value rather than wrapping around. Failure to account for saturation may lead to unexpected data values in applications expecting standard two's complement wrap-around behavior.