> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADDSUB132PS

Computes the product of two floating-point values and adds or subtracts the result to/from a third floating-point value. The operation is performed as $f32 = (f1 \times f2) + f3$ or $f32 = (f1 \times f2) - f3$ based on the instruction variant and the sign of the operands. It operates on packed single-precision floating-point values.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m128 |
| m128 | reg |
| m128 | m128 |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or 32-bit mode. It is NOT available in compatibility mode. It REQUIRES the AVX and FMA support in the processor.

To avoid precision loss or unexpected #P (Inexact result) exceptions, ensure that the floating-point control word is configured correctly. The instruction uses a single rounding step at the end of the fused operation, which prevents intermediate rounding errors typically found in separate multiplication and addition instructions. Use of this instruction on unsupported hardware SHALL result in an invalid opcode exception.