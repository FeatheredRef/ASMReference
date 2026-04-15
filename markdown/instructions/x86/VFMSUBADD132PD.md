> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUBADD132PD

Performs a fused multiply-subtract and add operation on double-precision floating-point values. The instruction computes `(a * b) - c + d` for each pair of double-precision floating-point values in the input vectors.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m128 |
| m128 | reg |
| m128 | m128 |

DO NOT support LOCK

This instruction is ONLY available when the processor is operating in 64-bit mode or compatibility mode. It REQUIRES the AVX-512 foundation and the AVX-512DQ instruction set extensions to be enabled.

The operation is performed using a single rounding step at the end of the entire fused calculation, which prevents intermediate rounding errors that would occur if multiply, subtract, and add were performed as discrete instructions. The result is subject to the current floating-point control word rounding mode.

Possible exceptions include:
- #O: Numeric overflow.
- #U: Numeric underflow.
- #P: Inexact result.
- #D: Denormalized operand.
- #Z: Divide-by-zero (not applicable to this operation).