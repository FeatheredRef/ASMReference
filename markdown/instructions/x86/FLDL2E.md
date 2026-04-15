> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FLDL2E

Loads the value $2^x$ onto the FPU register stack, where $x$ is the value of the specified operand. The result is pushed onto the stack, and the original operand remains in its location.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | r80 |
| m8 | r80 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates exclusively on the x87 FPU register stack.

The exponent $x$ MUST be within the range $-21024 \le x \le 21024$ to avoid a #O exception. If the input is a NaN, the result is a NaN. The precision of the result depends on the current FPU control word setting; if the result cannot be represented exactly, a #P exception is generated.