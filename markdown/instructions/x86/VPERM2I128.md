> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERM2I128

This instruction permutes 32-bit elements from two 128-bit source operands based on an index provided in a third operand. The elements are selected from the combined 256-bit pool of the two sources and placed into the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| xmm reg | xmm reg |
| xmm reg | xmm reg |

DO NOT support LOCK

This instruction SHALL only be executed when the AVX-512 VBMI (Vector Byte Manipulation Instructions) subset is supported by the processor. It is NOT supported in compatibility mode if the CPU requires 64-bit mode for AVX-512 extensions.

To avoid undefined behavior, the index provided in the control register MUST be within the range of 0 to 7 for each 32-bit lane of the destination. Indices outside this range SHALL result in the corresponding element in the destination being set to zero.