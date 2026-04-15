> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXTRACTF64x4

Extracts four double-precision floating-point values from the specified index of the source vector and writes them to the destination vector.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm reg | zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 foundation extensions to be enabled.

The index provided via the immediate operand SHALL be within the range of the source vector's elements to avoid undefined behavior; specifically, since it extracts four f64 values, the maximum valid index is 4 (for a 512-bit zmm register containing eight f64 values). Using an out-of-bounds index MAY result in the extraction of zero values or other architecture-specific behavior depending on the implementation.