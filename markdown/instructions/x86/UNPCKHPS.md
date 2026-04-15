> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# UNPCKHPS

Unpacks the high quadword from two XMM operands into a single XMM destination register. The instruction selects the upper two f32 elements from each source operand and concatenates them into the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 32-bit mode and 64-bit mode. It requires the SSE3 instruction set extension.

The destination register must not be the same as the memory source operand to avoid undefined behavior resulting from read-after-write hazards. All operations are performed on f32 values; if the source operands contain non-canonical floating-point representations, the instruction will process them as-is without triggering #S exceptions.