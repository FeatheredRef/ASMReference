> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB231SS

Subtracts the product of the second source operand and the third source operand from the first source operand, then stores the result in the destination operand. The operation is performed on the lowest-precision scalar floating-point values.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg, m32, m64 | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or in compatibility mode. It requires the AVX instruction set to be enabled in the processor and the operating system.

The instruction uses the VEX encoding scheme. If the destination register is the same as one of the source registers, the operation is performed destructively. To avoid unintended data loss, the programmer SHALL ensure that the source register to be preserved is not used as the destination. The operation is subject to floating-point exception flags such as #P, #O, #U, #D, and #Z according to the IEEE 754 standard.