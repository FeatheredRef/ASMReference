> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# UNPCKLPD

Unpacks two 64-bit floating-point values from two 128-bit XMM operands into a single 128-bit XMM register. It selects the low 64 bits of the first operand and the low 64 bits of the second operand, placing them as the low and high 64 bits of the destination register, respectively.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction requires the SSE2 instruction set. It is available in both 64-bit mode and compatibility mode.

The destination operand SHALL NOT be the same as the second source operand if the first source operand is a memory reference, as this may lead to undefined behavior depending on the specific processor implementation of memory-to-register operations. Ensure that the source operands contain valid 64-bit floating-point values to avoid unexpected results, although the instruction performs a bit-level shuffle and does not trigger floating-point exceptions.