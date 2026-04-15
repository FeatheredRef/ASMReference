> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SUBSS

Subtracts scalar single-precision floating-point values. The instruction subtracts the lower 32 bits of the source operand from the lower 32 bits of the destination operand. The upper bits of the destination XMM register are undisturbed.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m32 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE support.

The operation is subject to the rounding control and exception masking defined in the MXCSR register. If the operation results in a value that cannot be represented as a single-precision floating-point number, it SHALL trigger the corresponding exception (#O, #U, or #P). Denormalized operands (#D) are handled according to the Flush-to-Zero (FTZ) and Denormals-Are-Zero (DAZ) flags in the MXCSR register.