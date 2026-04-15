> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB132PS

Performs a fused multiply-subtract operation on packed single-precision floating-point values. The instruction computes the result as `(a * b) - c`, where `a` is the first operand, `b` is the second operand, and `c` is the third operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m128 |
| m128 | reg |
| m128 | m128 |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires support for the AVX and FMA3 instruction sets.

The instruction uses a three-operand syntax where the destination register can also serve as one of the source operands (specifically the addend `c`). To avoid unintended data loss, the destination register SHALL NOT be used as a source if its original value is required for subsequent operations. All operations are performed according to the IEEE 754 standard for floating-point arithmetic.

The operation may generate floating-point exceptions including #D, #O, #U, and #P depending on the input values and the current floating-point control word. If the result is an inexact value, the #P exception is signaled.