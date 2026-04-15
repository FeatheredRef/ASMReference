> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FSINCOS

Computes both the sine and cosine of the angle (in radians) currently stored in the ST(0) floating-point register. The sine result is stored in ST(0) and the cosine result is stored in ST(1).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| ST(0) | ST(0), ST(1) |

DO NOT support LOCK

This instruction is part of the x87 floating-point instruction set. In x86-64 architecture, it is available in both 64-bit mode and compatibility mode. It requires the FPU state to be properly initialized.

The input angle must be within the range of the FPU's internal reduction capabilities to avoid significant precision loss. If the input is too large, the result may be inaccurate. The instruction may trigger #O if the result cannot be represented. It may also trigger #P if the result requires rounding.