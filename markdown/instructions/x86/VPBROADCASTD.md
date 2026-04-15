> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPBROADCASTD

Broadcasts a dword-size value from a memory location or a YMM register to all 32-bit lanes of the destination YMM or ZMM register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m4 | ymm/zmm |
| ymm | ymm/zmm |

DO NOT support LOCK

This instruction requires AVX or AVX2 support. It is only available in 64-bit mode or compatibility mode when executing 64-bit code.

When the destination is a ZMM register and the source is a YMM register, the dword value is broadcast to all 8 lanes of the ZMM register. If the destination is a YMM register and the source is a ZMM register, the value is broadcast from the lowest 32 bits of the source ZMM register to the 4 lanes of the YMM register. Using a ZMM register as a destination with a YMM source results in the upper 128 bits of the source being ignored.