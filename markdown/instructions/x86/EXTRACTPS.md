> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# EXTRACTPS

Extracts a single-precision floating-point value from a packed XMM register and stores it in a destination operand. The index of the element to be extracted is specified by an immediate value.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |
| xmm reg | m32 |

DO NOT support LOCK

This instruction is available in 32-bit and 64-bit modes. It requires the SSE3 instruction set extension.

The immediate operand MUST be between 0 and 3. Any value outside of this range will result in an invalid operation. The destination XMM register is updated such that the extracted f32 value is placed in the lowest 32 bits, and the upper bits are zeroed. If the destination is a memory region (m32), only the specified 4 bytes are modified.