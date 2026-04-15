> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PABSW

PABSW packs the signed 16-bit integers from the source into signed 8-bit integers in the destination. For each 16-bit element, if the absolute value is less than or equal to 127, the value is truncated to 8 bits; otherwise, the value is saturated to the maximum or minimum signed 8-bit integer (127 or -128).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor supports the SSE2 instruction set. It operates exclusively on XMM registers; memory operands are not supported.

The source and destination must be the same size (128 bits). Using this instruction on registers containing NaNs or infinity will result in those values being treated as signed integers, which may lead to unexpected saturation results. Ensure that the input data is properly aligned to avoid performance penalties.