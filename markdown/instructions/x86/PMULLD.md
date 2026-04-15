> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMULLD

Multiplies two 32-bit unsigned integers from the source operands and stores the 64-bit unsigned result into the destination. This operation is performed on each corresponding pair of 32-bit elements within the XMM registers.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or 32-bit mode when the SSE4.1 extension is supported. It is not available in compatibility mode if the processor does not support the required feature flags.

The instruction is a SIMD operation; the result of each 32-bit multiplication is stored as a 64-bit unsigned integer (u64), meaning the destination register must be capable of holding the expanded result size to avoid data truncation. Failure to account for the 64-bit output per element will lead to incorrect data alignment in subsequent vector operations.