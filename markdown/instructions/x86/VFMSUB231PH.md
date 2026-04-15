> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB231PH

Subtracts the product of the second and third operands from the first operand, using packed half-precision floating-point values. The operation is performed as: $dest = (a \times c) - b$.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| xmm/ymm/zmm | xmm/ymm/zmm |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is ONLY available if AVX-512 FP16 is supported. It requires the processor to be in 64-bit mode.

To avoid precision loss or incorrect results, ensure that the destination register is not used as an input operand unless the operation is intended to be destructive. The instruction uses the half-precision (16-bit) format; if the input data is not properly aligned to the vector length, a general protection fault may occur.

The instruction may trigger the following floating-point exceptions:
- #P: If the result is inexact.
- #O: If the result exceeds the maximum representable value of f16.
- #U: If the result is smaller than the minimum representable value of f16.
- #D: If any operand is a denormalized number.