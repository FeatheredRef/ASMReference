> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSUBD

Subtracts two packed signed dwords.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m4 | xmm |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit modes. It requires SSE2 support.

The destination register MUST be an XMM register. Memory-to-memory operations are NOT supported. Using a memory operand as a source requires the processor to support the corresponding alignment or handle the resulting penalty.