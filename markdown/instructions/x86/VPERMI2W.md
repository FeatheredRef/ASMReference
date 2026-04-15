> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMI2W

Permutes two 256-bit source operands based on the indices provided in a third source operand. Each element of the destination is selected from either the first or second source operand (word-sized elements) as determined by the most significant bit of the corresponding index in the index register.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | reg |
| reg | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the AVX-512 BW foundation.

The index register is treated as a vector of 8-bit unsigned integers (u8). Since the elements being permuted are words (16-bit), the indices must be within the range of 0-15. If an index is greater than 15, the result for that specific element is undefined. Users MUST ensure that the index values do not exceed the number of available word elements in the source registers to avoid unpredictable behavior.