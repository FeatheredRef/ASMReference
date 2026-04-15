> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVSQW

Moves with sign-extension quadwords to words. Each 64-bit signed integer element from the source is sign-extended and truncated to a 16-bit signed integer element in the destination.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m16/m32/m64 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512BW instruction set extension to be supported by the processor.

To avoid undefined behavior or general protection faults, the destination register MUST be the same size as the source register if using the EVEX prefix to ensure consistent element processing. When using memory operands, the memory region MUST be aligned to the element size to prevent performance degradation or alignment checks.