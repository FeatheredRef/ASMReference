> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMINSB

This instruction computes the minimum of the signed bytes in the source and the destination, storing the result in the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m128 | r128 |
| r128 | r128 |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE4.1 instruction set extension.

The instruction operates on 16 packed signed bytes. The source and destination MUST be 128-bit XMM registers or memory locations. If a memory operand is used, it MUST be aligned to 16 bytes to avoid performance penalties or faults depending on the alignment check flags.