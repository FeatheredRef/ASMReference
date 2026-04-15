> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSCATTERDD

VPSCATTERDD scatters 32-bit doublewords from a zmm register to memory locations specified by a set of indices in another zmm register. For each element in the index register, the instruction calculates the destination address by adding the scaled index to a base address and stores the corresponding doubleword from the source register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm | m4 |
| zmm | zmm (indices) |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512 single-instruction multiple-data (SIMD) extensions. It requires a processor implementing the AVX-512F instruction set.

The instruction SHALL be used with a mask register (k-register); elements not enabled by the mask are not written to memory. Since the instruction performs multiple memory writes, it is not atomic. If the index register contains duplicate indices, the order of writes is determined by the element index in the registers, and the final value stored at the duplicated address is the one from the highest index element.

To avoid undefined behavior or segmentation faults, the user MUST ensure that the calculated effective addresses are within the allocated memory boundaries and that the index register contains valid `i32` values. Failure to align the base address may result in performance degradation or alignment check exceptions depending on the processor configuration.