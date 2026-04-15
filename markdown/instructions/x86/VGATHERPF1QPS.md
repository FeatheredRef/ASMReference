> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERPF1QPS

Gathers packed single-precision floating-point values from memory using a base address, a vector of indices, and a scale factor. The instruction fetches elements from memory locations calculated as `base + (index * scale)`, where the index is retrieved from a specified ZMM register. It uses a mask register to track which elements have been successfully loaded; elements already processed or masked out are skipped.

The following table describes the supported sources and destinations:

| source | destination(s) |
| :--- | :--- |
| reg, m64 | zmmN |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX-512 support (specifically AVX-512F).

The instruction modifies the mask register used for the gather operation. If a memory access faults, the instruction saves the current state of the mask register and the index, allowing the operation to resume after the exception handler returns. Failure to properly manage the mask register before the call may result in redundant memory accesses or incorrect data loading.

The `PF` (Prefetch) variant of this instruction is designed to hint to the processor that the data should be prefetched into the cache. However, the actual loading of data into the destination register still occurs. Users MUST ensure that the memory addresses calculated do not cross page boundaries in a way that triggers unexpected page faults if the mask is not correctly initialized.