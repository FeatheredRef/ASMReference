> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMOVDQA64

Moves 64 bytes of data from a source to a destination. The move is performed as an aligned operation.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| zmm | zmm |
| m64 | zmm |
| zmm | m64 |

DO NOT support LOCK

The instruction SHALL be used only in 64-bit mode. It requires the AVX-512 foundation (AVX-512F) to be supported and enabled.

The memory operand SHALL be aligned on a 64-byte boundary. If the memory address is not aligned, a general-protection exception (#GP) SHALL be generated.