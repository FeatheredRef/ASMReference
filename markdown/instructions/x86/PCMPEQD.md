> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCMPEQD

Compares packed signed 32-bit integers in the destination operand with the signed 32-bit integers in the source operand. For each pair of corresponding dword elements, the instruction sets the corresponding 32-bit lane in the destination to all ones (mask) if the elements are equal, and all zeros otherwise.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires SSE4.1 support.

To avoid incorrect results, ensure that both operands are aligned to 16-byte boundaries when using memory references; otherwise, a general-protection exception may occur depending on the alignment check flag and the specific processor implementation. The instruction does not affect any EFLAGS bits.