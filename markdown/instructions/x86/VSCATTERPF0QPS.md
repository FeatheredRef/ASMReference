> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCATTERPF0QPS

VSCATTERPF0QPS stores packed single-precision floating-point values from a zmm register to memory locations specified by a set of indices in another zmm register. It uses a base address and a scale factor to calculate the effective addresses. The instruction specifically ignores the mask (masking is not applicable in the PF0 variant) and performs the scatter operation.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg (zmm) | m32 |
| reg (zmm) | m32 |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It requires the AVX-512 foundation and the AVX-512PF (Prefetch) extension.

To avoid undefined behavior or memory faults, the indices provided in the index register MUST be within the valid bounds of the allocated memory region. Since VSCATTERPF0QPS is a non-faulting prefetch-style scatter, it is designed to provide hints to the memory system; however, accessing non-canonical addresses MAY still result in a general protection exception (#GP). Multiple indices pointing to the same memory location SHALL result in undefined behavior regarding the final value stored at that location.