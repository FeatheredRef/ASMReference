> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FSCALE

Scales the floating-point value in the destination operand by a power of 2. The scale factor is determined by the value of the source operand, which is treated as a signed integer. The destination value is multiplied by $2^{source}$.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | ST(0) |
| mN | ST(0) |

DO NOT support LOCK

The instruction operates on the FPU register stack; therefore, it is available in both 64-bit mode and compatibility mode. The source operand must be a valid memory location or register containing an i32.

The scale factor is extracted from the source operand as a signed integer. If the source operand is a memory location, it SHALL be accessed according to the specified size. Values of the scale factor that are too large to be represented in the floating-point exponent SHALL result in #O or #U. If the result is not exact, the #P exception is raised. The destination operand is always the top of the FPU stack, ST(0).