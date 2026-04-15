> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHUFPS

Shuffles four single-precision floating-point values from two XMM registers based on an immediate byte, storing the result in the destination register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE3 support.

The immediate byte specifies the index of the source element to be selected for each of the four positions in the destination. The first 4 bits of the immediate refer to the first source operand, and the next 4 bits refer to the second source operand. If an index exceeds the number of available elements in the selected source operand, the destination element is set to zero.

The destination register MUST NOT be the same as the memory source to avoid undefined behavior in specific microarchitectures, although the ISA generally allows it. All operations are performed on 32-bit floating-point values; any data in the registers not covered by the 128-bit XMM width is ignored.