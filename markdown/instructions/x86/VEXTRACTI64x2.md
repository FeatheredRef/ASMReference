> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXTRACTI64x2

VEXTRACTI64x2 extracts two 64-bit elements from a YMM register based on an immediate index and stores the result in a XMM register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| ymm | xmm |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction requires AVX support and is available in both 64-bit mode and compatibility mode. It SHALL NOT be used in 32-bit mode unless the processor supports the corresponding AVX extensions.

The immediate index SHALL be within the range of 0 to 3. Any value outside this range will result in an invalid operation. Because this instruction transitions data from a 256-bit register to a 128-bit register, it may trigger transitions between AVX and SSE states, potentially causing performance penalties due to VZEROUPPER requirements if mixed with legacy SSE code.