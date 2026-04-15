> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVUSQW

Moves unsigned 16-bit integers from a source to 64-bit integers in a destination, with zero-extension.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m16 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX or AVX-512 extensions depending on the register width used (XMM, YMM, or ZMM).

The destination register MUST be sufficiently large to hold the expanded data; failure to provide a register of the correct width results in an invalid opcode. When using the EVEX prefix, the instruction supports masking and broadcasting of the source operand. Memory accesses MUST be aligned to the size of the accessed element to avoid performance penalties, although the hardware handles unaligned accesses.