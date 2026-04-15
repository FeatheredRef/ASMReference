> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCATTERPF0DPD

This instruction stores packed double-precision floating-point values from a zmm register to memory locations specified by an index vector. It uses the provided base address and a scale factor to calculate the effective addresses for each element.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm reg | m64 |
| zmm reg | zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX-512 support (specifically AVX-512F) to be enabled.

The instruction utilizes a mask register to determine which elements are stored; elements with a mask bit set to 0 are not written to memory. When the mask is not used, all elements are processed. Since this instruction performs multiple memory writes, it is subject to memory aliasing; if multiple indices point to the same memory location, the final value stored is determined by the order of the elements in the vector. To avoid undefined behavior or data corruption in multi-threaded environments, software MUST ensure proper synchronization or use unique indices.