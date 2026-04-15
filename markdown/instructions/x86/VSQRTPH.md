> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSQRTPH

Computes the square root of the lower 16 bits of the source operand as a half-precision floating-point number and stores the result in the destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m16 | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX-512 FP16 extension.

The instruction supports rounding control via the `round-control` field in the EVEX prefix; if the field is set to `00`, the rounding is governed by the MXCSR register. If the input is a NaN, the result is a qNaN. If the input is negative, the instruction generates #I and returns a qNaN. If the input is zero, the result is zero with the sign of the input. Precision #P is raised if the result is not exact.