> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVB2M

VPMOVB2M moves bytes from a SIMD register to a memory location. The instruction takes each byte from the source register and stores it as an individual byte in the destination memory address, effectively expanding or moving a byte-stream.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | m1 |
| reg | imm #I |

DO NOT support LOCK

This instruction requires the AVX-512 extension set to be enabled. It is only available in 64-bit mode and 32-bit mode; it is NOT supported in compatibility mode.

The destination memory address MUST be aligned to the requirements of the data size to avoid potential performance penalties or general protection faults depending on the alignment check attribute. Ensure that the memory region m1 is sufficiently large to hold the number of bytes being moved from the source register to prevent out-of-bounds memory access.