> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPBLENDMD

Selects elements from two source operands based on a mask specified by an immediate. For each 64-bit element, if the corresponding bit in the immediate is 1, the element from the second source is selected; if the bit is 0, the element from the first source is selected.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, m64 | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the AVX instruction set.

The immediate operand MUST be a byte. If the vector length (determined by the register size) exceeds the number of 64-bit elements that can be indexed by the immediate, the remaining elements of the destination register shall remain unchanged. Ensure that the destination register is not used as a source if the immediate does not cover all elements and previous data must be preserved.