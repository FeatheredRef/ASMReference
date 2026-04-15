> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTSH2USI

Converts a packed signed 16-bit integer vector to a packed unsigned 32-bit integer vector with truncation. The instruction converts each signed 16-bit integer element in the source to a 32-bit unsigned integer, saturating the result to the unsigned 32-bit integer range.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| m16 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires AVX-512 FDL (Floating Point and Low Precision) or AVX-512 BW (Byte and Word) support depending on the register width used.

When using masked versions of this instruction, the destination register elements that are masked are not updated, preserving their original values. Failure to ensure the destination register is properly initialized when using masking may lead to unexpected data persistence.