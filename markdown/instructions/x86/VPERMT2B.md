> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMT2B

Permutes two 512-bit source registers based on an index provided in a third register. For each byte in the destination, the instruction selects a byte from either the first or second source register based on the corresponding index byte. If the high bit of the index byte is 0, the byte is selected from the first source; if the high bit is 1, it is selected from the second source. The remaining 7 bits of the index determine the byte offset within the selected 512-bit source.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It REQUIRES the AVX-512 foundation and the AVX-512 BW (Bytes and Words) extension to be supported by the processor.

The destination register MUST NOT be one of the source registers if the implementation does not support destructive operands. Failure to comply with register constraints may lead to undefined behavior in specific microarchitectures. Ensure that the index register contains values within the range 0-127 for each byte to avoid logic errors, although the instruction will technically execute based on the 7-bit mask.