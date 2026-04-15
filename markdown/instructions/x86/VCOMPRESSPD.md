> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCOMPRESSPD

Compresses packed double-precision floating-point values from a source register into a destination register based on a mask. For each bit set in the mask, the corresponding element from the source is copied to the next available slot in the destination; elements corresponding to zero bits in the mask are skipped.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires AVX-512 support (specifically AVX-512F).

The destination register is updated based on the number of set bits in the mask. Because the instruction writes to the destination register starting from the first element, any existing data in the destination register beyond the number of elements compressed is preserved. The mask register must be a k-register; using an invalid mask register will result in an invalid operand exception. To avoid data corruption or unexpected results, the programmer SHALL ensure the mask contains only bits corresponding to valid elements within the vector length of the registers being used.