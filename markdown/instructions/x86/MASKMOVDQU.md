> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MASKMOVDQU

Moves data from a source to a destination based on a mask. For each bit set in the mask register, the corresponding byte is moved from the source to the destination; bytes corresponding to unset bits in the mask are not modified in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m16 |
| reg | m32 |
| reg | m64 |
| m16 | reg |
| m32 | reg |
| m64 | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It REQUIRES the AVX-512 foundation extensions to be supported by the processor.

The destination memory operand MUST be aligned to the size of the data being moved to avoid performance penalties, although the instruction is designed to handle unaligned accesses. Failure to ensure correct mask register initialization MAY result in unintended data being preserved in the destination.