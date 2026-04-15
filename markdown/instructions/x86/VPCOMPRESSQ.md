> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPCOMPRESSQ

Compresses 64-bit quadwords from a source vector to a destination vector based on a mask. For each bit set in the mask, the corresponding quadword from the source is copied to the next available slot in the destination vector, effectively removing elements where the mask bit is zero.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm | zmm/ymm/xmm |
| m64 | zmm/ymm/xmm |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It is NOT supported in compatibility mode.

The instruction requires AVX-512 foundation support. If the destination register is also used as a source (destructive operand), the elements of the destination are overwritten sequentially; however, since the compression process moves from the lowest index to the highest, the original values are read before they are potentially overwritten.

When using the instruction with a memory operand for the source, the alignment of the memory access SHALL follow the standard AVX-512 alignment rules to avoid performance penalties or faults in strict alignment environments. The mask register MUST be specified; failure to provide a valid mask register will result in an invalid opcode exception.