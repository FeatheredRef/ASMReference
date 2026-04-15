> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD213PS

Multiplies packed single-precision floating-point values in the first source operand by packed single-precision floating-point values in the third source operand, then adds the result to the packed single-precision floating-point values in the second source operand and stores the result in the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| reg, m32, reg | reg |
| reg, reg, m32 | reg |

DO NOT support LOCK

This instruction SHALL only be executed if the processor supports the FMA3 feature set. It is available in 64-bit mode and compatibility mode.

The instruction is subject to the following floating-point exceptions: #I, #D, #O, #U, and #P. The behavior regarding rounding and precision SHALL be governed by the MXCSR register settings. If the destination register is also used as a source operand, the operation is performed as a destructive source update. To avoid precision loss or unexpected results, ensure that the destination register is correctly initialized or that the intended source values are preserved in separate registers.