> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMB

Permutes bytes within a 256-bit vector register. The instruction selects bytes from the first source operand based on indices specified in the second source operand. For each byte in the destination, the index from the second operand determines which byte from the first operand is selected.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm | zmm |
| m32 | zmm |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It requires AVX-512 support (specifically AVX512BW). The instruction is subject to masking if a mask register is specified; if not, all elements are updated.

The index for the byte selection is taken as the low 8 bits of the corresponding byte in the index register. If the index value is greater than or equal to 32, the resulting byte in the destination is set to zero. To avoid unexpected zeroing of data, ensure that the index register contains values in the range [0, 31].