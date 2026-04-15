> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FSIN

Computes the sine of the ST(0) value. The result is stored in ST(0).

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (ST(0)) | reg (ST(0)) |

DO NOT support LOCK

This instruction is executed in the x87 FPU environment. In x86-64 mode, it is available in both 64-bit mode and compatibility mode, as the x87 FPU state is preserved independently of the general-purpose register width.

The input value in ST(0) MUST be in radians. If the input is too large to be reduced modulo $2\pi$, the instruction SHALL signal the `#O` exception. Precision control is governed by the current FPU control word; if the result cannot be represented exactly, the `#P` exception SHALL be signaled. Any NaN input SHALL result in the `#I` exception.