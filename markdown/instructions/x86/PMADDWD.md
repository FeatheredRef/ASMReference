> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMADDWD

Multiplies signed word-sized elements from two XMM registers and adds the resulting dword-sized products to a destination register. Specifically, it multiplies the signed i16 elements of the source operands, and the 32-bit results are summed in pairs to produce signed i32 elements in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or 32-bit mode and requires the SSE4.1 instruction set extension. It is NOT supported in compatibility mode if the hardware does not implement SSE4.1.

To avoid data truncation or incorrect results, ensure that the input operands are treated as signed i16 integers. The operation performs a "multiply-accumulate" step; since the sum of two products of i16 values can exceed the range of a signed i16, the destination MUST be treated as a sequence of i32 elements. Failure to account for the change in element size from word to dword when processing the destination register will result in incorrect data interpretation.