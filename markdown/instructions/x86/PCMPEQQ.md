> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCMPEQQ

Compares two 64-bit quadword integers from the source and destination operands for equality. If the quadwords are equal, the corresponding 64 bits of the destination register are set to 1; otherwise, they are set to 0.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m8/m16/m32/m64 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension or higher.

The destination register MUST be an XMM, YMM, or ZMM register. Using this instruction with unsupported register widths or in 32-bit mode SHALL result in an invalid opcode exception. When using YMM or ZMM registers, the operation is performed as a SIMD parallel comparison across the vector width.