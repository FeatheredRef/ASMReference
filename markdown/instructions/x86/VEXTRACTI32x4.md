> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXTRACTI32x4

VEXTRACTI32x4 extracts four 32-bit doublewords from a 256-bit vector register, based on an immediate index, and stores the result in a 128-bit vector register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm reg | xmm reg |
| imm | #I |
| memory | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX instruction set to be enabled.

The immediate index SHALL be in the range 0 to 3. If the source is a 128-bit register (xmm), the index MUST be 0; otherwise, the behavior is undefined. The destination register is treated as a 128-bit vector, and any bits beyond the 128-bit boundary in the destination register are undefined.