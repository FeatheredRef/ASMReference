> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPEXPANDD

Expands a packed sequence of double-precision floating-point values from a source operand to a destination operand by inserting zeros between the original elements. The number of elements to be expanded is determined by the vector length and the mask register.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm reg | zmm/ymm/xmm reg |
| m64 | zmm/ymm/xmm reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It requires AVX-512 support and specifically the AVX-512 Foundation instructions.

The destination register is updated based on the provided opmask register. Elements in the destination that are not selected by the mask remain unchanged. If the source operand is a memory region (m64), the alignment MUST comply with the requirements of the specific AVX-512 implementation to avoid performance penalties or faults. Using a mask that results in out-of-bounds access relative to the destination register size SHALL result in undefined behavior or a general protection fault.