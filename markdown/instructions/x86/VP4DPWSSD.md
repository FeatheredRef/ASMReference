> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VP4DPWSSD

VP4DPWSSD packs four signed 32-bit integers from a source into a destination, performing a signed saturation to 16-bit integers. The instruction takes four signed 32-bit values, saturates each to the range of a signed 16-bit integer ([-32768, 32767]), and stores the resulting four signed 16-bit values into the destination.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m128/m256/m512 | xmm/ymm/zmm reg |
| #I | #I |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or 32-bit mode. It is NOT supported in compatibility mode. The instruction requires the AVX-512 BW (Bypass/Word) extension to be supported by the processor.

To avoid unexpected behavior, ensure that the destination register is not used as a source unless the desired behavior is to overwrite the original data. When using memory operands, the memory address SHALL be aligned to the size of the vector register to avoid performance penalties or faults, depending on the alignment check settings.