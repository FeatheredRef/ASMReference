> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPBROADCASTB

Broadcasts a byte from a source operand to all bytes of a destination vector register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m1 | xmm/ymm/zmm |
| r8 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or 32-bit mode. It requires the AVX or AVX-512 instruction set extensions depending on the vector length used.

The destination register is overwritten; therefore, the original content of the destination register is not preserved. When using YMM or ZMM registers, the operation is subject to the specific AVX-512 masks if used in a masked version of the instruction. Failure to ensure the processor supports the required extension (AVX/AVX-512) will result in an invalid opcode exception.