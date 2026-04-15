> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FDIV

Divides the destination floating-point value by the source floating-point value.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |

DO NOT support LOCK

FDIV is available in 64-bit mode, compatibility mode, and 32-bit mode. It operates exclusively on the x87 FPU register stack.

The destination operand SHALL be the top of the stack (ST(0)) unless the instruction specifies a different register (ST(i)). If the division results in a value that cannot be represented in the destination format, the #O exception MAY be triggered. Division by zero SHALL trigger the #Z exception. Precision loss during rounding SHALL trigger the #P exception. If an operand is denormal, the #D exception SHALL be triggered.