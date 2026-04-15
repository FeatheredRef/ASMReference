> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD132SD

Performs a fused multiply-add operation on scalar double-precision floating-point values. It computes the result of `f64(a * b + c)` where the operands are provided in the order specified by the 132 permutation (third operand, first operand, second operand), and stores the result in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f64, f64, f64 | f64 |
| r64 (xmm), r64 (xmm), m8 | r64 (xmm) |
| r64 (xmm), r64 (xmm), r64 (xmm) | r64 (xmm) |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or 32-bit mode. It is NOT available in compatibility mode. The instruction requires the FMA3 extension to be supported by the processor.

The operation is performed as a single ternary operation with unbounded precision before rounding, which avoids intermediate rounding errors. The instruction MAY trigger the following floating-point exceptions: #I, #D, #O, #U, and #P. If any operand is a Signaling NaN, #I is signaled. The behavior of the rounding is governed by the rounding control bits in the MXCSR register.