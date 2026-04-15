> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMT2Q

VPERMT2Q permutes quadwords from two source registers based on an index provided in a third register. For each 64-bit element in the destination, the instruction uses the corresponding 2-bit index from the index register to select a quadword from either the first or second source register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 foundation and the AVX-512BW instruction set.

The index register is treated as a series of 2-bit indices; any bits in the index register beyond those required to address the available quadwords in the sources are ignored. Users MUST ensure the index register is correctly initialized to avoid unexpected element selection, although it does not trigger a processor exception for out-of-range indices.