> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCATTERPF1DPS

Stores a single-precision floating-point value from a zmm register to a memory location based on an index from another zmm register, applying a mask.

The following table specifies the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | m32 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX-512 support (specifically AVX-512PF).

The index register must contain signed 32-bit integers. If the mask bit is 0, the store operation for that specific element SHALL NOT be performed. Memory accesses MUST be aligned to the requirements of the data type to avoid performance degradation, although the instruction supports unaligned accesses. Multiple indices pointing to the same memory address result in undefined behavior regarding the final value stored.