> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD231PD

Performs a fused multiply-add operation on double-precision floating-point values. It computes the product of the second and third operands and adds the result to the first operand. The operation is performed as a single step with one rounding at the end.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| reg, reg, m64 | reg |
| reg, m64, reg | reg |
| m64, reg, reg | reg |

DO NOT support LOCK

The instruction SHALL only be executed in 64-bit mode or compatibility mode. It REQUIRES the AVX extension to be enabled in the processor.

To avoid performance degradation due to AVX-SSE transitions, the programmer SHOULD ensure that VEX-encoded instructions are not interleaved with legacy SSE instructions without appropriate alignment or state management. Failure to do so MAY cause the processor to insert transition penalties.

The instruction MAY trigger the following floating-point exceptions:
- #I: Invalid operation.
- #O: Numeric overflow.
- #U: Numeric underflow.
- #P: Inexact result (precision).