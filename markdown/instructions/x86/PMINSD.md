> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMINSD

Computes the minimum of two signed 32-bit integers packed into XMM registers. The operation is performed on each pair of corresponding 32-bit elements in the source and destination operands.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit modes. It requires SSE4.1 support.

The instruction treats elements as signed integers (i32). If a NaN (Not-a-Number) is encountered in a floating-point context, it does not apply here as this is an integer operation; however, users MUST ensure the data is intended for signed integer comparison to avoid incorrect results due to two's complement interpretation. If an element is processed as a signed integer, the comparison follows the signed integer ordering.