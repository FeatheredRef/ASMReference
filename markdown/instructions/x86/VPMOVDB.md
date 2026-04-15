> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVDB

Moves bytes from the source to the destination. It rearranges the bytes from the source and places them into the destination according to the specified vector length.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m8 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It REQUIRES the AVX-512BW extension.

The destination register's upper bits (beyond the vector length specified by the opcode) are zeroed. Failure to account for the zeroing of the upper register elements MAY lead to data loss if the programmer expects the destination register to maintain its previous state.