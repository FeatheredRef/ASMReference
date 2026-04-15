> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVAPD

Moves aligned packed single-precision floating-point values from a source to a destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |
| xmm reg | m128 |

DO NOT support LOCK

The instruction REQUIRES that both the source and destination memory operands be aligned on a 16-byte boundary. If the memory operand is not aligned, a general-protection exception (#GP) SHALL be generated.

To avoid #GP exceptions, the programmer MUST ensure that memory addresses used with MOVAPD are 16-byte aligned. If alignment cannot be guaranteed, MOVUPS SHALL be used instead.