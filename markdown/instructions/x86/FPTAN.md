> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FPTAN

Computes the hyperbolic tangent of the extended precision floating-point value in the ST(0) register and stores the result in ST(0).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (ST(0)) | reg (ST(0)) |

DO NOT support LOCK

This instruction is available in 64-bit mode only in compatibility mode. In 64-bit mode, it is not supported.

The instruction operates exclusively on the 80-bit extended precision floating-point format. If the input value is a NaN or infinity, the result is NaN and #I is signaled. If the result is too large to be represented, #O is signaled. Precision (#P) and underflow (#U) exceptions may be signaled depending on the rounding mode and the result.