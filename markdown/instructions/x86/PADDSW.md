> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PADDSW

Adds two packed signed 16-bit integers. The operation is performed on each pair of elements in the source and destination operands, with the result stored in the destination. The addition is performed with signed saturation; if the result exceeds the maximum or minimum value of a signed 16-bit integer, it is clamped to the respective limit.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is only available when the processor is in 64-bit mode or compatibility mode. It requires the SSE2 instruction set extension.

To avoid incorrect calculations, ensure that the input data is treated as signed i16 integers. If unsigned addition is required, use the `PADDW` instruction instead. Since the operation utilizes saturation, the standard arithmetic flags (such as Carry or Overflow flags) are not affected.