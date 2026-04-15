> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PADDW

Adds two signed packed word integers. The operation is performed on each pair of corresponding 16-bit elements in the source operands, and the result is stored in the destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE2 instruction set.

The operation performs signed addition on each 16-bit element. Overflow in an individual element is ignored and the result wraps around. This behavior ensures that the operation adheres to two's complement arithmetic.