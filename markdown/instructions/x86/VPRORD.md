> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPRORD

Performs a bitwise rotation to the right of each element in the source operand by the count specified in the second source operand.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode and requires the AVX-512 ER (Extended Registers) extension. It cannot be used in compatibility mode.

The rotation count is masked to the size of the element being rotated; for example, if rotating a dword, only the lower 5 bits of the count register are used. Failure to account for this masking may lead to unexpected results if the count register contains values outside the 0-31 range for dword elements.