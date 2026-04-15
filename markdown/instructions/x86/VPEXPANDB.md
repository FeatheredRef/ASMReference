> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPEXPANDB

Expands unsigned bytes from a source vector to a destination vector of unsigned words. The instruction takes a mask register and uses it to select which bytes from the source are expanded and placed into the destination; elements not selected by the mask are either left unchanged or zeroed based on the masking write-back policy.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm | zmm/ymm |
| m1 | zmm/ymm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 EBMI (Experimental Basic Multimedia Instructions) extension.

To avoid undefined behavior, ensure that the mask register `k` is correctly initialized; if a mask bit is 0 and the masking policy is set to merging, the original value in the destination register is preserved. When using memory operands, ensure the memory region is aligned to the vector length to avoid performance penalties or faults.