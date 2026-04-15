> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVW2M

Moves word-sized elements from a SIMD register to a contiguous sequence of memory locations.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | m2 |
| #I | imm |

DO NOT support LOCK

The instruction SHALL be executed in 64-bit mode or compatibility mode. It REQUIRES support for AVX or AVX-512 depending on the register width used.

When using zmm registers, the instruction SHALL be used in conjunction with AVX-512. Ensure that the memory destination is correctly aligned to avoid performance penalties, although the instruction DOES NOT REQUIRE strict alignment. Overlapping memory regions between source and destination are NOT supported and may result in undefined behavior.