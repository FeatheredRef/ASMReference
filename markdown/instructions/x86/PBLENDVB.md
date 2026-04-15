> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PBLENDVB

Blends bytes from two YMM or ZMM registers based on a mask provided in a third register. For each byte in the source registers, if the corresponding mask bit is set, the byte from the second source register is selected; otherwise, the byte from the first source register is selected.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| #I | mN |
| #I | imm |

DO NOT support LOCK

This instruction is available only in 64-bit mode and 32-bit mode. It requires the AVX extension for YMM registers and AVX-512 for ZMM registers.

To avoid unexpected behavior, ensure that the mask register is properly initialized; bits in the mask register that do not correspond to the vector length of the operands are ignored. When using AVX-512, the behavior depends on the specific foundation or extension used to define the register width (e.g., EVEX prefixing).