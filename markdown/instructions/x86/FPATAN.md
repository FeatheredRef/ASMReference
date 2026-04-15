> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FPATAN

Computes the arctangent of the value in the ST(0) register using a polynomial approximation and stores the result in ST(0).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (ST(0)) | reg (ST(0)) |

DO NOT support LOCK

This instruction is only available in x86-64 when the processor is operating in compatibility mode. It is not supported in 64-bit mode.

The result of the approximation is less precise than the results of typical transcendental instructions. It MAY produce #I, #Z, #D, #O, #U, or #P exceptions depending on the input value and the current rounding mode. If the input is a NaN, #I is signaled.