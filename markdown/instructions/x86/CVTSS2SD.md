> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTSS2SD

Converts a scalar single-precision floating-point value to a scalar double-precision floating-point value.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m4 | xmm reg |

DO NOT support LOCK

This instruction is available in 32-bit and 64-bit modes. It requires SSE3 support.

The operation is performed according to the rounding control in the MXCSR register. If the source value cannot be represented in double-precision format, #O is signaled. If the conversion results in a loss of precision, #P is signaled.

To avoid unexpected behavior, ensure that the destination xmm register is not relied upon for its upper 64 bits, as the instruction only modifies the lower qword of the destination register.