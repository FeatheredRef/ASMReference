> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCALEFPD

Scales a double-precision floating-point value by a specified scale factor. The instruction multiplies the source value by a power of two determined by the immediate operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m64 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is available only when AVX-512 is supported. It operates on zmm, ymm, or xmm registers depending on the vector length (EVEX prefix).

To avoid unexpected results, ensure the immediate value is within the valid range for scaling; otherwise, the operation may result in #O or #U. The instruction follows the standard IEEE 754 rounding mode specified in the MXCSR register. If the scale factor results in a value that cannot be represented, the standard floating-point exception flags (#O, #U, #P) SHALL be set.