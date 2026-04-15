> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSLLVW

Shifts the 32-bit doubleword elements of the first source operand to the left by the number of positions specified by the second source operand. The shift amount is taken from the corresponding element of the second source operand (vector shift count), and the result is stored in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm register | xmm/ymm/zmm register |
| xmm/ymm/zmm register | xmm/ymm/zmm register |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX-512F instruction set extension.

The shift count is masked to 5 bits (u5), meaning only values from 0 to 31 are used. Any bits beyond the 5th bit in the shift count operand are ignored. If the destination register is the same as the first source register, the instruction performs an in-place operation. Because this is a variable shift, the operation is performed element-wise; a failure to ensure the shift count register is properly initialized may result in unpredictable shift distances.