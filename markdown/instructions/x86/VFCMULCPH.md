> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFCMULCPH

Multiplies two packed half-precision floating-point numbers and rounds the result to the nearest half-precision floating-point number using the rounding mode specified in the immediate operand.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm / ymm / zmm reg | xmm / ymm / zmm reg |
| m128 / m256 / m512 | xmm / ymm / zmm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the AVX-512 FP16 extension.

The rounding mode is controlled by the 3-bit immediate operand; if the immediate is not provided, the rounding is governed by the MXCSR register. The result is subject to the following floating-point exceptions: #I, #D, #O, #U, and #P.

Failure to ensure the processor supports the AVX-512 FP16 feature flag before execution WILL result in an `#UD` (Undefined Instruction) exception. The alignment of memory operands MUST be consistent with the vector size to avoid general protection faults or performance penalties.