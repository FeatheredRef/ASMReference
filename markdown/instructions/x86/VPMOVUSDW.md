> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVUSDW

Moves unsigned 16-bit integers from a source to a destination, converting them to signed 32-bit integers by zero-extending the 16-bit values into 32-bit elements.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m16 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX extensions to be enabled.

The destination register must be larger than or equal to the source register size to avoid undefined behavior during vector length transitions. When using YMM or ZMM registers, the instruction may trigger VEX-encoded transition penalties if mixed with legacy SSE instructions without intervening VZEROUPPER or VZEROALL instructions. Memory operands MUST be aligned to the element size to avoid performance degradation or faults in specific memory models.