> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# UNPCKHPD

Unpacks packed single-precision floating-point values from a source to a destination. The lower 32 bits of the source are moved to the lower 32 bits of the destination, and the upper 32 bits of the source are moved to the upper 32 bits of the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m32 | xmm |
| m64 | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in both 64-bit mode and compatibility mode. It requires SSE support.

The destination register is overwritten by the result; therefore, the original values in the destination xmm register are lost. When using memory operands, the memory must be properly aligned to the size of the accessed data to avoid performance penalties or exceptions, depending on the alignment check flags.