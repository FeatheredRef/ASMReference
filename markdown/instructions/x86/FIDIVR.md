> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FIDIVR

Divides the fN value in the ST(0) register by the fN value in a specified register and stores the result in ST(0).

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | reg (ST(0)) |

DO NOT support LOCK

This instruction is available only in x86-64 compatibility mode. It is not supported in 64-bit mode.

The precision and range of the operation depend on the floating-point control word. If the divisor is zero, #Z is generated. If the result is too large to be represented, #O is generated. If the result is too small, #U is generated. If the result cannot be represented exactly, #P is generated. Operations involving denormalized operands may trigger #D.