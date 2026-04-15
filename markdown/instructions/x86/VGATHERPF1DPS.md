> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERPF1DPS

Gathers 32-bit floating-point values from memory using a base address, a vector of indices, and a scale. For each single-precision floating-point element in the destination register, the instruction calculates the effective address by adding the base address to the product of the index and the scale. Only elements with a corresponding mask bit set to 1 are fetched; once a value is successfully loaded, the corresponding mask bit is cleared to 0.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m32 | zmm/ymm/xmm |
| reg | zmm/ymm/xmm |
| reg | zmm/ymm/xmm |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode. It requires AVX2 support and is NOT available in compatibility mode.

The instruction depends on the mask register (k-register or opmask) to determine which elements are processed. If a page fault occurs during the gather operation, the mask register SHALL be updated to reflect the elements that were successfully loaded before the fault, allowing the handler to resume the operation without re-fetching data. The scale value SHALL be one of 1, 2, 4, or 8. If the calculated address is not aligned to the element size, the behavior is subject to the alignment check settings of the processor.