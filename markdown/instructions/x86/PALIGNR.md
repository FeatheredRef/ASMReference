> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PALIGNR

Concatenates two XMM registers, extracts a contiguous sequence of 128 bits starting from a specified bit offset, and stores the result in the destination XMM register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| xmm reg | m128 |
| m128 | xmm reg |

DO NOT support LOCK

The instruction is available in both 32-bit and 64-bit mode. It requires the SSE4.1 instruction set extension to be supported by the processor.

The immediate operand specifies the number of bits to shift the first source operand to the right. This value MUST be between 0 and 127. An immediate value outside this range SHALL result in an invalid opcode exception. The alignment is performed by concatenating the two source registers into a 256-bit temporary register before extracting the 128-bit window based on the immediate offset.