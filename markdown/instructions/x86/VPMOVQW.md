> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVQW

Zero-extends 16-bit signed integers from a source to 64-bit signed integers in a destination.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m16 | xmm/ymm/zmm |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction requires a processor with AVX-512 BW extensions. It is not available in compatibility mode for 32-bit applications if the AVX-512 BW feature is not explicitly supported by the operating system and hardware.

When using the EVEX encoding, the instruction supports masking. If a mask is applied, the merge or zeroing behavior is determined by the mask register; elements not enabled by the mask will either retain their original value or be zeroed, depending on the masking setting. Failure to account for mask behavior can result in corrupted destination data.