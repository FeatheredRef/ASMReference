> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ANDPS

Performs a bitwise AND operation on two packed single-precision floating-point values. The result is stored in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction requires the SSE extension to be supported by the processor. It is available in both 64-bit mode and compatibility mode.

To avoid undefined behavior or exceptions, ensure that the memory operands are aligned to 16 bytes; otherwise, a general-protection exception (#GP) may occur depending on the alignment check flag and the specific processor implementation. This instruction does not affect the EFLAGS register.