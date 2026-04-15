> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PADDSB

Adds unsigned 8-bit integers from a source to a destination in a SIMD manner, with saturation. If the result of an addition exceeds 255, it is clamped to 255.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the SSE2 instruction set extension.

The destination register is overwritten by the result. Because this is a saturated addition, unsigned overflow does not trigger the CPU flags (such as CF or OF). Users MUST ensure that the data is aligned to 16-byte boundaries when using aligned load/store instructions surrounding this operation to avoid performance degradation or general protection faults.