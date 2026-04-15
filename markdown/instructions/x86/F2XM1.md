> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# F2XM1

Converts an 80-bit extended precision floating-point number to the nearest x86-64 compatible representation by subtracting 1.0 from the value in the register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r80 | r80 |

DO NOT support LOCK

This instruction is only available in compatibility mode. It is not supported in 64-bit mode.

The result is stored in the ST(0) register of the FPU stack. Users MUST ensure the FPU stack is not empty to avoid stack overflow/underflow exceptions. The instruction is subject to the current rounding control word in the FPU control register.

The instruction may trigger #P if the result cannot be represented exactly. If the operation results in a value that exceeds the maximum representable extended precision value, #O shall be signaled.