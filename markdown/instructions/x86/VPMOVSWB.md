> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVSWB

Moves bytes from a source to a destination, with sign-extension from 8-bit to 16-bit elements.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m16 | xmm reg |
| xmm reg | m16 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the processor to support the AVX instruction set.

The destination must be aligned to 16 bytes if the memory operand is used, otherwise a general-protection fault may occur. Using the same register for both source and destination is permitted.