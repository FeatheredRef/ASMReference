> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPBLENDD

Selects elements from two source operands based on a mask provided by an immediate value and blends them into a destination operand. For each 32-bit element, if the corresponding bit in the mask is 1, the element from the second source operand is selected; if the bit is 0, the element from the first source operand is selected.

The following table covers what the source and destinations can be:

| Source | Destination |
| :--- | :--- |
| reg | reg |
| m8 | reg |

DO NOT support LOCK

This instruction requires the AVX CPUID leaf to be supported and enabled. It operates on YMM registers. If the instruction is executed in 32-bit mode (compatibility mode), it is still available provided the AVX feature set is enabled.

To avoid undefined behavior or application crashes, ensure that the destination register is not the same as the first source operand if the implementation requires non-destructive source usage, although VPBLENDD is generally designed to support destructive destinations. When using memory operands, the memory region m8 MUST be naturally aligned to the element size to avoid performance penalties or alignment exceptions on certain configurations.