> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVQD

Moves packed signed quadwords from the source to the destination, converting them to packed signed doublewords.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m8/m16/m32/m64 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires AVX or AVX-512 support depending on the register width used (YMM or ZMM). When using ZMM registers, the instruction may require the EVEX prefix for masking or broadcasting capabilities.

The destination register is overwritten; therefore, the original data in the destination register SHALL be preserved in a separate register if it is required for subsequent operations. If the source is a memory operand, the memory access SHALL be aligned to the size of the element to avoid performance penalties, although the hardware may handle unaligned accesses.