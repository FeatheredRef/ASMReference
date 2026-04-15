> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ADDSD

Adds two scalar-double-precision floating-point values. The instruction adds the source operand to the destination operand and stores the result in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm8 (f64) | xmm8 (f64) |
| m8 (f64) | xmm8 (f64) |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE3 extension to be supported by the processor.

The operation is performed according to the IEEE 754 standard. The instruction ignores the precision control field of the MXCSR register.

The result may trigger floating-point exceptions: #D if a denormal operand is encountered, #O if the result overflows, #U if the result underflows, and #P if the result is inexact. If the result is NaN, the behavior follows IEEE 754 propagation rules. Using a memory operand for the destination SHALL result in an invalid operation as ADDSD requires the destination to be an xmm register.