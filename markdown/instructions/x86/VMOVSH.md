> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMOVSH

Moves a packed signed 16-bit integer from the source to the low 16 bits of the destination.

The following table covers the supported sources and destinations:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| m16 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX or AVX-512 instruction sets. It MUST be executed in 64-bit mode or 32-bit mode.

The destination register is overwritten. If a zmm register is used as the destination, the upper bits of the register beyond the target word are preserved unless the instruction is part of a specific AVX-512 mask operation. Users SHOULD ensure that the source memory is properly aligned to avoid performance penalties.